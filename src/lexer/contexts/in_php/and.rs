use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_and_symbol(&mut self) {
        let start = self.byte_offset;
        if self.look() == Some(b'&') {
            self.next();
            self.push_token(TokenTag::LogicalAnd, start);
        } else {
            self.push_token(TokenTag::BitAnd, start);
        }
    }
}