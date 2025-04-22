mod compress_rle;
mod compressor_lz;
mod helper;
mod test;

use std::env;

use compress_rle::compressor::CompressorRle;
use compressor_lz::compress::Compressor;
use helper::helper;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        helper();
        return;
    }
    let window_size = 1024;

    let lz77_compress = Compressor::new(window_size);
    let operation = &args[1].to_lowercase();
    let input_file = &args[2];
    let output_file = &args[3];
    let algrithm = &args[4].to_uppercase();

    match operation.as_str() {
        "compression" => match algrithm.as_str() {
            "--RLE" => match CompressorRle::compress(input_file, output_file) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            },
            "--LZ77" => match lz77_compress.compress_file(input_file, output_file) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            },
            _ => println!("Unsupported algorithm"),
        },
        "decompression" => match algrithm.as_str() {
            "--RLE" => match CompressorRle::decompress(input_file, output_file) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            },
            "--LZ77" => match lz77_compress.decompress_file(input_file, output_file) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            },
            _ => println!("Unsupported algorithm!"),
        },
        _ => println!("Unsupported operation"),
    }
}

// fn main() -> std::io::Result<()> {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 4 {
//         println!(
//             "Usage: {} <input_file> <output_file> <compress/decompress>",
//             args[0]
//         );
//         return Ok(());
//     }

//     let input_file = &args[1];
//     let output_file = &args[2];
//     let action = &args[3].to_lowercase();

//     let window_size = 1024;
//     match action.as_str() {
//         "compress" => {
//             let compressor = Compressor::new(window_size);
//             compressor.compress_file(input_file, output_file)?;
//         }
//         "decompress" => {
//             let decompressor = Decompressor::new(window_size);
//             decompressor.decompress_file(input_file, output_file)?;
//         }

//         _ => {
//             println!("Invalid action. Please use 'compress' or 'decompress' .");
//         }
//     }
//     Ok(())
// }
