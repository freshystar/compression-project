pub struct LZ77 {
    window_size: usize,
}

impl LZ77 {
    pub fn new(window_size: usize) -> Self {
        LZ77 { window_size }
    }

    pub fn compress(&self, data: &[u8]) -> Vec<(usize, usize, u8)> {
        let mut compressed = Vec::new();
        let mut i = 0;

        while i < data.len() {
            let mut max_len = 0;
            let mut max_dist = 0;

            for j in 0..self.window_size.min(i) {
                let len = LZ77::match_len(&data[i..], &data[i - j - 1..i]);

                if len > max_len {
                    max_len = len;
                    max_dist = j + 1;
                }
            }
            if max_len > 0 {
                compressed.push((max_dist, max_len, data[i + max_len]));
                i += max_len + 1;
            } else {
                compressed.push((0, 0, data[i]));
                i += 1;
            }
        }
        println!("{:?}", compressed);

        compressed
    }

    fn match_len(data1: &[u8], data2: &[u8]) -> usize {
        data1
            .iter()
            .zip(data2.iter())
            .take_while(|&(a, b)| a == b)
            .count()
    }

    pub fn decompress(&self, compressed_data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();

        for chunk in compressed_data.chunks(6) {
            if chunk.len() < 6 {
                continue;
            }
            let dist = u16::from_le_bytes([chunk[0], chunk[1]]) as usize;
            let len = u16::from_le_bytes([chunk[2], chunk[3]]) as usize;
            let next_byte = chunk[4];
            compressed.push((dist, len, next_byte));
        }

        let mut decompressed = Vec::new();
        for &(dist, len, next_byte) in &compressed {
            if len > 0 {
                let start = decompressed.len().saturating_sub(dist);

                for i in 0..len {
                    if start + i < decompressed.len() {
                        decompressed.push(decompressed[start + i]);
                    } else {
                        break;
                    }
                }
                decompressed.push(next_byte);
            } else {
                decompressed.push(next_byte);
            }
        }
        decompressed
    }
}
