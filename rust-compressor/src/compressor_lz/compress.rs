use super::{file_io::FileIo, lz77::LZ77};

pub struct Compressor {
    lz77: LZ77,
}

impl Compressor {
    pub fn new(window_size: usize) -> Self {
        Compressor {
            lz77: LZ77::new(window_size),
        }
    }

    pub fn compress_file(&self, input_file: &str, output_file: &str) -> std::io::Result<()> {
        let data = FileIo::read_file(input_file)?;
        let compressed = self.lz77.compress(&data);
        let mut compressed_bytes = Vec::new();

        for &(dist, _len, next_byte) in &compressed {
            compressed_bytes.extend_from_slice(&dist.to_le_bytes());
            compressed_bytes.push(next_byte);
        }

        FileIo::write_file(output_file, &compressed_bytes)?;
        println!("✅ File was compressed successfully 💯");

        Ok(())
    }

    pub fn decompress_file(&self, input_file: &str, output_file: &str) -> std::io::Result<()> {
        let data = FileIo::read_file(input_file)?;
        let decompressed = self.lz77.decompress(&data);
        FileIo::write_file(output_file, &decompressed)?;

        println!("✅ File was decompressed successfully 💯");
        Ok(())
    }

}
