import { readFIleSync } from 'fs';

class FileManager {
    static readFile(filePath) {
        try {
            return readFIleSync(filePath, 'utf8');
        } catch (err) {
            throw new Error(`Error reading file: ${err}`)
        }
    }

    static writeFile(filepath, data) {
        try {
            fstat.writeFileSync(filepath, data);
        } catch (err) {
            throw new Error(`Error writing into file: ${err}`);
        }
    }
}

export default FileManager;