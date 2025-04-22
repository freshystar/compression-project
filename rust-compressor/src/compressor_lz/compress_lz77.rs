pub struct Lz77;

impl Lz77 {
    pub fn compress(data: &[u8]) -> String {
        let mut compressed = String::new();
        let mut i = 0;
        while i < data.len() {
            let (offset, length) = Self::find_longest_match(data, i);
           
            if length > 0 {
                if i + length >= data.len() {
                    compressed.push_str(&format!("<{},{}>", offset, length));
                    i += length;
                } else {
                    compressed.push_str(&format!("<{},{},{}>", offset, length, data[i + length] as char));
                    i += length + 1;
                }
            } else {
                compressed.push_str(&format!("{}", data[i] as char));
                i += 1;
            }
        }
        compressed
    }

   pub  fn decompress(compressed: &str) -> Vec<u8> {
        let mut decompressed = Vec::new();
        let mut i = 0;
        while i < compressed.len() {
            if compressed.chars().nth(i).unwrap() == '<' {
                let j = compressed.find('>').unwrap();
                let parts: Vec<&str> = compressed[i + 1..j].split(',').collect();
                if parts.len() == 2 {
                    let offset: usize = parts[0].parse().unwrap();
                    let length: usize = parts[1].parse().unwrap();
                    let start = decompressed.len() - offset;
                    for k in 0..length {
                        decompressed.push(decompressed[start + k]);
                    }
                } else {
                    let offset: usize = parts[0].parse().unwrap();
                    let length: usize = parts[1].parse().unwrap();
                    let start = decompressed.len() - offset;
                    for k in 0..length {
                        decompressed.push(decompressed[start + k]);
                    }
                    decompressed.push(parts[2].chars().nth(0).unwrap() as u8);
                }
                i = j + 1;
            } else {
                decompressed.push(compressed.chars().nth(i).unwrap() as u8);
                i += 1;
            }
        }
        decompressed
    }

    fn find_longest_match(data: &[u8], pos: usize) -> (usize, usize) {
        let mut max_len = 0;
        let mut max_offset = 0;
        for i in 0..pos {
            let mut j = 0;
            while pos + j < data.len() && i + j < pos && data[pos + j] == data[i + j] {
                j += 1;
            }
            if j > max_len {
                max_len = j;
                max_offset = pos - i;
            }
        }
        (max_offset, max_len)
    }
}