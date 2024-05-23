pub struct FixedPredictor;

impl FixedPredictor {
    /// Get order that yields the least sum of residuals
    /// 
    /// The predictor orders are from 0 to 4 inclusive and is retrieved
    /// by finding the predictor that yields the *minimum* absolute
    /// sum of residuals for the given `data` and derived predictor.
    pub fn best_predictor_order(data: &Vec <i64>) -> Option <u8> {
        let mut min_residual_sum = i64::MAX;
        let mut best_order = None;

        for order in 0..=4 {
            if let Some(residuals) = FixedPredictor::get_residuals(data, order) {
                let residual_sum = FixedPredictor::get_sum(&residuals);
                if residual_sum < min_residual_sum {
                    min_residual_sum = residual_sum;
                    best_order = Some(order);
                }
            }
        }

        best_order
    }

    fn get_sum(residuals: &Vec <i64>) -> i64 {
        let mut sum = 0 as i64;
        for &residual in residuals {
            sum += residual.abs() as i64;
        }
        sum
    }

    /// Get residuals of a fixed predictor order 
    /// 
    /// The predictor orders are from 0 to 4 inclusive and corresponds
    /// to one of the five "fixed" predictor orders written in the FLAC
    /// specification. The predictor orders are defined as follows:
    /// 
    /// 0: r[i] = 0
    /// 1: r[i] = data[i - 1]
    /// 2: r[i] = 2 * data[i - 1] - data[i - 2]
    /// 3: r[i] = 3 * data[i - 1] - 3 * data[i - 2] + data[i - 3]
    /// 4: r[i] = 4 * data[i - 1] - 6 * data[i - 2] + 4 data[i - 3] - data[i - 4]
    /// 
    /// This function returns a vector with each element containing data[i] - r[i].
    /// 
    /// # Errors
    /// `None` is returned if an error occurs in the function. This includes whether
    /// the predictor order provided is not within 0 and 4 inclusive and whether the
    /// size of `data` is less than the predictor order.
    pub fn get_residuals(data: &Vec <i64>, predictor_order: u8) -> Option <Vec <i64>> {
        // Check if predictor order is between 0 and 4 inclusive and if the size of the data is at least the predictor order
        if predictor_order > 4 || data.len() < predictor_order as usize {
            return None;
        }

        let mut residuals = Vec::new();
        for i in 0..data.len() {
            let r_i = match predictor_order {
                0 => 0,
                1 => if i >= 1 { data[i - 1] } else { 0 },
                2 => if i >= 2 { 2 * data[i - 1] - data[i - 2] } else { 0 },
                3 => if i >= 3 { 3 * data[i - 1] - 3 * data[i - 2] + data[i - 3] } else { 0 },
                4 => if i >= 4 { 4 * data[i - 1] - 6 * data[i - 2] + 4 * data[i - 3] - data[i - 4] } else { 0 },
                _ => return None,
            };
            residuals.push(data[i] - r_i);
        }
        if residuals.is_empty() {
            return None;
        }
        Some(residuals)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod lpc_fixed {
        use super::*;

        #[test]
        fn test_best_predictor_order() {    
            let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let best_order = FixedPredictor::best_predictor_order(&data);
            assert_eq!(best_order, Some(2)); 
        }
    
        #[test]
        fn test_get_residuals_order_0() {   // Check residual for order 0
            let data = vec![1, 2, 3, 4, 5];
            let residuals = FixedPredictor::get_residuals(&data, 0).unwrap();
            assert_eq!(residuals, vec![1, 2, 3, 4, 5]); 
        }
    
        #[test]
        fn test_get_residuals_order_1() {   // Check residual for order 1
            let data = vec![1, 2, 3, 4, 5];
            let residuals = FixedPredictor::get_residuals(&data, 1).unwrap();
            assert_eq!(residuals, vec![1, 1, 1, 1, 1]);
        }
    
        #[test]
        fn test_get_residuals_order_2() {   // Check residual for order 2
            let data = vec![1, 2, 3, 4, 5];
            let residuals = FixedPredictor::get_residuals(&data, 2).unwrap();
            assert_eq!(residuals, vec![1, 2, 0, 0, 0]); 
        }

        
        #[test]
        fn test_get_residuals_order_3() {
            let data = vec![1, 2, 3, 4, 5];
            let residuals = FixedPredictor::get_residuals(&data, 3).unwrap();
            assert_eq!(residuals, vec![1, 2, 3, 0, 0]); 
        }

        #[test]
        fn test_get_residuals_order_4() {
            let data = vec![1, 2, 3, 4, 5];
            let residuals = FixedPredictor::get_residuals(&data, 4).unwrap();
            assert_eq!(residuals, vec![1, 2, 3, 4, 0]); 
        }

        #[test]
        fn test_get_residuals_non_linear_order_1() {
            let data = vec![1, 3, 6, 10, 15];
            let residuals = FixedPredictor::get_residuals(&data, 1).unwrap();
            assert_eq!(residuals, vec![1, 2, 3, 4, 5]); // For order 1, r[i] = data[i-1], residuals should be [2, 3, 4, 5]
        }

        #[test]
        fn test_get_residuals_non_linear_order_2() {
            let data = vec![1, 3, 6, 10, 15];
            let residuals = FixedPredictor::get_residuals(&data, 2).unwrap();
            assert_eq!(residuals, vec![1, 3, 1, 1, 1]);
        }

        #[test]
        fn test_get_residuals_non_linear_order_3() {
            let data = vec![1, 3, 6, 10, 15];
            let residuals = FixedPredictor::get_residuals(&data, 3).unwrap();
            assert_eq!(residuals, vec![1, 3, 6, 0, 0]);
        }

        #[test]
        fn test_get_residuals_non_linear_order_4() {
            let data = vec![1, 3, 6, 10, 15];
            let residuals = FixedPredictor::get_residuals(&data, 4).unwrap();
            assert_eq!(residuals, vec![1, 3, 6, 10, 0]); 
        }
    
        #[test]
        fn test_get_residuals_invalid_order() { // Predictor order is invalid, should return None
            let data = vec![1, 2, 3, 4, 5];
            let residuals = FixedPredictor::get_residuals(&data, 5);
            assert!(residuals.is_none()); 
        }
    
        #[test]
        fn test_get_residuals_insufficient_data() { // Data length is insufficient, should return None
            let data = vec![1];
            let residuals = FixedPredictor::get_residuals(&data, 5);
            assert!(residuals.is_none()); 
        }

        #[test]
        fn test_get_residuals_empty_data() { // Data length is equal to order, should return None
            let data = vec![1];
            let residuals = FixedPredictor::get_residuals(&data, 2);
            assert!(residuals.is_none()); 
        }

        #[test]
        fn test_get_residuals_repeated_values() {
            let data = vec![5, 5, 5, 5, 5];
            let residuals = FixedPredictor::get_residuals(&data, 1).unwrap();
            assert_eq!(residuals, vec![5, 0, 0, 0, 0]); 
        }
    
        #[test]
        fn test_best_predictor_order_non_linear() {
            let data = vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100];
            let best_order = FixedPredictor::best_predictor_order(&data);
            assert_eq!(best_order, Some(3)); 
        }
    }
}