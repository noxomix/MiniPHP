use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_star_symbol(&mut self) {
        let start = self.byte_offset;
        match self.peek() {
            Some(b'=') => {
                self.next();
                self.push_token(TokenTag::MulAssign, start);
            }
            Some(b'*') => {
                self.next();
                if self.peek() == Some(b'=') {
                    self.next();
                    self.push_token(TokenTag::PowerAssign, start);
                } else {
                    self.push_token(TokenTag::Power, start);
                }
            }
            _ => {
                self.push_token(TokenTag::Multiply, start);
            }
        }
    }
}