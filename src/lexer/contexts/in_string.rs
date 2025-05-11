use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn handle_string(&mut self) {
        let start_position = self.position.clone();
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
                            value: unsafe { self.strquick(start_position.byte_offset+1, self.position.byte_offset-1) },
                            debug_value: unsafe { self.strquick(start_position.byte_offset, self.position.byte_offset) },
                            double_quoted: true 
                        }, start_position.clone());
                        
                        self.context.pop();
                        self.consume(); //hinteres '"' zeichen wird nicht mehr gebraucht
                        return
                    }
                },
                Some(b'\'') => {
                    if !double_quoted {
                        //end
                        self.push_token(TokenTag::StringLiteral {
                            value: unsafe { self.strquick(start_position.byte_offset+1, self.position.byte_offset-1) },
                            debug_value: unsafe { self.strquick(start_position.byte_offset, self.position.byte_offset) },
                            double_quoted: false 
                        }, start_position.clone());
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