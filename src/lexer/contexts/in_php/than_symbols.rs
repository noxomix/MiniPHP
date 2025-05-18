use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_less_than_symbol(&mut self) {
        let start = self.byte_offset;
        match self.peek() {
            Some(b'=') => {
                self.next();
                self.push_token(TokenTag::IsSmallerOrEqual, start);
            }
            Some(b'<') => {
                self.next();
                self.push_token(TokenTag::ShiftLeft, start);
            }
            _ => {
                self.push_token(TokenTag::IsSmaller, start);
            }
        }
    }

    #[inline(always)]
    pub fn match_greater_than_symbol(&mut self) {
        let start = self.byte_offset;
        match self.peek() {
            Some(b'=') => {
                self.next();
                self.push_token(TokenTag::IsGreaterOrEqual, start);
            }
            Some(b'>') => {
                self.next();
                self.push_token(TokenTag::ShiftRight, start);
            }
            _ => {
                self.push_token(TokenTag::IsGreater, start);
            }
        }
    }
}