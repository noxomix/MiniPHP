use std::process::exit;
use std::sync::Arc;
use std::time::Instant;
use crate::common::reader::{Reader};
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::Token;

mod lexer;
mod common;

fn main() {
    let file_path = "./php_files/test.php";
    let bytes = read(file_path);
    let lexing_start = Instant::now();
    let tokens = lex(bytes.clone());
    let lexing_duration = lexing_start.elapsed();
    /*for token in &tokens {
        println!("{:?}", &token.tag.as_ref());
    }*/

    /*{
        let mut free = false;
        for token in &tokens {
            let tag_name = token.tag.as_ref(); // aus strum
            let slice = &bytes[token.start_position..=token.end_position];
            let source = std::str::from_utf8(slice).unwrap_or("<invalid utf8>");

            if tag_name.to_ascii_lowercase().contains("comment")  {
                println!("{tag_name} =>");
                println!("\t{:?}", source);
                println!("");
                free = true;
            } else if free {
                println!("{tag_name} =>");
                println!("\t{:?}", source);
                println!("");
                free = false;
            }
        }
    }*/

    {
        for token in &tokens {
            let tag_name = token.tag.as_ref(); // aus strum
            let slice = &bytes[token.start_position..=token.end_position];
            let source = std::str::from_utf8(slice).unwrap_or("<invalid utf8>");
            //println!("{tag_name} =>");
            //println!("\t{:?}", source);
            //println!("");
        }
    }


    println!("Number of Tokens found: {:?}", tokens.len());
    println!("Lexing time: {:?}", lexing_duration);
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