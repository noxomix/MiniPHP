use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_left_brace(&mut self) {
        self.push_token(TokenTag::LeftBrace, self.byte_offset)
    }

    #[inline(always)]
    pub fn match_right_brace(&mut self) {
        self.push_token(TokenTag::RightBrace, self.byte_offset)
    }
}