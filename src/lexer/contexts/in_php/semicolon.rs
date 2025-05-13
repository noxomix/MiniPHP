use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_semicolon(&mut self) {
        self.push_token(TokenTag::Semicolon, self.byte_offset)
    }
}