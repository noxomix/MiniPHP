use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn context_in_dq_string(&mut self) {
        let start_offset = self.byte_offset;
        self.next();

        let mut current = self.current();
        loop {
            match current {
                Some(b'\\') => {
                    //escape
                    self.next(); //also insgesamt werden 2 zeichen konsumiert, also wird \" und \' gecovert
                }
                Some(b'"') => {
                    //end
                    self.push_token(TokenTag::StringLiteral {
                        //value: unsafe { self.strquick(start_offset+1, self.byte_offset-1) },
                        double_quoted: true
                    }, start_offset.clone());

                    self.context.pop();
                    self.next(); //hinteres '"' zeichen wird nicht mehr gebraucht
                    return
                },
                /*Some(b'$') => {
                    //interpolation beginnt:
                    match self.look() {
                        Some(b'{') => {
                            //bracket beginn so further instruction is parsed
                        },
                        _ => {
                            //normal variable
                        }
                    }
                },*/
                None => {
                    //unfinished string
                    self.context.pop();
                    return
                },
                _ => {}
            }
            current = self.next();
            //println!("lol {:?} - {:?}", current.unwrap() as char, &self.position);
        }
    }
}