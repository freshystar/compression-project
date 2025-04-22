use std::{
    fs::File,
    io::{Read, Write},
};

pub struct FileIo;

impl FileIo {
    pub fn read_file(file_path: &str) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }

    pub fn write_file(file_path: &str, data: &[u8]) -> std::io::Result<()> {
        let mut output = File::create(file_path)?;
        output.write_all(data)?;
        Ok(())
    }
}
