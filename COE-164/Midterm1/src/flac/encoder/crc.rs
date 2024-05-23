/// Represents a kind of CRC encoding
/// 
/// This struct is used to configure the type of CRC encoding to use.
/// For example, if the generator polynomial for a CRC8 encoding is:
/// 
/// `x^8 + x^2 + x^1 + 1`
/// 
/// Then, the value of `poly` should be 0b0000_0111 (note the missing
/// MSB `1` bit) and `poly_len` should be `u8`.
pub struct CrcOptions <T> {
    poly: T,
    poly_len: T,
}


impl <T> CrcOptions <T> {
    /// Create a builder to the CRC encoder
    pub fn new(poly: T, poly_len: T) -> Self {  // todo!()
        Self {
            poly,
            poly_len,
        }
    }
}

impl CrcOptions <u8> {
    /// Encode data using CRC8 encoding
    /// 
    /// This method is available only if `CrcOptions` is of type `u8`.
    pub fn build_crc8(&self, data: &Vec <u8>) -> u8 { //
        let mut crc: u8 = 0;

        for byte in data {
            crc ^= byte;                    // XOR
            for _ in 0..8 {
                if crc & 0x80 != 0 {        // (0x80 = 0b1000 0000)
                    crc = (crc << 1) ^ self.poly;   // If the MSB is 1, XOR the crc shifted by 1 with the polynomial 
                } else {
                    crc <<= 1;              // Else, shift the crc to the left by 1 
                }
            }
        }
        crc
    }
}

impl CrcOptions <u16> {
    /// Encode data using CRC16 encoding
    /// 
    /// This method is available only if `CrcOptions` is of type `u16`.
    pub fn build_crc16(&self, data: &Vec <u16>) -> u16 {
        let mut crc: u16 = 0;

        for word in data {
            crc ^= word;                    // XOR
            for _ in 0..16 {
                if crc & 0x8000 != 0 {      // (0x8000 = 0b1000 0000 0000 0000)
                    crc = (crc << 1) ^ self.poly;   // If the MSB is 1, XOR the crc shifted by 1 with the polynomial
                } else {
                    crc <<= 1;              // Else, shift the crc to the left by 1 
                }
            }
        }
        crc
    }
}