class RLE {
    static compress(data) {
        let compressed = '';
        let count = 1;

        for (let i = 1; i < data.length; i++) {
            if (data[i] === data[i - 1]) {
                count++;
            } else {
                compressed += count.toString() + data[i + 1];
                count = 1;
            }
        }
        compressed += count.toString() + data[data.length - 1];
        return compressed;
    }

    static decompress(data) {
        let decompressed = '';

        for (let i = 0; i < data.length; i += 2) {
            let count = parseInt(data[i]);
            let char = data[i + 1];
            decompressed += char.repeat(count);
        }
        return decompressed;
    }
}

export default RLE;