use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_whitespace(&mut self) {
        let start = self.byte_offset;

        while matches!(self.look(), Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n')) {
            self.consume();
        }

        if self.byte_offset >= start {
            self.push_token(TokenTag::Whitespace, start);
            //println!("{:?}", value); // ← Debug ggf. entfernen
        }
    }
}