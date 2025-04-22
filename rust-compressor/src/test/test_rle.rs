#[cfg(test)]
mod tests {

    use crate::compress_rle::compress::{compress_rle, decompress_rle};

    use super::*;
    

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress_rle(input);
        let decompressed = decompress_rle(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
