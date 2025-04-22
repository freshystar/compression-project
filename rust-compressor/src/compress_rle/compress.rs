pub fn compress_rle(data: &[u8]) -> Vec<(u8, u32)> {
    let mut encoded = Vec::new();
    if data.is_empty() {
        return encoded;
    }

    let mut current_val = data[0];
    let mut current_count = 0;

    for &value in data {
        if value == current_val {
            current_count += 1;
        } else {
            encoded.push((current_val, current_count));
            current_val = value;
            current_count = 1;
        }
    }
    encoded.push((current_val, current_count));
    encoded
}


pub fn decompress_rle(encoded_data: &[(u8, u32)]) -> Vec<u8> {
    let mut decoded = Vec::new();
    for (value, count) in encoded_data {
        for _ in 0..*count {
            decoded.push(*value);
        }
    }
    decoded
}