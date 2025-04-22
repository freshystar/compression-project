# 🗜 Compression Tool

This is a compression and decompression tool that has been implemented both in rust and javascript using the `RLE` and `LZ77` algorithms.

- `RLE`: stands for **Run-Length Encoding** and is a simple compression algorithm that replaces sequences of identical characters with a single character and a count of the number of times it appears in the sequence.

- `LZ77`: is a compression algorithm that finds repeated patterns in the input data and replaces them with a reference to the previous occurrence.

## 📚Usage
- Clone this repository and do:
 **Rust Implementation**
 ```rs
 cargo build
 cargo run <operation(compression/decompression)> <input_file> <output_file> <algorithm(--RLE/--LZ77)>
 ```
- To run test, you can do:
```rs
cargo test
```

 **Javascript Implementation**
 ```js
 node index.js <operation(compression/decompression)> <input_file> <output_file> <algorithm(--RLE/--LZ77)>
 ```
 - To run test, you can do:
 ```js
 npm install mocha
 npx mocha test
```

 ## 🔖Example 
 **Rust Implementation**
 ```rs
 cargo run compression input.txt output.rle --RLE
 ```
 You can use the same logic for the **Js** implementation.

 ## 📋Requirements
 **Rust Implementation**
 - Cargo v1.84 or latest

 **Javascript Implementation**
 - Node.js v18 or latest

 

