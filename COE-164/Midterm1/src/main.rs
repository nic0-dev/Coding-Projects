pub mod wav;
pub mod flac;

// use std::fs::File;

fn main() {
    let file_path = String::from("sample_audio/music_b.wav");
    // let wav_file = File::open(&file_path);
    
    let wave_obj = wav::WaveReader::open_pcm(&file_path);

    if let Err(e) = wave_obj {
        println!("Error while reading file {}: {}", file_path, e);
    }
    else if let Ok(wave_obj_safe) = wave_obj {
        println!("File {} opened successfully!", file_path);
        println!("{}", wave_obj_safe);

        for each_chunk in wave_obj_safe.data_chunks {
            for (i, each_sample) in each_chunk.chunks_byte_rate().enumerate() {
                println!("each sample index {} len {}", i, each_sample.len());
            }
        }
        // 1. Apply LPC
        let lpc_order = 2;
        let residuals, lpc_coeffs = apply_lpc(wave_obj_safe.data_chunks, lpc_order);
        // 2. Quantize LPC coeffs
        let precision_val = 14;
        let quantize_coeffs, qlp_shift = quantize_coeffs(lpc_coeffs, precision_val);
        // 3. Rice Encoding
        let encoded_data = apply_rice_encoding(residuals);
        write_flac_metadata(flac_file, pcm_wave_info.fmt_header);

        write_flac_audio(flac_file, encoded_data);
    }

    // let flac_writer = FlacWriter::new(&wave_obj);
    // let output_path = "sample_audio/music_b.flac";
    // let mut output_file = File::create(output_path)?;
    // flac_writer.write_to(&mut output_file);

    // println!("File converted to FLAC!");
}
