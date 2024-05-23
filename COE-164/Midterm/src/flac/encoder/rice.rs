use std::ops::{Shl, Shr};

use crate::flac::bitstream;
pub struct RiceEncoderOptions {
    num_samples: u64,
    predictor_order: u8,
}

#[derive(Debug)]
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
    pub fn encode(rice_param: u8, residuals: &Vec <i64>) -> RiceEncodedStream {
        let mut stream = vec![];
        let mut extra_bits_len = 0;
        let divisor = 1.shl(rice_param) as u64;

        // Encode each residual value
        for &residual in residuals.iter() {
            let quotient = (residual.abs() as u64) / divisor;
            let remainder = (residual.abs() as u64) % divisor;
            let mut bits = vec![];

            // Unary encode the quotient
            for _ in 0..quotient {
                bits.push(1);
            }
            bits.push(0);

            // Binary encode the remainder
            for i in (0..rice_param).rev() {
                bits.push((remainder.shr(i)) as u8 & 1);
            }

            // Append bits to the stream
            for bit in bits {
                if stream.len() * 8 == extra_bits_len {
                    stream.push(0);
                }
                if bit == 1 {
                    stream[extra_bits_len / 8] |= 1.shl(7 - (extra_bits_len % 8)) as u8;
                }
                extra_bits_len += 1;
            }
        }

        RiceEncodedStream {
            stream,
            param: rice_param,
            extra_bits_len: extra_bits_len as u8,
        }
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
            streams.push(RiceEncoderOptions::encode(best_params[i as usize], &partition_residuals));
        }

        (streams, best_order)
    }
}