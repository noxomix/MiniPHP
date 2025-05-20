use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    /*
    Direct Interpolation = "Hallo $name" or "Hallo $person->first_name" or "Hallo $x[44]"
     */
    pub fn context_in_direct_interpolation(&mut self) {
        let mut had_arrow = false;

        while let Some(c) = self.current() {
            match c {
                b'$' => {
                    if self.peek() == Some(b'{') {
                        //complex interpolation context switch todo
                    } else {
                        self.match_variable();
                    }
                },
                b'-' => {
                    if self.peek() == Some(b'>') && !had_arrow {
                        self.push_token(TokenTag::Arrow, self.byte_offset); //'->'
                        had_arrow = true;
                        self.next(); // '-'
                    } else {
                        //abbruch
                        self.context.pop();
                        return;
                    }
                },
                b'\'' => {
                    self.match_single_q_string(); // z. B. ${'foo'}
                },
                b'.' | b'+' | b'*' | b'/' | b'%' |
                b'!' | b'?' | b':' | b'=' | b'&' | b'|' |
                b'^' | b'~' | b',' | b';' | b')' | b'(' |
                b'{' | b'}' | b'<' | b'>' |
                b' ' | b'\t' | b'\n' | b'\r' | b'"' => {
                    self.context.pop();
                    return;
                },
                _ => {
                    self.match_keywords()
                }
            }

            self.next();
        }

        // Falls das Ende erreicht ist ohne schließende Klammer
        self.context.pop(); // raus aus Interpolation
    }

}