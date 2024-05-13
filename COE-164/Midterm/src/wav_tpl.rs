use core::fmt;
use std::fs::File;
use std::path::Path;
use std::error;
use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{ByteOrder, LittleEndian};

/// Represents a PCM WAV file
pub struct PCMWaveInfo {
    pub riff_header: RiffChunk,
    pub fmt_header: PCMWaveFormatChunk,
    pub data_chunks: Vec <PCMWaveDataChunk>,
}

/// Represents a RIFF chnk from a WAV file
/// 
/// The RIFF chunk is the first 12 bytes of a WAV file.
pub struct RiffChunk {
    pub file_size: u32,
    pub is_big_endian: bool,
}

/// Represents a format chunk from a WAV file
/// 
/// A format chunk in a WAV file starts with a magic string
/// `fmt_` where `_` is a space (0x20 in hex) and then followed by
/// 20 bytes of metadata denoting information about the audio file
/// itself such as the sample and bit rates.
#[derive(Clone, Copy)]
pub struct PCMWaveFormatChunk {
    pub num_channels: u16,
    pub samp_rate: u32,
    pub bps: u16,
}

/// Represents a data chunk from a WAV file
/// 
/// A data chunk in a WAV file starts with a magic string `data` and then
/// followed by the number of samples that follow and then finally the
/// audio data samples themselves.
pub struct PCMWaveDataChunk {
    pub size_bytes: u32,
    pub format: PCMWaveFormatChunk,
    pub data_buf: io::BufReader<File>,
}

/// Represents an iterator to a data chunk from a WAV file
/// 
/// This struct is not instantiated by itself and is generated
/// by calling the methods `PCMWaveDataChunk::chunks_byte_rate()`
/// and `PCMWaveDataChunk::chunks()`.
pub struct PCMWaveDataChunkWindow {
    chunk_size: usize,
    data_chunk: PCMWaveDataChunk
}

/// Represents a WAV reader
pub struct WaveReader;

/// Represents an error in the WAV reader
#[derive(Debug)]
pub enum WaveReaderError {
    NotRiffError,
    NotWaveError,
    NotPCMError,
    ChunkTypeError,
    DataAlignmentError,
    ReadError,
}

impl WaveReader {
    /// Open a PCM WAV file
    /// 
    /// The WAV file located at `file_path` will be represented as a `PCMWaveInfo`
    /// struct for further processing.
    /// 
    /// # Errors
    /// Returns a `WaveReaderError` with the appropriate error if something
    /// happens.
    pub fn open_pcm(file_path: &str) -> Result <PCMWaveInfo, WaveReaderError> {
        todo!();
    }

    /// Read the RIFF header from a PCM WAV file
    /// 
    /// The RIFF header is the first twelve bytes of a PCM WAV
    /// file of the format `<RIFF_magic_str:4B><file_size:4B><RIFF_type_magic_str:4B>`.
    /// Note that the file handle `fh` should point to the very start of the file.
    /// 
    /// # Errors
    /// Returns a `WaveReaderError` with the appropriate error if something
    /// happens. This includes file read errors and format errors.
    fn read_riff_chunk(fh: &mut File) -> Result <RiffChunk, WaveReaderError> {
        todo!();
    }

    /// Read the format chunk from a PCM WAV file
    /// 
    /// The format chunk usually appears immediately after the RIFF header and consists of 24 bytes of metadata.
    /// Note that the file handle `fh` should point to the start of a format chunk.
    /// 
    /// # Errors
    /// Returns a `WaveReaderError` with the appropriate error if something
    /// happens. This includes file read errors and format errors.
    fn read_fmt_chunk(fh: &mut File) -> Result <PCMWaveFormatChunk, WaveReaderError> {
        todo!();
    }

    /// Read the data chunk from a PCM WAV file
    /// 
    /// The data chunk usually appears immediately after the format
    /// chunk and contains the samples of the audio itself. Note that
    /// a file can contain multiple data chunks, and it is possible that this
    /// method should be called more than once to completely read the file.
    /// Note that the file handle `fh` should point to the start of a data chunk.
    /// 
    /// # Errors
    /// Returns a `WaveReaderError` with the appropriate error if something
    /// happens. This includes file read errors and format errors.
    fn read_data_chunk(start_pos: u64, fmt_info: &PCMWaveFormatChunk, mut fh: File) -> Result <PCMWaveDataChunk, WaveReaderError> {
        todo!();
    }
}

impl error::Error for WaveReaderError {}

impl fmt::Display for WaveReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl From <io::Error> for WaveReaderError {
    fn from(_: io::Error) -> Self {
        todo!("Convert an I/O error into a WaveReaderError")
    }
}

impl fmt::Display for PCMWaveInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Display a PCMWaveInfo struct in this format: WAVE File <FileSize> bytes, <BitDepth>-bit <NumChannels> channels, <SampleRate>Hz, <NumDataChunks> data chunks")
    }
}

impl PCMWaveFormatChunk {
    /// Get or calculate the byte rate of this PCM WAV file
    fn byte_rate(&self) -> u32 {
        todo!();
    }

    /// Get or calculate the block alignment of this PCM WAV file
    /// 
    /// The *block alignment* is the size of one *inter-channel* sample
    /// in bytes. An *inter-channel sample* is a sample with all of its
    /// channels collated together.
    fn block_align(&self) -> u16 {
        todo!();
    }
}

impl Iterator for PCMWaveDataChunk {
    type Item = Vec <i64>;

    fn next(&mut self) -> Option <Self::Item> {
        todo!("Return one inter-channel sample at a time")
    }
}

impl Iterator for PCMWaveDataChunkWindow {
    type Item = Vec <Vec <i64>>;

    fn next(&mut self) -> Option <Self::Item> {
        todo!("Return self.chunk_size amount of inter-channel samples at a time")
    }
}

impl PCMWaveDataChunk {
    /// Consume a data chunk and get an iterator
    /// 
    /// This method is used to get a *single* inter-channel
    /// sample from a data chunk.
    pub fn chunks_byte_rate(self) -> PCMWaveDataChunkWindow {
        todo!();
    }

    /// Consume a data chunk and get an iterator
    /// 
    /// This method is used to get a `chunk_size` amount of inter-channel
    /// samples. For example, if there are two channels and the chunk size is
    /// 44100 corresponding to a sample rate of 44100 Hz, then the iterator will
    /// return a `Vec` of size *at most* 44100 with each element as another `Vec`
    /// of size 2.
    pub fn chunks(self, chunk_size: usize) -> PCMWaveDataChunkWindow {
        todo!();
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

    mod read_data_fmt {
        // TODO
    }
}