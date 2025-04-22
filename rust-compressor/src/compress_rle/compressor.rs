use std::time::Instant;

use super::{file_handler::FileHandler, rle::RLE};

pub struct CompressorRle;

impl CompressorRle {
    pub fn compress(input_file: &str, output_file: &str) -> Result<(), String> {
        let data = match FileHandler::read_file(input_file) {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        let start = Instant::now();
        let result = RLE::compress(&data);
        let duration = start.elapsed();

        println!("🕑 Operation took {:?}", duration);

        match FileHandler::write_file(output_file, &result) {
            Ok(_) => {
                println!("✅ Operation completed successfully 💯");
                println!(
                    "Compression ratio: {:.2}%",
                    (result.len() as f64 / data.len() as f64) * 100.0
                );
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn decompress(input_file: &str, output_file: &str) -> Result<(), String> {
        let data = match FileHandler::read_file(input_file) {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        let start = Instant::now();
        let result = RLE::decompress(&data);
        let duration = start.elapsed();

        println!("🕑 Operation took {:?}", duration);

        match FileHandler::write_file(output_file, &result) {
            Ok(_) => {
                println!("✅ Operation completed successfully 💯");
                println!(
                    "Decompression ratio: {:.2}%",
                    (data.len() as f64 / result.len() as f64) * 100.0
                );
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
