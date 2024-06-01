use core::fmt;
use std::fs::File;
use std::path::Path;
use std::error;
use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{ByteOrder, LittleEndian};

// ------------------------------------------------------------------ //

// Represents a PCM WAV file
pub struct PCMWaveInfo {
    pub riff_header: RiffChunk,
    pub fmt_header: PCMWaveFormatChunk,
    pub data_chunks: Vec <PCMWaveDataChunk>,
}

// Represents the RIFF chunk of a WAV file
pub struct RiffChunk {
    pub file_size: u32,
    pub is_big_endian: bool,
}

// Represents the format chunk of a WAV file
#[derive(Clone, Copy)]
pub struct PCMWaveFormatChunk {             // Holds the audio format details for interpreting audio data
    pub num_channels: u16,
    pub samp_rate: u32,
    pub bps: u16,
}

// Represents a data chunk in a WAV file
pub struct PCMWaveDataChunk {               // To manage and read the audio sample data
    pub size_bytes: u32,
    pub format: PCMWaveFormatChunk,
    pub data_buf: io::BufReader<File>,
}

// Represents an interator over a window of data chunks
// Used to iterate over chunks of inter-channel samples
// Facilitates batch processing of audio samples
pub struct PCMWaveDataChunkWindow {
    chunk_size: usize,
    data_chunk: PCMWaveDataChunk
}

/// Represents a WAV reader
pub struct WaveReader;

// Represents possible errors in the WAV Reader
#[derive(Debug)]
#[derive(PartialEq)]
pub enum WaveReaderError {
    NotRiffError,
    NotWaveError,
    NotPCMError,
    ChunkTypeError,
    DataAlignmentError,
    ReadError,
}

impl WaveReader {
    // Opens a PCM WAV file
    // Reads RIFF, format, and data chunk in sequence
    pub fn open_pcm(file_path: &str) -> Result <PCMWaveInfo, WaveReaderError> {
        let mut file = File::open(Path::new(file_path))?;

        // Read RIFF chunk
        let riff_chunk = WaveReader::read_riff_chunk(&mut file)?;
        // Check if it's a valid WAVE file
        if riff_chunk.is_big_endian {
            return Err(WaveReaderError::NotWaveError);
        }

        // Read format chunk
        let fmt_chunk = WaveReader::read_fmt_chunk(&mut file)?;

        // Read the data chunks
        let mut data_chunks = Vec::new();
        while let Ok(data_chunk) = WaveReader::read_data_chunk(file.seek(SeekFrom::Current(0))?, &fmt_chunk, file.try_clone()?) {
            data_chunks.push(data_chunk);
        }
 
        Ok(PCMWaveInfo { riff_header: riff_chunk, fmt_header: fmt_chunk, data_chunks })
    }
    // Reads the RIFF chunk from the file
    fn read_riff_chunk(fh: &mut File) -> Result <RiffChunk, WaveReaderError> {
        let mut riff_header = [0u8; 12];        // Buffer to read the first 12 Bytes
        fh.read_exact(&mut riff_header)?;

        // Check if the header is 'RIFF' or 'RIFX'
        if &riff_header[0..4] != b"RIFF" && &riff_header[0..4] != b"RIFX" {
            return Err(WaveReaderError::NotRiffError);
        }

        // Determine the endianness and read the file size accordingly
        let is_big_endian = &riff_header[0..4] == b"RIFX";
        let file_size = if is_big_endian {
            u32::from_be_bytes([riff_header[4], riff_header[5], riff_header[6], riff_header[7]])
        } else {
            LittleEndian::read_u32(&riff_header[4..8])
        };

        // Check if the file type is 'WAVE'
        if &riff_header[8..12] != b"WAVE" {
            return Err(WaveReaderError::NotWaveError);
        }

        Ok(RiffChunk { file_size, is_big_endian })
    }
    // Reads the format chunk from the file
    fn read_fmt_chunk(fh: &mut File) -> Result <PCMWaveFormatChunk, WaveReaderError> {
        let mut fmt_header = [0u8; 24];         // Buffer to read the next 24 Bytes
        fh.read_exact(&mut fmt_header)?;        

        // Check if the chunk type is 'fmt '
        if &fmt_header[0..4] != b"fmt " {
            return Err(WaveReaderError::ChunkTypeError);
        }

        // Read the format details using LittleEndian
        let num_channels = LittleEndian::read_u16(&fmt_header[10..12]);
        let samp_rate = LittleEndian::read_u32(&fmt_header[12..16]);
        let byte_rate = LittleEndian::read_u32(&fmt_header[16..20]);
        let block_align = LittleEndian::read_u16(&fmt_header[20..22]);
        let bps = LittleEndian::read_u16(&fmt_header[22..24]);

        let fmt_chunk = PCMWaveFormatChunk { num_channels, samp_rate, bps };

        // Validate byte rate and block alignment
        if byte_rate != fmt_chunk.byte_rate() {
            return Err(WaveReaderError::DataAlignmentError);
        }
        if block_align != fmt_chunk.block_align() {
            return Err(WaveReaderError::DataAlignmentError);
        }

        Ok(fmt_chunk)
    }
    // Reads a data chunk from the file
    fn read_data_chunk(start_pos: u64, fmt_info: &PCMWaveFormatChunk, mut fh: File) -> Result <PCMWaveDataChunk, WaveReaderError> {
        fh.seek(SeekFrom::Start(start_pos))?;

        let mut data_header = [0u8; 8];
        fh.read_exact(&mut data_header)?;

        // Check if the chunk type is 'data'
        if &data_header[0..4] != b"data" {
            return Err(WaveReaderError::ChunkTypeError);
        }
        // Read the size of the data chunk using LittleEndian
        let size_bytes = LittleEndian::read_u32(&data_header[4..8]);        // DSubChunkSize
        let data_buf = io::BufReader::new(fh);

        Ok(PCMWaveDataChunk { size_bytes, format: *fmt_info, data_buf })
    }
}

impl error::Error for WaveReaderError {}

impl fmt::Display for WaveReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            WaveReaderError::NotRiffError => "Not a valid RIFF header",
            WaveReaderError::NotWaveError => "Not a valid WAVE file",
            WaveReaderError::NotPCMError => "Not a PCM format",
            WaveReaderError::ChunkTypeError => "Chunk type error",
            WaveReaderError::DataAlignmentError => "Data alignment error",
            WaveReaderError::ReadError => "Error reading from file",
        })
    }
}

impl From <io::Error> for WaveReaderError {
    fn from(_: io::Error) -> Self {
        WaveReaderError::ReadError
    }
}

impl fmt::Display for PCMWaveInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // WAVE File <FileSize> bytes, <BitDepth>-bit <NumChannels> channels, <SampleRate>Hz, <NumDataChunks> data chunks
        write!(f, "WAVE File {} bytes, {}-bit {} channels, {}Hz, {} data chunks",
            self.riff_header.file_size,
            self.fmt_header.bps,
            self.fmt_header.num_channels,
            self.fmt_header.samp_rate,
            self.data_chunks.len())
    }
}

impl PCMWaveFormatChunk {
    // Calculates the byte rate of the WAV file
    pub fn byte_rate(&self) -> u32 {
        self.samp_rate * (self.num_channels as u32) * (self.bps as u32) / 8
    }
    // Calculates the block alignment of the WAV file
    pub fn block_align(&self) -> u16 {
        self.num_channels * self.bps / 8
    }
}

// Defines how to iterate over inter-channel samples
impl Iterator for PCMWaveDataChunk {
    type Item = Vec <i64>;

    fn next(&mut self) -> Option <Self::Item> {
        let mut sample = vec![0; self.format.num_channels as usize];
        for ch in &mut sample {
            let mut buf = [0u8; 2];
            if self.data_buf.read_exact(&mut buf).is_err() {
                return None;
            }
            *ch = LittleEndian::read_i16(&buf) as i64;
        }
        Some(sample)
    }
}

// Defines how to iterate over chunks of inter-channel samples
impl Iterator for PCMWaveDataChunkWindow {
    type Item = Vec <Vec <i64>>;

    fn next(&mut self) -> Option <Self::Item> {
        let mut batch = Vec::with_capacity(self.chunk_size);
        for _ in 0..self.chunk_size {
            if let Some(sample) = self.data_chunk.next() {
                batch.push(sample);
            } else {
                break;
            }
        }
        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }
}

impl PCMWaveDataChunk {
    // Consumes a data chunk and returns an iterator for single inter-channel samples
    // Helps iterate over individual audio samples.
    pub fn chunks_byte_rate(self) -> PCMWaveDataChunkWindow {
        PCMWaveDataChunkWindow {
            chunk_size: self.format.byte_rate() as usize,
            data_chunk: self,
        }
    }
    // Consumes a data chunk and returns an iterator for a specified number of inter-channel samples
    // Enables batch processing of audio samples
    pub fn chunks(self, chunk_size: usize) -> PCMWaveDataChunkWindow {
        PCMWaveDataChunkWindow {
            chunk_size,
            data_chunk: self,
        }
    }
}

// TODO: Add more tests here!
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod read_riff {
        use super::*;
        use std::io::Write;

        fn create_temp_file(file_name: &str, content: &[u8]) -> Result <(), io::Error> {
            let mut file = File::create(file_name)?;
            file.write_all(content)?;

            Ok(())
        }
        
        macro_rules! internal_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() -> Result <(), WaveReaderError> {
                    let (input, (will_panic, expected)) = $value;

                    let file_name = format!("midp_{}.wav.part", stringify!($name));
                    let result;
                    {
                        create_temp_file(&file_name, input)?;
                        let mut input_fh = File::open(&file_name)?;
                        result = WaveReader::read_riff_chunk(&mut input_fh);
                    }
                    std::fs::remove_file(&file_name)?;

                    if will_panic {
                        assert!(result.is_err());
                    }
                    else if let Ok(safe_result) = result {
                        assert_eq!(expected.file_size, safe_result.file_size);
                        assert_eq!(expected.is_big_endian, safe_result.is_big_endian);
                    }
                    else {
                        result?;
                    }

                    Ok(())
                }
            )*
            }
        }
        
        internal_tests! {
            it_valid_le_00: (
                &[0x52, 0x49, 0x46, 0x46, 0x0, 0x0, 0x0, 0x0, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 0,
                        is_big_endian: false,
                    },
                )),
            it_valid_le_01: (
                &[0x52, 0x49, 0x46, 0x46, 0x80, 0x0, 0x0, 0x0, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 128,
                        is_big_endian: false,
                    },
                )),
            it_valid_le_02: (
                &[0x52, 0x49, 0x46, 0x46, 0x1C, 0x40, 0x36, 0x0, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 3_555_356,
                        is_big_endian: false,
                    },
                )),
            it_valid_be_00: (
                &[0x52, 0x49, 0x46, 0x58, 0x0, 0x0, 0x0, 0x0, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 0,
                        is_big_endian: true,
                    },
                )),
            it_valid_be_01: (
                &[0x52, 0x49, 0x46, 0x58, 0x00, 0x0, 0x0, 0x80, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 128,
                        is_big_endian: true,
                    },
                )),
            it_valid_be_02: (
                &[0x52, 0x49, 0x46, 0x58, 0x00, 0x36, 0x40, 0x1C, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 3_555_356,
                        is_big_endian: true,
                    },
                )),
            it_bad_riff: (
                &[0x00, 0x49, 0x46, 0x46, 0x00, 0x36, 0x40, 0x1C, 0x57, 0x41, 0x56, 0x45],
                (
                    true,
                    RiffChunk {
                        file_size: 0,
                        is_big_endian: false,
                    },
                )),
            it_bad_wave: (
                &[0x52, 0x49, 0x46, 0x46, 0x00, 0x36, 0x40, 0x1C, 0x57, 0x41, 0x56, 0x00],
                (
                    true,
                    RiffChunk {
                        file_size: 0,
                        is_big_endian: false,
                    },
                )),
        }
    }

    #[cfg(test)]
    mod read_wav_fmt {
        use super::*;
        use std::io::Write;

        fn create_temp_file(file_name: &str, content: &[u8]) -> Result <(), io::Error> {
            let mut file = File::create(file_name)?;
            file.write_all(content)?;

            Ok(())
        }
        
        macro_rules! internal_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() -> Result <(), WaveReaderError> {
                    let (input, (will_panic, expected)) = $value;

                    let file_name = format!("midp_{}.wav.part", stringify!($name));
                    let result;
                    {
                        create_temp_file(&file_name, input)?;
                        let mut input_fh = File::open(&file_name)?;
                        result = WaveReader::read_fmt_chunk(&mut input_fh);
                    }
                    std::fs::remove_file(&file_name)?;

                    if will_panic {
                        assert!(result.is_err());
                    }
                    else if let Ok(safe_result) = result {
                        assert_eq!(expected.num_channels, safe_result.num_channels);
                        assert_eq!(expected.samp_rate, safe_result.samp_rate);
                        assert_eq!(expected.bps, safe_result.bps);
                    }
                    else {
                        result?;
                    }

                    Ok(())
                }
            )*
            }
        }
        
        internal_tests! {
            it_valid_00: (
                &[
                    0x66, 0x6d, 0x74, 0x20,
                    0x10, 0x0, 0x0, 0x0,
                    0x01, 0x0,
                    0x01, 0x0,
                    0x44, 0xac, 0x0, 0x0,
                    0x44, 0xac, 0x0, 0x0,
                    0x01, 0x00, 0x08, 0x0,
                ],
                (
                    false,
                    PCMWaveFormatChunk {
                        num_channels: 1,
                        samp_rate: 44100,
                        bps: 8,
                    },
                )),
            it_valid_01: (
                &[
                    0x66, 0x6d, 0x74, 0x20,
                    0x10, 0x0, 0x0, 0x0,
                    0x01, 0x0,
                    0x02, 0x0,
                    0x44, 0xac, 0x0, 0x0,
                    0x88, 0x58, 0x01, 0x0,
                    0x02, 0x00, 0x08, 0x0,
                ],
                (
                    false,
                    PCMWaveFormatChunk {
                        num_channels: 2,
                        samp_rate: 44100,
                        bps: 8,
                    },
                )),
            it_valid_02: (
                &[
                    0x66, 0x6d, 0x74, 0x20,
                    0x10, 0x0, 0x0, 0x0,
                    0x01, 0x0,
                    0x02, 0x0,
                    0x44, 0xac, 0x0, 0x0,
                    0x10, 0xb1, 0x02, 0x0,
                    0x04, 0x00, 0x10, 0x0,
                ],
                (
                    false,
                    PCMWaveFormatChunk {
                        num_channels: 2,
                        samp_rate: 44100,
                        bps: 16,
                    },
                )),
        }
    }

    #[cfg(test)]
    mod read_data_fmt {
        use super::*;
        use std::io::Write;
    
        fn create_temp_file(file_name: &str, content: &[u8]) -> Result<(), io::Error> {
            let mut file = File::create(file_name)?;
            file.write_all(content)?;
    
            Ok(())
        }
        
        #[test]
        fn it_valid_data_chunk() -> Result<(), WaveReaderError> {
            let input = &[
                0x64, 0x61, 0x74, 0x61, // "data"
                0x0C, 0x00, 0x00, 0x00, // Chunk size: 12 bytes
                // Sample data: two channels, each with a 16-bit sample
                0x00, 0x00, 0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x01, 0x00, 0xFF, 0xFF,
            ];
    
            let file_name = "data_chunk_test.wav";
            let result;
            {
                create_temp_file(&file_name, input)?;
                let input_fh = File::open(&file_name)?;
                result = WaveReader::read_data_chunk(0, &PCMWaveFormatChunk {
                    num_channels: 2,
                    samp_rate: 44100,
                    bps: 16,
                }, input_fh);
            }
            std::fs::remove_file(&file_name)?;
    
            if let Ok(data_chunk) = result {
                assert_eq!(data_chunk.size_bytes, 12);
                // Ensure format details are correct
                assert_eq!(data_chunk.format.num_channels, 2);
                assert_eq!(data_chunk.format.samp_rate, 44100);
                assert_eq!(data_chunk.format.bps, 16);
            } else {
                result?;
            }
    
            Ok(())
        }
    
        #[test]
        fn it_invalid_chunk_type() -> Result<(), WaveReaderError> {
            let input = &[
                0x73, 0x75, 0x62, 0x2D, // "sub-"
                0x0C, 0x00, 0x00, 0x00, // Chunk size: 12 bytes
                0x00, 0x00, 0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x01, 0x00, 0xFF, 0xFF,
            ];
    
            let file_name = "invalid_chunk_test.wav";
            let result;
            {
                create_temp_file(&file_name, input)?;
                let input_fh = File::open(&file_name)?;
                result = WaveReader::read_data_chunk(0, &PCMWaveFormatChunk {
                    num_channels: 2,
                    samp_rate: 44100,
                    bps: 16,
                }, input_fh);
            }
            std::fs::remove_file(&file_name)?;
    
            assert!(result.is_err());
    
            Ok(())
        }
    
        // Add more test cases as needed
    }
    
    #[cfg(test)]
    mod pcm_wave_info_display {
        use super::*;
    
        #[test]
        fn test_display() {
            let wave_info = PCMWaveInfo {
                riff_header: RiffChunk {
                    file_size: 1234,
                    is_big_endian: false,
                },
                fmt_header: PCMWaveFormatChunk {
                    num_channels: 2,
                    samp_rate: 44100,
                    bps: 16,
                },
                data_chunks: Vec::new(),
            };
    
            assert_eq!(
                format!("{}", wave_info),
                "WAVE File 1234 bytes, 16-bit 2 channels, 44100Hz, 0 data chunks"
            );
        }
    }
    
    #[cfg(test)]
    mod pcm_wave_format_chunk_methods {
        use super::*;
    
        #[test]
        fn test_byte_rate() {
            let format_chunk = PCMWaveFormatChunk {
                num_channels: 2,
                samp_rate: 44100,
                bps: 16,
            };
    
            assert_eq!(format_chunk.byte_rate(), 176400);
        }
    
        #[test]
        fn test_block_align() {
            let format_chunk = PCMWaveFormatChunk {
                num_channels: 2,
                samp_rate: 44100,
                bps: 16,
            };
    
            assert_eq!(format_chunk.block_align(), 4);
        }
    }

    #[cfg(test)]
    mod endianess_detector {
        use super::*;
    
        #[test]
        fn test_endianess() {
            let wave_info = PCMWaveInfo {
                riff_header: RiffChunk {
                    file_size: 1234,
                    is_big_endian: false,
                },
                fmt_header: PCMWaveFormatChunk {
                    num_channels: 2,
                    samp_rate: 44100,
                    bps: 16,
                },
                data_chunks: Vec::new(),
            };
    
            let expected_endianess = false;
            assert_eq!(wave_info.riff_header.is_big_endian, expected_endianess);
        }
    }

    #[cfg(test)]
    mod other_error_handling {
        use super::*;
    
        #[test]
        fn test_display() {
            let error = WaveReaderError::NotRiffError;
            assert_eq!(format!("{}", error), "Not a valid RIFF header");
        }
    
        #[test]
        fn test_from_io_error() {
            let io_error = io::Error::new(io::ErrorKind::Other, "Test error");
            let wave_reader_error: WaveReaderError = io_error.into();
            assert_eq!(wave_reader_error, WaveReaderError::ReadError);
        }
    }
}