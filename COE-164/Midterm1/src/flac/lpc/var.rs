use std::cmp::max;

pub struct VarPredictor;

impl VarPredictor {
    /// Get the correlation of a vector of data
    pub fn get_autocorrelation(data: &Vec <i32>, lag: u32) -> Vec <f64> {
        let n = data.len();
        let lag = lag as usize;
        let mut autocorrelation = vec![0.0; lag + 1];

        for i in 0..=lag {
            let mut sum = 0.0 ;
            for n in 0..(n - i) {
                sum += (data[n] * data[n + i]) as f64;
            }
            autocorrelation[i] = sum;
        }
        
        autocorrelation
    }
    /// Get the predictor coefficients
    /// 
    /// The coefficients are computed using the Levinson-Durbin algorithm.
    pub fn get_predictor_coeffs(autoc: &Vec <f64>, predictor_order: u32) -> Vec <f64> {
        let mut a = vec![0.0; (predictor_order + 1) as usize];
        let mut e = autoc[0];

        for i in 1..=predictor_order as usize {
            let mut acc = 0.0 ;
            for j in 1..i {
                acc += a[j] * autoc[i - j];
            }
            let k = (autoc[i] - acc) / e;
            a[i] = k;

            for j in 1..i {
                a[j] = a[j] - k * a[i - j];
            }
            e *= 1.0 - k * k;
        }
        a
    }

    pub fn quantize_coeffs(lpc_coefs: &Vec <f64>, mut precision: u32) -> (Vec <u32>, u32) {
        let mut qlp_coefs = Vec::new();

        for &coef in lpc_coefs {
            let q_coef = (coef * (1 << precision) as f64).round() as i32;
            qlp_coefs.push(q_coef as u32);
        }

        (qlp_coefs, precision)
    }

    /// Compute the residuals from a given linear predictor
    /// 
    /// The residuals are computed with the provided quantized coefficients
    /// `qlp_coefs` and shift factor `qlp_shift`.
    pub fn get_residuals(data: &Vec <i32>, qlp_coefs: &Vec <u32>, predictor_order: u32, qlp_shift: u32) -> Option <Vec <i32>> {
        let n = data.len();
        let predictor_order = predictor_order as usize;
        
        if predictor_order >= n {
            return None;
        }
    
        let mut residuals = vec![0; n];
        let mut unquantized_coeffs = vec![0.0; predictor_order];

        // Unquantize the coefficients
        for i in 0..predictor_order {
            unquantized_coeffs[i] = qlp_coefs[i] as f64 / (1 << qlp_shift) as f64;
        }

        // Compute the residuals
        for i in predictor_order..n {
            let mut predicted = 0.0;
            for j in 0..predictor_order {
                predicted += unquantized_coeffs[j] * data[i - 1 - j] as f64;
            }
            residuals[i] = data[i] - predicted as i32;
        }

        Some(residuals)
    }

    /// Get the best coefficient precision
    /// 
    /// FLAC uses the bit depth and block size to determine the best coefficient
    /// precision. By default, the precision is 14 bits but can be one of the
    /// following depending on several parameters:
    /// 
    /// | Bit depth | Block size |     Best precision      |
    /// |-----------|------------|-------------------------|
    /// |   < 16    |     any    | max(1, 2 + bit_depth/2) |
    /// |     16    |     192    |           7             |
    /// |     16    |     384    |           8             |
    /// |     16    |     576    |           9             |
    /// |     16    |    1152    |          10             |
    /// |     16    |    2304    |          11             |
    /// |     16    |    4608    |          12             |
    /// |     16    |     any    |          13             |
    /// |   > 16    |     384    |          12             |
    /// |   > 16    |    1152    |          13             |
    /// |   > 16    |     any    |          14             |
    pub fn get_best_precision(bps: u32, block_size: u32) -> u32 {
        if bps < 16 {
            return max(1, 2 + (bps / 2));
        } else {
            match bps {
                16 => match block_size {
                    192 => 7,
                    384 => 8,
                    576 => 9,
                    1152 => 10,
                    2304 => 11,
                    4608 => 12,
                    _ => 13,
                },
                _ => match block_size {
                    384 => 12,
                    1152 => 13,
                    _ => 14,
                },
            }
        }
    }
}