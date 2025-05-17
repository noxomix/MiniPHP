use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_double_dot_symbol(&mut self) {
        let start = self.byte_offset;
        match self.look() {
            Some(b':') => {
                self.next();
                self.push_token(TokenTag::DoubleColon, start);
            }
            _ => self.push_token(TokenTag::Colon, start),
        }
    }
}