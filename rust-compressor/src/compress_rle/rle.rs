pub struct RLE;

impl RLE {
    pub fn compress(data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();
        let mut count = 1;

        for i in 1..data.len() {
            if data[i] == data[i - 1] {
                count += 1;
            } else {
                compressed.push(count);
                compressed.push(data[i - 1]);
                count = 1;
            }
        }
        compressed.push(count);
        compressed.push(*data.last().unwrap_or(&0));
        compressed
    }

    pub fn decompress(data: &[u8]) -> Vec<u8> {
        let mut decompressed = Vec::new();

        for i in (0..data.len()).step_by(2) {
            if i + 1 >= data.len() {
                break;
            }
            let count = data[i] as usize;
            let byte = data[i + 1];
            decompressed.extend(std::iter::repeat(byte).take(count));
        }
        decompressed
    }
}
