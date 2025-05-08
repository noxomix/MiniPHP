use std::process::exit;
use std::sync::Arc;
use crate::common::reader::{Reader};
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::Token;

mod lexer;
mod common;

fn main() {
    let file_path = "./php_files/test.php";
    let bytes = read(file_path);
    let tokens = lex(bytes);
    println!("Number of Tokens found: {:?}", tokens.len());
    for token in tokens {
        println!("{:?}", token);
    }

}

fn read(file_path: &str) -> Arc<[u8]> {
    let bytes = Reader::read_file(file_path);
    if let Ok(bytes) = bytes {
        return bytes
    } else {
        println!("Error reading file: {}", file_path);
        exit(2);
    }
}

fn lex(bytes: Arc<[u8]>) -> Vec<Token> {
    let mut lexer = Lexer::new(bytes);
    let tokens = lexer.tokenize();
    tokens
}