use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_left_bracket(&mut self) {
        self.push_token(TokenTag::LeftBracket, self.byte_offset)
    }

    #[inline(always)]
    pub fn match_right_bracket(&mut self) {
        self.push_token(TokenTag::RightBracket, self.byte_offset)
    }
}