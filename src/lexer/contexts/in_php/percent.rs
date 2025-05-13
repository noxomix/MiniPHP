use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_percent_symbol(&mut self) {
        let start = self.byte_offset;
        if self.look() == Some(b'=') {
            self.consume();
            self.push_token(TokenTag::ModAssign, start);
        } else {
            self.push_token(TokenTag::Modulo, start);
        }
    }
}