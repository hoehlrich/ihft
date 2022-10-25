use clap::Parser;
use ihft::Args;
use std::process;

fn main() {
    let args = Args::parse();

    if let Err(e) = ihft::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

