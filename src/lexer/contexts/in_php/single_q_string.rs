use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_single_q_string(&mut self) {
        let start_offset = self.byte_offset;
        let mut curr = self.next();
        loop {
            match curr {
                Some(b'\'') => {
                    self.push_token(TokenTag::StringLiteral {
                        //value: unsafe { self.strquick(start_offset+1, self.byte_offset-1) },
                        double_quoted: false
                    }, start_offset);
                    break;
                }
                Some(b'\\') => {
                    match self.look() {
                        Some(b'\'') | Some(b'\\') => {
                            self.next(); // skip escape
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            curr = self.next();
        }
    }
}