
pub fn helper() {
    println!("🚀  Welcome to Rust File Compressor! 🎊");
    println!("\n🚩Available commands:");
    println!("  compress <source_path> <output_path>  <algorithm(--RLE/--LZ77)>- Compress a file or folder");
    println!("  decompress <archive_path> <output_dir> <algorithm(--RLE/--LZ77)> - Extract compressed archive");
    println!("\n💡Examples:");
    println!("  compress /path/to/folder archive.bin RLE");
    println!("  decompress archive.bin /path/to/extract RLE");
}