use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn context_in_comment_block(&mut self) {
        let mut is_doc = false;
        let start_offset = self.byte_offset;
        self.next_n(2); //'/*'
        //println!("Handle {:?}", self.byte_offset);

        //is it doc comment?
        if let Some(b'*') = self.current() {
            is_doc = true;
            self.next();
        }

        let mut current = self.current();
        loop {
            match current {
                Some(b'*') => {
                    if let Some(b'/') = self.peek() {
                        self.next();
                        //multiline comment has ended
                        if is_doc {
                            self.push_token(TokenTag::DocComment{}, start_offset);
                            self.next();
                        } else {
                            self.push_token(TokenTag::Comment{multiline: true}, start_offset);
                            self.next();
                        }
                        self.context.pop();
                        return
                    }
                },
                _ => {}
            }
            current = self.next();
        }
    }
}