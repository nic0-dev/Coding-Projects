pub struct Utf8Encoder;

impl Utf8Encoder {
    /// Encode a number into its UTF-9 equivalent encoding
    /// 
    /// Although UTF-8 encoding is for characters, characters are
    /// mapped to certain numbers.
    pub fn encode(mut num: u64) -> Vec <u8> {
        let mut encoded_bytes = Vec::new();

        if num <= 0x7F {
            // 1 byte: 0xxxxxxx
            encoded_bytes.push(num as u8);
        } else if num <= 0x7FF {
            // 2 bytes: 110xxxxx 10xxxxxx
            // First byte: 110xxxxx
            encoded_bytes.push(0xC0 | ((num >> 6) as u8) & 0x1F); 
            // Second byte: 10xxxxxx
            encoded_bytes.push(0x80 | (num & 0x3F) as u8); 
        } else if num <= 0xFFFF {
            // 3 bytes: 1110xxxx 10xxxxxx 10xxxxxx
            // First byte: 1110xxxx
            encoded_bytes.push(0xE0 | ((num >> 12) as u8) & 0x0F);
            // Second byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 6) & 0x3F) as u8); 
            // Third byte: 10xxxxxx
            encoded_bytes.push(0x80 | (num & 0x3F) as u8); 
        } else if num <= 0x1FFFFF {
            // 4 bytes: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
            // First byte: 11110xxx
            encoded_bytes.push(0xF0 | ((num >> 18) as u8) & 0x07); 
            // Second byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 12) & 0x3F) as u8);
            // Third byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 6) & 0x3F) as u8); 
            // Fourth byte: 10xxxxxx
            encoded_bytes.push(0x80 | (num & 0x3F) as u8); 
        } else if num <= 0x3FFFFFF {
            // 5 bytes: 111110xx 10xxxxxx 10xxxxxx 10xxxxxx 10xxxxxx
            // First byte: 111110xx
            encoded_bytes.push(0xF8 | ((num >> 24) as u8) & 0x03); 
            // Second byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 18) & 0x3F) as u8);
            // Third byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 12) & 0x3F) as u8); 
            // Fourth byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 6) & 0x3F) as u8);
            // Fifth byte: 10xxxxxx
            encoded_bytes.push(0x80 | (num & 0x3F) as u8); 
        } else if num <= 0x7FFFFFFF {
            // 6 bytes: 1111110x 10xxxxxx 10xxxxxx 10xxxxxx 10xxxxxx 10xxxxxx
            // First byte: 1111110x
            encoded_bytes.push(0xFC | ((num >> 30) as u8) & 0x01); 
            // Second byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 24) & 0x3F) as u8); 
            // Third byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 18) & 0x3F) as u8); 
            // Fourth byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 12) & 0x3F) as u8); 
            // Fifth byte: 10xxxxxx
            encoded_bytes.push(0x80 | ((num >> 6) & 0x3F) as u8); 
            // Sixth byte: 10xxxxxx
            encoded_bytes.push(0x80 | (num & 0x3F) as u8); 
        }

        encoded_bytes
    }
}

/* 
pub struct Utf8Encoder;

impl Utf8Encoder {
    pub fn encode(mut num: u64) -> Vec<u8> {
        let mut encoded_bytes = Vec::new();

        if num < 0x80 {
            encoded_bytes.push(num as u8);
        } else {
            let mut bytes = Vec::new();
            while num > 0 {
                bytes.push((num & 0x3F) as u8 | 0x80);
                num >>= 6;
            }
            bytes.reverse();
            bytes[0] &= 0x3F;
            bytes[0] |= 0xC0;
            encoded_bytes.extend(bytes);
        }

        encoded_bytes
    }
}
*/