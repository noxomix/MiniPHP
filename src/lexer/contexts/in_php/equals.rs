use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_equals(&mut self) {
        let start = self.byte_offset;
        match self.look() {
            Some(b'>') => {
                self.consume();
                self.push_token(TokenTag::FatArrow, start);
            }
            Some(b'=') => {
                self.consume();
                if self.look() == Some(b'=') {
                    self.consume();
                    self.push_token(TokenTag::IsIdentical, start);
                } else {
                    self.push_token(TokenTag::IsEqual, start);
                }
            }
            _ => self.push_token(TokenTag::Assign, start),
        }
    }
}