class LZ77 {
    static compress(data) {
        let compressed = '';
        let windowSize = 4096;
        let i = 0;
        while (i < data.length) {
            let match = this.findLongestMatch(data, i, windowSize);
            if (match.length > 0) {
                compressed += `<${match.offset}, ${match.length}, ${data[i + match.length]}>`;
                i += match.length + 1;
            } else {
                compressed += data[i];
                i++;
            }
        }
        return compressed;
    }

    static decompress(data) {
        let decompressed = '';
        let regex = /<(\d+),(\d+),([a-zA-Z0-9])>/g;
        let match;
        let i = 0;
        while (i < data.length) {
            match = regex.exec(data.substring(i));
            if (match) {
                let offset = parseInt(match[1]);
                let length = parseInt(match[2]);
                let char = match[3];
                let start = decompressed.length - offset;
                let substring = decompressed.substring(start, start + length);
                decompressed += substring + char;
                i += match[0].length + i;
            } else {
                decompressed += data[i];
                i++;
            }
        }
        return decompressed;
    }

    static findLongestMatch(data, position, windowSize) {
        let maxLength = 0;
        let maxOffset = 0;

        for (let i = Math.max(0, position - windowSize); i < position; i++) {
            let matchLength = 0;
            while (position + matchLength < data.length && i + matchLength < position && data[position + matchLength] === data[i + matchLength]) {
                matchLength++;
            }
            if (matchLength > maxLength) {
                maxLength = matchLength;
                maxOffset = position - i;
            }
        }
        return { offset: maxOffset, length: maxLength };
    }
}

export default LZ77;