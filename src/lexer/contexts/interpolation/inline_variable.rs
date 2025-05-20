use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn match_inline_variable(&mut self) -> bool {
        let start = self.byte_offset;

        // Erstes Zeichen: Muss `$` sein
        let Some(b'$') = self.current() else {
            return false;
        };
        self.next(); // `$` konsumieren

        let var_start = self.byte_offset;

        // Vorschau auf erstes Zeichen nach `$`
        let Some(b) = self.peek() else {
            return false; // nur `$` allein → keine Variable
        };

        // Erlaubter erster Zeichen für Variable: ASCII-Buchstabe oder `_`, oder gültiges Unicode-Zeichen
        let first_valid = if b < 0x80 {
            b == b'_' || b.is_ascii_alphabetic()
        } else {
            let slice = &self.bytes[self.byte_offset..];
            match Self::decode_utf8_char(slice) {
                Some((cp, _)) => Self::is_php_identifier_char(cp, true),
                None => false,
            }
        };

        if !first_valid {
            return false; // kein gültiger Start → keine Variable
        }

        // Jetzt in die bestehende Schleife
        let mut valid = false;
        let mut first = true;

        while let Some(b) = self.peek() {
            if b < 0x80 {
                if Self::is_variable_break_byte(b) {
                    println!("lol {}", b as char);
                    break;
                }

                if (b == b'_' || b.is_ascii_alphabetic()) || (!first && b.is_ascii_digit()) {
                    self.next();
                    valid = true;
                    first = false;
                } else {
                    break;
                }
            } else {
                let slice = &self.bytes[self.byte_offset..];
                let (cp, len) = match Self::decode_utf8_char(slice) {
                    Some(v) => v,
                    None => break,
                };

                if Self::is_php_identifier_char(cp, first) {
                    self.next_n(len);
                    valid = true;
                    first = false;
                } else {
                    break;
                }
            }
        }

        if valid {
            let value = unsafe { self.strquick(var_start, self.exclusive_pos()) };
            self.push_token(TokenTag::Variable(value), start);
            println!("Context: {:?}", self.current().unwrap() as char);
            true
        } else {
            false
        }
    }

}