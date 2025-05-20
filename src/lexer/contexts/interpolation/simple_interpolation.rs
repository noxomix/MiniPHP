use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    /*
    Direct Interpolation = "Hallo $name" or "Hallo $person->first_name"
     */
    pub fn match_simple_interpolation(&mut self) -> bool {
        let Some(b'$') = self.current() else {
            return false;
        };

        // Save initial position in case we want to rewind
        let start = self.byte_offset;

        // Match variable name
        if !self.match_inline_variable() {
            // Invalid variable name after $
            return false;
        }
        //current ist jetzt auf dem letzten zeichen der variable

        // Optional: match `->prop` once
        if self.peek() == Some(b'-') && self.peek_n(2) == Some(b'>') {
            self.next(); //letztes zeichen von variable konsumieren
            self.next(); //'-' konsumieren, also ist current = '>'
            self.push_token_withend(TokenTag::Arrow, self.byte_offset-1, self.exclusive_pos());
            self.next(); //'>' konsumieren
            
            //todo maybe what if there is no valid keyword here?

            self.match_keywords();

            // Disallow any further ->
            if self.peek() == Some(b'-') && self.peek_n(2) == Some(b'>') {
                //self.byte_offset = start;
                return false;
            }
        }

        // Valid simple interpolation ends cleanly
        true
    }

}