pub struct VarPredictor;

impl VarPredictor {
    /// Get the autocorrelation of a vector of samples
    ///
    /// The function computes the autocorrelations of the provided vector of
    /// data from `R[0]` until `R[max_lag]`. For example, if `max_lag` is 2, then
    /// the output contains three elements corresponding to R[0] until R[3],
    /// respectively
    pub fn get_autocorrelation(samples: &Vec <i64>, max_lag: u8) -> Vec <f64> {
        let max_lag = max_lag as usize;
        let mut autocorrelations = vec![0.0; max_lag + 1];

        for lag in 0..=max_lag {
            let mut sum = 0.0;
            for i in 0..(samples.len() - lag) {
                sum += samples[i] as f64 * samples[i + lag] as f64;
            }
            autocorrelations[lag] = sum;
        }

        autocorrelations
    }

    /// Get the predictor coefficients
    /// 
    /// `autoc` contains the autocorrelation vector where `autoc[i]` corresponds to
    /// the autocorrelation value of lag `i - 1`. `predictor_order` should be
    /// less than `autoc.len()`. The coefficients are computed using the Levinson-Durbin
    /// algorithm.
    pub fn get_predictor_coeffs(autoc: &Vec <f64>, predictor_order: u8) -> Vec <f64> {
        let mut coeffs = vec![0.0; predictor_order as usize];
        let mut error = autoc[0];

        for i in 0..predictor_order as usize {
            let mut k = autoc[i + 1];
            for j in 0..i {
                k -= coeffs[j] * autoc[i - j];
            }
            k /= error;

            for j in 0..(i + 1) / 2 {
                let temp = coeffs[j];
                coeffs[j] += k * coeffs[i - j - 1];
                coeffs[i - j - 1] += k * temp;
            }
            if i % 2 == 1 {
                coeffs[i / 2] += k * coeffs[i / 2];
            }

            coeffs[i] = k;
            error *= 1.0 - k * k;
        }

        coeffs
    }

    /// Get a the list of LPC coefficients until some provided predictor order inclusive.
    /// 
    /// For the return value `lpc_list`, `lpc_list[i]` contains a `Vec` of coefficients
    /// for predictor order `i + 1`. The Levinson-Durbin algorithm is used to progressively
    /// compute the LPC coefficients across multiple predictor orders.
    pub fn build_predictor_coeffs(autoc: &Vec <f64>, max_predictor_order: u8) -> Vec <Vec <f64>> {
        let mut lpc_list = Vec::new();

        for order in 1..=max_predictor_order {
            lpc_list.push(VarPredictor::get_predictor_coeffs(autoc, order));
        }

        lpc_list
    }

    /// Quantize the predictor coefficients and find their shift factor
    /// 
    /// The shift factor `S` is computed from the maximum absolute value of a coefficient
    /// `L_max`. This value is computed as `precision - lg(L_max)` or to
    /// the maximum shift value of 1 << 5 = 31, whichever is smaller. Note that it is
    /// possible for this shift factor to be negative. In that case, the shift value
    /// will still be used in quantizing the coefficients but its effective value
    /// will be zero.
    /// 
    /// Quantization involves converting the provided floating-point coefficients
    /// into integers. Each of the values are rounded up or down depending on
    /// some accummulated rounding error `\epsilon`. Initially, this error is zero.
    /// For each coefficient `L_i`, the coefficient is multiplied (for positive shift)
    /// or divided (for negative shift) by `1 << abs(S)` to get the raw value `L_i_r + \epsilon`.
    /// Then, `L_i_r + \epsilon` is rounded away from zero to get the quantized coefficient.
    /// The new rounding error `\epsilon = L_i_r + \epsilon - round(L_i_r)` is then updated for the
    /// next coefficient.
    pub fn quantize_coeffs(lpc_coefs: &Vec <f64>, mut precision: u8) -> (Vec <i64>, u8) {
        let mut l_max = 0.0;
        for &coef in lpc_coefs.iter() {
            if coef.abs() > l_max {
                l_max = coef.abs();
            }
        }

        let shift = (precision as f64 - l_max.log2()) as i32;

        precision = if shift > 31 { 31 } else { shift as u8 };

        let mut quantized = Vec::new();
        let mut error = 0.0;

        for &coef in lpc_coefs.iter() {
            let raw = coef * (1 << precision) as f64;
            let quant = (raw + error).round();
            error = raw + error - quant;
            quantized.push(quant as i64);
        }

        (quantized, precision)
    }

    /// Compute the residuals from a given linear predictor
    /// 
    /// The resulting vector `residual[i]` corresponds to the `i + predictor_order`th
    /// signal. The first `predictor_order` values of the residual are the "warm-up"
    /// samples, or the unencoded samples, equivalent to `&samples[..predictor_order]`.
    /// 
    /// The residuals are computed with the `samples` reversed. For some `i`th residual,
    /// `residual[i] = data[i] - (sum(dot(qlp_coefs, samples[i..(i - predictor_order)])) >> qlp_shift)`.
    pub fn get_residuals(samples: &Vec <i64>, qlp_coefs: &Vec <i64>, predictor_order: u8, qlp_shift: u8) -> Vec <i64> {
        let mut residuals = Vec::new();
        for i in 0..predictor_order as usize {
            residuals.push(samples[i]);
        }

        for i in predictor_order as usize..samples.len() {
            let mut prediction = 0;
            for j in 0..predictor_order as usize {
                prediction += qlp_coefs[j] * samples[i - j - 1];
            }
            residuals.push(samples[i] - (prediction >> qlp_shift));
        }

        residuals
    }

    /// compute the quantized LPC coefficients, precision, and shift for the given
    /// predictor order
    pub fn get_predictor_coeffs_from_samples(samples: &Vec <i64>, predictor_order: u8, bps: u8, block_size: u64) -> (Vec <i64>, u8, u8) {
        let autoc = VarPredictor::get_autocorrelation(samples, predictor_order);
        let lpc_coefs = VarPredictor::get_predictor_coeffs(&autoc, predictor_order);
        let precision = VarPredictor::get_best_precision(bps, block_size);
        let (quantized, shift) = VarPredictor::quantize_coeffs(&lpc_coefs, precision);
        (quantized, precision, shift)
    }

    /// Get the quantized LPC coefficients, precision, and shift for the best predictor order
    /// for the given sample
    /// 
    /// This function selects the best predictor order by finding the order that yields the
    /// absolute minimum sum of residuals. Note that the maximmum predictor order is 32.
    pub fn get_best_lpc(samples: &Vec <i64>, bps: u8, block_size: u64) -> (Vec <i64>, u8, u8) {
        let mut best_coeffs = Vec::new();
        let mut best_precision = 0;
        let mut best_shift = 0;
        let mut min_residual_sum = i64::MAX;

        for order in 1..=32 {
            let (coeffs, precision, shift) = VarPredictor::get_predictor_coeffs_from_samples(samples, order, bps, block_size);
            let residuals = VarPredictor::get_residuals(samples, &coeffs, order, shift);
            let mut residual_sum = 0;
            for &residual in &residuals {
                residual_sum += residual.abs();
            }

            if residual_sum < min_residual_sum {
                min_residual_sum = residual_sum;
                best_coeffs = coeffs;
                best_precision = precision;
                best_shift = shift;
            }
        }

        (best_coeffs, best_precision, best_shift)
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
    pub fn get_best_precision(bps: u8, block_size: u64) -> u8 {
        if bps < 16 {
            return std::cmp::max(1, 2 + bps / 2);
        }
        match (bps, block_size) {
            (16, 192) => 7,
            (16, 384) => 8,
            (16, 576) => 9,
            (16, 1152) => 10,
            (16, 2304) => 11,
            (16, 4608) => 12,
            (16, _) => 13,
            (_, 384) => 12,
            (_, 1152) => 13,
            _ => 14,
        }
    }
}