use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn handle_comment_line(&mut self) {
        let start_offset = self.byte_offset;
        if self.current() == Some(b'#') {
            self.consume(); //'#'
        } else {
            self.consume_n(2); //'//'
        }

        let mut current = self.current();
        loop {
            match current {
                Some(b'\n') | Some(b'\r') | None => {
                    if let Some(b'\n') = self.look() {
                        self.consume();
                    }
                    self.push_token(TokenTag::Comment {
                        value: unsafe { self.strquick(start_offset, self.byte_offset) },
                        multiline: false}, start_offset);
                    self.context.pop();
                    return
                },
                _ => {
                    current = self.consume();
                }
            }
        }
    }
}