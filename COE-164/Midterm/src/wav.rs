// use core::fmt;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::error;
use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

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
    ReadError1,
    ReadError2,
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
        // Open the File
        let mut file = File::open(Path::new(file_path)).map_err(|_| WaveReaderError::ReadError1)?;

        // 1. read_riff header
        let riff_chunk = Self::read_riff_chunk(&mut file)?;
        // 2. read_fmt format chunk
        let fmt_chunk = Self::read_fmt_chunk(&mut file)?;
        // 3. read_data_chunk
        // The data chunk appears immediately after the format
        // The file handle should point to the start of a data chunk
        let start_pos = file.stream_position().map_err(|_| WaveReaderError::ReadError2)?;
        let data_chunk = Self::read_data_chunk(start_pos, &fmt_chunk, file)?;

        // Return the PCMWaveInfo object
        Ok(PCMWaveInfo {
            riff_header:riff_chunk,
            fmt_header: fmt_chunk,
            data_chunks: vec![data_chunk],
        })
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
        let mut riff_header = [0u8; 12];
        fh.read_exact(&mut riff_header).map_err(|_| WaveReaderError::ReadError)?;
        if &riff_header[0..4] != [0x52, 0x49, 0x46, 0x46] { // 'RIFF'
            return Err(WaveReaderError::NotRiffError);
        } 
        if &riff_header[8..12] != [0x57, 0x41, 0x56, 0x45] { // 'WAVE'
            return Err(WaveReaderError::NotRiffError);
        }

        let file_size = LittleEndian::read_u32(&riff_header[4..8]);
        let is_big_endian = &riff_header[0..4] == [0x52, 0x49, 0x46, 0x58];   // RIFX

        Ok(RiffChunk {
            file_size,
            is_big_endian, 
        })
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
        let mut fmt_header = [0u8; 24];
        fh.read_exact(&mut fmt_header).map_err(|_| WaveReaderError::ReadError)?;
        if &fmt_header[0..4] != [0x66, 0x6d, 0x74, 0x20] { // 'fmt '
            return Err(WaveReaderError::ChunkTypeError);
        }
        let fsub_chunk_size = LittleEndian::read_u32(&fmt_header[4..8]);    // FSubchunkSize
        if fsub_chunk_size != 16 {  // FSubchunkSize = 16 for PCM
            return Err(WaveReaderError::DataAlignmentError);
        }
        let audio_fmt = LittleEndian::read_u16(&fmt_header[8..10]);         // AudioFmt
        if audio_fmt != 0x01 {  // audio format = 1 for PCM
            return Err(WaveReaderError::NotPCMError);
        }

        let num_channels = LittleEndian::read_u16(&fmt_header[10..12]);     // NumChs
        let samp_rate = LittleEndian::read_u32(&fmt_header[12..16]);         // SampleRate
        let byte_rate = LittleEndian::read_u32(&fmt_header[16..20]);        // ByteRate
        let block_align = LittleEndian::read_u16(&fmt_header[20..22]);      // BlockAlign
        let bps = LittleEndian::read_u16(&fmt_header[22..24]);              // BitDepth

        if byte_rate != samp_rate * (num_channels as u32) * (bps as u32) / 8 {
            return Err(WaveReaderError::DataAlignmentError);
        }
        if block_align != (num_channels as u16) * (bps as u16) / 8 {
            return Err(WaveReaderError::DataAlignmentError);
        }

        Ok(PCMWaveFormatChunk {
            num_channels,
            samp_rate,
            bps,
        })
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
        fh.seek(SeekFrom::Start(start_pos)).map_err(|_| WaveReaderError::ReadError)?;

        // Read data chunk header
        let mut fmt_header = [0u8; 8];
        fh.read_exact(&mut fmt_header).map_err(|_| WaveReaderError::ReadError)?;
        if &fmt_header[0..4] != [0x64, 0x61, 0x74, 0x61] { // 'data'
            return Err(WaveReaderError::ChunkTypeError);
        }

        let dsub_chunk_size = LittleEndian::read_u32(&fmt_header[4..8]);        // DSubChunkSize

        // We use the file handle as is and wrap it in a BufReader.
        Ok(PCMWaveDataChunk {
            size_bytes: dsub_chunk_size,
            format: *fmt_info,
            data_buf: io::BufReader::new(fh),
        })
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
            WaveReaderError::ReadError1 => "Error reading from file 1",
            WaveReaderError::ReadError2 => "Error reading from file 2",
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
    /// Get or calculate the byte rate of this PCM WAV file
    fn byte_rate(&self) -> u32 {
        self.samp_rate * (self.num_channels as u32) * (self.bps as u32) / 8
    }

    /// Get or calculate the block alignment of this PCM WAV file
    /// 
    /// The *block alignment* is the size of one *inter-channel* sample
    /// in bytes. An *inter-channel sample* is a sample with all of its
    /// channels collated together.
    fn block_align(&self) -> u16 {
        (self.num_channels as u16) * (self.bps as u16) / 8
    }
}

impl Iterator for PCMWaveDataChunk {
    type Item = Vec <i16>;

    fn next(&mut self) -> Option <Self::Item> {
        let mut sample = Vec::new();
        for _ in 0..self.format.num_channels {
            match self.data_buf.read_i16::<LittleEndian>() {
                Ok(val) => sample.push(val),
                Err(_) => return None,
            }
        }
        Some(sample)
    }
}

impl Iterator for PCMWaveDataChunkWindow {
    type Item = Vec <Vec <i16>>;

    fn next(&mut self) -> Option <Self::Item> {
        // todo!("Return self.chunk_size amount of inter-channel samples at a time")
        let mut batch = Vec::new();
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
    /// Consume a data chunk and get an iterator
    /// 
    /// This method is used to get a *single* inter-channel
    /// sample from a data chunk.
    pub fn chunks_byte_rate(self) -> PCMWaveDataChunkWindow {
        PCMWaveDataChunkWindow {
            chunk_size: 1,
            data_chunk: self,
        }
    }

    /// Consume a data chunk and get an iterator
    /// 
    /// This method is used to get a `chunk_size` amount of inter-channel
    /// samples. For example, if there are two channels and the chunk size is
    /// 44100 corresponding to a sample rate of 44100 Hz, then the iterator will
    /// return a `Vec` of size *at most* 44100 with each element as another `Vec`
    /// of size 2.
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

    mod read_data_fmt {
        // TODO
    }
}