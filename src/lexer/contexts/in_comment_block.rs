use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn handle_comment_block(&mut self) {
        let mut is_doc = false;
        let start_position = self.position.clone();
        self.consume_n(2); //'/*'
        
        //is it doc comment?
        if let Some(b'*') = self.current() {
            is_doc = true;
            self.consume();
        }

        let mut current = self.current();
        loop {
            match current {
                Some(b'*') => {
                    if let Some(b'/') = self.look() {
                        self.consume_n(2);
                        //multiline comment has ended
                        let value = unsafe { self.strquick(start_position.byte_offset, self.position.byte_offset) };
                        if is_doc {
                            self.push_token(TokenTag::DocComment { value }, start_position.clone());
                        } else {
                            self.push_token(TokenTag::Comment { value, multiline: true }, start_position.clone());
                        }
                        self.context.pop();
                        return
                    }
                },
                _ => {}
            }
            current = self.consume();
        }
    }
}