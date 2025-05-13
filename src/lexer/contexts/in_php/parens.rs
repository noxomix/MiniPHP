use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_left_paren(&mut self) {
        self.push_token(TokenTag::LeftParen, self.byte_offset)
    }

    #[inline(always)]
    pub fn match_right_paren(&mut self) {
        self.push_token(TokenTag::RightParen, self.byte_offset)
    }
}