use std::ops::{Shl, Shr};

fn to_binary(mut decimal: u32) -> Vec<u32> { // modified from https://codereview.stackexchange.com/questions/210967/decimal-to-binary-in-rust
    let mut bits = Vec::new();
    if decimal == 0 {
        bits.push(0);
    } else {


        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push(0);
            } else {
                bits.push(1);
            }

            decimal /= 2;
        }
   
    }
    return bits
}


pub struct RiceEncoderOptions {
    num_samples: u64,
    predictor_order: u8,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RiceEncodedStream {
    pub stream: Vec <u8>,
    pub param: u8,
    pub extra_bits_len: u8,
}

impl RiceEncoderOptions {
    /// Create a builder to the Rice encoder
    pub fn new(num_samples: u64, predictor_order: u8) -> Self {
        RiceEncoderOptions {
            num_samples,
            predictor_order
        }
    }

    /// Get the minimum partition order
    /// 
    /// The default minimum partition order is zero
    fn min_rice_partition_order() -> u8 {
        0
    }

    /// Get the maximum partition order
    /// 
    /// The maximum partition order is determined by the position of the least significant 1 bit in the block size
    fn max_rice_partition_order(mut block_size: u64) -> u8 {
        let mut order = 0;
        while block_size > 1 {
            block_size = block_size.shr(1);
            order += 1;
        }
        order
    }

    // Compute the best partition order and best Rice parameters for each partition
    // The best partition order is the one that minimizes the total number of bits in the Rice encoding
    // There will be 2^order partitions
    fn best_partition_and_params(&self, residuals: &Vec <i64>) -> (Vec <u8>, u8) {
        let mut best_order = 0;
        let mut best_params = vec![];
        let mut min_bits = u64::MAX;
        
        // Iterate over all possible partition orders to find the best one
        for order in RiceEncoderOptions::min_rice_partition_order()..=RiceEncoderOptions::max_rice_partition_order(self.num_samples) {
            if let Some((params, bits)) = self.best_parameters(order, residuals) {
                if bits < min_bits {
                    min_bits = bits;
                    best_order = order;
                    best_params = params;
                }
            }
        }
        (best_params, best_order)
    }

    /// Compute the best Rice parameters for a given partition order
    fn best_parameters(&self, partition_order: u8, residuals: &Vec <i64>) -> Option <(Vec <u8>, u64)> {
        let n_partitions = 1.shl(partition_order);
        let mut params = vec![0; n_partitions as usize];
        let mut total_bits = 0;

        // Calculate the best Rice parameter for each partition
        for i in 0..n_partitions {
            let start = if partition_order == 0 {
                self.predictor_order as u64
            } else {
                (i as u64 * self.num_samples).shr(partition_order)
            };
            let end = ((i as u64 + 1) * self.num_samples).shr(partition_order);
            let abs_residual_sum = RiceEncoderOptions::sum_abs_residuals(&residuals[start as usize..end as usize]);
            let mut best_param = 0;
            let mut best_bits = u64::MAX;

            // Try all possible Rice parameters and find the one with the fewest bits
            for param in 0..15 {
                let bits = RiceEncoderOptions::bits_in_partition_sums(param, end - start, abs_residual_sum);
                if bits < best_bits {
                    best_bits = bits;
                    best_param = param;
                }
            }

            params[i as usize] = best_param;
            total_bits += best_bits;
        }

        Some((params, total_bits))
    }

    fn sum_abs_residuals(residuals: &[i64]) -> u64 {
        let mut sum = 0;
        for &residual in residuals {
            sum += residual.abs() as u64;
        }
        sum
    }

    /// Find the total number of bits occupied by this encoding
    /// 
    /// Rice encoding uses `q + 1` bits for the unary-encoded quotient `q` and
    /// `rice_param` bits for the binary remainder
    fn bits_in_partition_sums(rice_param: u8, n_partition_samples: u64, abs_residual_sum: u64) -> u64 {
        let mut bits = 0;
        let divisor = 1.shl(rice_param) as u64;

        // Calculate bits for each residual value
        for _ in 0..n_partition_samples {
            let quotient = abs_residual_sum / divisor;
            bits += quotient + 1 + rice_param as u64;
        }

        bits
    }

    /// Encode residuals into Rice encoding
    /// 
    /// This function computes the Rice encoding of each residual and returns the
    /// byte-aligned encodings and number of unused bits in the last element, respectively.
    /// Rice encoding is variable-length, so there is a chance that the stream is not
    /// byte-aligned.


    pub fn encode(rice_param: u32, residuals: &Vec <i64>) -> RiceEncodedStream {
        let mut encoded_bits = Vec::new();
        let m = rice_param as u32;
        let m_float = m as f32;
        let k = m_float.log2() as u32;
        let mut initial_num_bits = 0;
        let mut final_num_bits = 0;

        for residual in residuals {
            // s: u32, m: u32, k: u32
            let s = *residual as u32;
            
            // Unary part: U = S >> K
            let u = s >> k;
            // Unary encoding: represent U in unary with '0's followed by a '1'
            for _ in 0..u {
                encoded_bits.push(1 as u8);
            }
            encoded_bits.push(0 as u8); 

            // Truncated Binary part: B = S & (M - 1)
            let b = s & (m - 1);
            // Binary encoding: represent B in binary, padded to the left with zeros until it is of length K
            for i in (0..k).rev() { // MSB -> LSB
                encoded_bits.push(((b >> i) & 1) as u8);
            }
            initial_num_bits += to_binary(s).len();
            final_num_bits = encoded_bits.len();
        }

        let extra_bits_len = final_num_bits - initial_num_bits;
        RiceEncodedStream {
            stream: encoded_bits,
            param: m as u8,
            extra_bits_len: extra_bits_len as u8,
        }
        //encoded_bits
    }


    /// Encode residuals into Rice encoding
    /// 
    /// This function computes the Rice encoding of each residual by first partitioning
    /// the residual into groups. Each group is then found its best Rice parameter and
    /// then encoded using the parameter. The Rice encoding of each group is then returned.
    pub fn encode_by_partition(&self, residuals: &Vec <i64>)  -> (Vec <RiceEncodedStream>, u8) {
        let (best_params, best_order) = self.best_partition_and_params(residuals);
        let n_partitions = 1.shl(best_order);
        let mut streams = vec![];

        // Encode each partition
        for i in 0..n_partitions {
            let start = (i as u64 * self.num_samples).shr(best_order) as usize;
            let end = ((i as u64 + 1) * self.num_samples).shr(best_order) as usize;
            let partition_residuals = residuals[start..end].to_vec();
            streams.push(RiceEncoderOptions::encode(best_params[i as usize].into(), &partition_residuals));
        }

        (streams, best_order)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let m = 16;
        let input_residuals = vec![18];
        let expected_output = RiceEncodedStream { stream: vec![1, 0, 0, 0, 1, 0], param: m as u8, extra_bits_len: 1}; // from CoE 164 MidP Annex
        
        let rice_encoded_stream = RiceEncoderOptions::encode(m, &input_residuals);

        assert_eq!(rice_encoded_stream, expected_output);
    }
    
    #[test]
    fn test_best_partition() {
        let input_residuals = vec![10];
        let input_num_samples = 1;
        let input_predictor_order = 0;
        let test_rice_encoder_options = RiceEncoderOptions::new(input_num_samples, input_predictor_order);
        let expected_order = 0;
        
        let best_order = test_rice_encoder_options.best_partition_and_params(&input_residuals).1;

        assert_eq!(best_order, expected_order); // There will be 2^order partitions, so there will only be one partition
    }

    #[test]
    fn test_best_parameter() {
        let input_residuals = vec![10];
        let input_num_samples = 1;
        let input_predictor_order = 0;
        let test_rice_encoder_options = RiceEncoderOptions::new(input_num_samples, input_predictor_order);
        let expected_output = vec![2];
        
        let best_params = test_rice_encoder_options.best_partition_and_params(&input_residuals).0;

        assert_eq!(best_params, expected_output); // best rice parameter for the single partition
        
    }

    #[test]
    fn test_encode_by_partition() { 
        let input_residuals = vec![10];
        let input_num_samples = 1;
        let input_predictor_order = 0;
        let test_rice_encoder_options = RiceEncoderOptions::new(input_num_samples, input_predictor_order);
        let expected_output = vec![RiceEncodedStream {stream: vec![1, 1, 1, 1, 1, 0, 0], param: 2, extra_bits_len: 3 }];
        // (streams, best_order) 
        let test_stream = test_rice_encoder_options.encode_by_partition(&input_residuals).0;

        assert_eq!(test_stream, expected_output); 
        
    }
    
}

   