use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn handle_string(&mut self) {
        let start_offset = self.byte_offset;
        let double_quoted = self.current() == Some(b'"');
        self.consume();

        let mut current = self.current();
        loop {
            match current {
                Some(b'\\') => {
                    //escape
                    self.consume(); //also insgesamt werden 2 zeichen konsumiert, also wird \" und \' gecovert
                }
                Some(b'"') => {
                    if double_quoted {
                        //end
                        self.push_token(TokenTag::StringLiteral {
                            //value: unsafe { self.strquick(start_offset+1, self.byte_offset-1) },
                            double_quoted: true
                        }, start_offset.clone());

                        self.context.pop();
                        self.consume(); //hinteres '"' zeichen wird nicht mehr gebraucht
                        return
                    }
                },
                Some(b'\'') => {
                    if !double_quoted {
                        //end
                        self.push_token(TokenTag::StringLiteral {
                            //value: unsafe { self.strquick(start_offset+1, self.byte_offset-1) },
                            double_quoted: false
                        }, start_offset.clone());
                        self.context.pop();
                        self.consume(); //hinteres "'" zeichen wird nicht mehr gebraucht
                        return
                    }
                }
                None => {
                    //unfinished string
                    self.context.pop();
                    return
                },
                _ => {}
            }
            current = self.consume();
            //println!("lol {:?} - {:?}", current.unwrap() as char, &self.position);
        }
    }
}