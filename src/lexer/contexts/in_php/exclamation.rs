use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_exclamation(&mut self) {
        let start = self.byte_offset;
        if self.peek() == Some(b'=') {
            self.next();
            if self.peek() == Some(b'=') {
                self.next();
                self.push_token(TokenTag::IsNotIdentical, start);
            } else {
                self.push_token(TokenTag::IsNotEqual, start);
            }
        }
    }
}