use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_pivot(&mut self) {
        self.push_token(TokenTag::BitXor, self.byte_offset);
    }
}