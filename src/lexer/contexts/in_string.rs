use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn context_in_dq_string(&mut self) {
        let mut start_offset = self.byte_offset; //kann bei interpolation durchaus scheiße aussehen aber muss vor augen führen das nur wichtig ist der inhalt nicht "" oder was auch immer vorher steht.
        self.next(); //consume des start zeichens

        let mut current = self.current();
        loop {
            match current {
                Some(b'$') => {
                    if self.peek() == Some(b'{') {
                        //dynamic variable name ${}
                        //todo
                    } else {
                        /* "Test $hallo->name" oder auch "Test $x lala $y zett" */
                        self.push_token_withend(TokenTag::StringLiteral {double_quoted: true}, start_offset, self.byte_offset+1); //das hier ein $ am ende steht ist tatsächlich gewollt weil strings immer +-1 zeichen. (nrmaleerwise steht da ja "")
                        self.push_token_withend(TokenTag::Dot, 0, 0);
                        let valid_interpolation = self.match_simple_interpolation();
                        println!("Context e: {:?}", self.current().unwrap() as char);
                        if !valid_interpolation {
                            self.pop_token(); //'pop stringliteral'
                            self.pop_token(); //'pop concat DOT'
                        } else {
                            self.push_token_withend(TokenTag::Dot, 0, 0);
                            //neuer string wird ab jetzt weiter geparsed
                            start_offset = self.byte_offset;
                            //println!("hm {}", self.current().unwrap() as char);
                        }
                    }
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
                    //println!("{:?}", self.context.last());
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