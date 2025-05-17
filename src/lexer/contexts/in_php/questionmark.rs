use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_question_symbol(&mut self) -> bool {
        let start = self.byte_offset;
        match self.look() {
            Some(b'>') => {
                self.next(); // consume '>'
                self.push_token(TokenTag::PhpCloseTag, start);
                self.context.pop();
                return true
            }
            Some(b'?') => {
                self.next(); // consume 2nd '?'
                if self.look() == Some(b'=') {
                    self.next();
                    self.push_token(TokenTag::NullCoalesceAssign, start);
                } else {
                    self.push_token(TokenTag::NullCoalesce, start);
                }
            }
            _ => {
                self.push_token(TokenTag::TernaryQuestion, start);
            }
        }
        false
    }
}