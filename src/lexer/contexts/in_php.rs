/*
    Default handler which will parse PHP context.
*/
use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext};

impl Lexer {
    pub fn handle_php(&mut self) {
        let mut current = self.current();
        while let Some(c) = current {
            match c {
                b'/' => {
                    match self.look() {
                        Some(b'/') => {
                            self.context.push(LexerContext::InCommentLine);
                            return;
                        },
                        Some(b'*') => {
                            self.context.push(LexerContext::InCommentBlock);
                            return;
                        },
                        _ => {
                            //'/' geteilt-durch zeichen
                        }
                    }
                }
                b'"' | b'\'' => {
                    self.context.push(LexerContext::InString);
                    return;
                },
                _ => {},
            }
            current = self.consume();
        }
    }
}