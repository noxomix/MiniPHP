use crate::lexer::lexer::{BytesOperation, Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn handle_comment_line(&mut self) {
        let start_position = self.position.clone();
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
                    /*self.push_token(TokenTag::Comment {
                        value: unsafe { self.string_from_range_unchecked(start_position.byte_offset, self.position.byte_offset) },
                        multiline: false}, start_position);*/
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