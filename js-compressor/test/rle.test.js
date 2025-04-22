import { strictEqual } from 'assert';
import { compress, decompress } from '../rle';

describe('RLE Compression', () => {
    it('should compress and decompress correctly', () => {
        const input = Buffer.from('AAABBBCCCCCDDDDE');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        strictEqual(decompressed.toString(), input.toString());
    });
});