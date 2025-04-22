import { compress as _compress } from '../rle-compressor/RLE';
import LZ77 from '../lz77-compressor/LZ77';

class Compressor {
    static compress(algorithm, data) {
        switch (algorithm) {
            case 'RLE':
                return compress(data);
            case 'LZ77':
                return LZ77.compress(data);
            default:
                throw new Error('Unsupported algorithm');
        }
    }

    static decompress(algorithm, data) {
        switch (algorithm) {
            case 'RLE':
                return decompress(data);
            case 'LZ77':
                return LZ77.decompress(data);
            default:
                throw new Error('Unsupported algorithm');
        }
    }
}

export default Compressor;