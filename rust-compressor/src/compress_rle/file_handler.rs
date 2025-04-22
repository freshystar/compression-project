use std::{
    fs::File,
    io::{Read, Write},
};

pub struct FileHandler;

impl FileHandler {
    pub fn read_file(filename: &str) -> Result<Vec<u8>, String> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => return Err(format!("❌ Failed to open input file: {}", e)),
        };
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(_) => (),
            Err(e) => return Err(format!("❌ Failed to read input file: {}", e)),
        };

        if data.is_empty() {
            return Err("Input file is empty 🛑".to_string());
        }
        Ok(data)
    }

    pub fn write_file(filename: &str, data: &[u8]) -> Result<(), String> {
        let mut output = match File::create(filename) {
            Ok(file) => file,
            Err(e) => return Err(format!("❌ Failed to create output file: {}", e)),
        };

        match output.write_all(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("❌ Failed to write to output file: {}", e)),
        }
    }
}
