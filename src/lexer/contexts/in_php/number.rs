use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_number(&mut self) {
        let start = self.byte_offset;
        
        //todo for the future: match nan and inf here aswell, currently we match via keyword parser.

        // Hex Float z.B. 0x1.fp3
        if self.match_str(b"0x") || self.match_str(b"0X") {
            let mut seen_dot = false;
            let mut seen_exponent = false;

            while let Some(b) = self.peek() {
                match b {
                    b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'_' => {
                        self.next();
                    }
                    b'.' if !seen_dot => {
                        seen_dot = true;
                        self.next();
                    }
                    b'p' | b'P' if !seen_exponent => {
                        seen_exponent = true;
                        self.next();
                        if let Some(b'+' | b'-') = self.peek() {
                            self.next();
                        }
                    }
                    _ => break,
                }
            }

            let value = unsafe { self.strquick(start, self.byte_offset + 1) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        // Binär z.B. 0b1010_0001
        if self.match_str(b"0b") || self.match_str(b"0B") {
            while let Some(b) = self.peek() {
                if b == b'0' || b == b'1' || b == b'_' {
                    self.next();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset + 1) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        // Oktal mit 0o oder 0O
        if self.match_str(b"0o") || self.match_str(b"0O") {
            while let Some(b) = self.peek() {
                if (b'0'..=b'7').contains(&b) || b == b'_' {
                    self.next();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset + 1) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        // Klassische Oktalzahl (beginnend mit 0)
        if self.peek() == Some(b'0') {
            self.next(); // erste 0 konsumieren
            while let Some(b) = self.peek() {
                if (b'0'..=b'7').contains(&b) || b == b'_' {
                    self.next();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset + 1) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        // Float/Dezimal
        let mut seen_dot = false;
        let mut seen_exp = false;

        while let Some(b) = self.peek() {
            match b {
                b'0'..=b'9' | b'_' => {
                    self.next();
                }
                b'.' if !seen_dot => {
                    seen_dot = true;
                    self.next();
                }
                b'e' | b'E' if !seen_exp => {
                    seen_exp = true;
                    self.next();
                    if let Some(b'+' | b'-') = self.peek() {
                        self.next();
                    }
                }
                _ => break,
            }
        }

        let value = unsafe { self.strquick(start, self.byte_offset + 1) };
        self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
    }

    #[inline(always)]
    fn match_str(&mut self, s: &[u8]) -> bool {
        let end = self.byte_offset + s.len();
        if self.bytes.len() >= end && &self.bytes[self.byte_offset..end] == s {
            for _ in 0..s.len() {
                self.next();
            }
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn match_str_ignore_case(&mut self, s: &[u8]) -> bool {
        let end = self.byte_offset + s.len();
        if self.bytes.len() >= end
            && self.bytes[self.byte_offset..end]
            .iter()
            .zip(s.iter())
            .all(|(a, b)| a.to_ascii_lowercase() == b.to_ascii_lowercase())
        {
            for _ in 0..s.len() {
                self.next();
            }
            true
        } else {
            false
        }
    }
}
