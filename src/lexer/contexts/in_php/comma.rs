use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_comma(&mut self) {
        self.push_token(TokenTag::Comma, self.byte_offset)
    }
}