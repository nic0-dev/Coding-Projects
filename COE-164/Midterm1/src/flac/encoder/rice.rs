pub struct RiceEncoder;

impl RiceEncoder {
    /// Let S be the number to be encoded
    /// Let M be the Rice parameter, which should be a power of 2
    pub fn encode(s: u32, m: u32) -> Vec<u8> {
        let mut encoded_bits = Vec::new();
        let k = (m as f64).log2() as u32;       // Number of bits to represent B
        
        // Unary part: U = S >> K
        let u = s >> k;
        // Unary encoding: represent U in unary with '0's followed by a '1'
        for _ in 0..u {
            encoded_bits.push(0);
        }
        encoded_bits.push(1); 

        // Truncated Binary part: B = S & (M - 1)
        let b = s & (m - 1);
        // Binary encoding: represent B in binary, padded to the left with zeros until it is of length K
        for i in (0..k).rev() { // MSB -> LSB
            encoded_bits.push(((b >> i) & 1) as u8);
        }

        encoded_bits
    }
}
