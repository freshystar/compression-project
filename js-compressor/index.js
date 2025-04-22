import { readFile, writeFile } from './Handler/fileManager';
import Compressor from './Handler/compressor';
import Benchmark from './Handler/benchmark';

function main() {
    const argv = process.argv.slice(2);
    if (argv.length !== 4) {
        console.error('Usage: node main.js <operation> <input_file> <output_file> <algorithm>');
        process.exit(1);
    }

    const operation = argv[0];
    const inputFile = argv[1];
    const outputFile = argv[2];
    const algorithm = argv[3];

    const data = readFile(inputFile);

    let result;
    let time;

    if (operation.toLowerCase === 'compress') {
        const startTime = Date.now();
        result = Compressor.compress(algorithm, data);
        time = Date.now() - startTime;
        writeFile(outputFile, result);
    } else if (operation.toLowerCase === 'decompress') {
        const startTime = Date.now();
        result = Compressor.decompress(algorithm, data);
        time = Date.now() - startTime;
        writeFile(outputFile, result);
    } else {
        console.error('Invalid operation');
        process.exit(1);
    }

    console.log(`Operation completed in ${time}ms`);
    Benchmark.logMemoryUsage();
}

main();
