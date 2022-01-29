use std::process;

mod compress;
mod config;
mod decompress;

use config::{Config, Mode};

fn main() {
    let config = Config::new(std::env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match config.mode {
        Mode::Compress => {
            if let Err(e) = compress::run(&config) {
                eprintln!("Compression error: {}", e);
            }
        }
        Mode::Decompress => {
            if let Err(e) = decompress::run(&config) {
                eprintln!("Decompression error: {}", e);
            }
        }
    }
}
