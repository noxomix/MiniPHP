use std::process::exit;
use crate::common::reader::{Reader};

mod lexer;
mod common;

fn main() {
    let file_path = "./php_files/test.php";
    start(file_path)
}

fn start(file_path: &str) {
    let bytes = Reader::read_file(file_path);
    if let Ok(bytes) = bytes {
        println!("{:?}", bytes.len());
    } else {
        println!("Error reading file - file not found or could not be read.");
        exit(2);
    }
}
