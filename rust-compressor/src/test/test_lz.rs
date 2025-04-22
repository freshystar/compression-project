#[cfg(test)]
mod tests {
    use crate::compressor_lz::compress_lz77::Lz77;
    use super::*;

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
        let compressed = Lz77::compress(input);
        let decompressed = Lz77::decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
