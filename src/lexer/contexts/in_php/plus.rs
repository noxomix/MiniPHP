use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_plus_symbol(&mut self) {
        let start = self.byte_offset;
        match self.look() {
            Some(b'+') => {
                self.consume();
                self.push_token(TokenTag::Increment, start);
            }
            Some(b'=') => {
                self.consume();
                self.push_token(TokenTag::AddAssign, start);
            }
            _ => {
                self.push_token(TokenTag::Plus, start);
            }
        }
    }
}