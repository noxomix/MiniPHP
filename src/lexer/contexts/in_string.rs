use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn context_in_dq_string(&mut self) {
        let start_offset = self.byte_offset; //kann bei interpolation durchaus scheiße aussehen aber muss vor augen führen das nur wichtig ist der inhalt nicht "" oder was auch immer vorher steht.
        self.next(); //consume des start zeichens

        let mut current = self.current();
        loop {
            match current {
                Some(b'$') => {
                    /* Falls es doch keine variable ist sonder allein stehendes '$' handeln wir das über einen workaround in "in interpolation" */
                    self.push_token_withend(TokenTag::StringLiteral {double_quoted: true}, start_offset, self.byte_offset);
                    self.context.push(LexerContext::InInterpolation);
                    return;
                },
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