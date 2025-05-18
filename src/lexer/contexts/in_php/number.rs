use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_number(&mut self) {
        let start = self.byte_offset;

        // Sonderformate: 0x, 0b, 0o
        if self.peek() == Some(b'x') || self.peek() == Some(b'X') {
            self.next(); // x
            while let Some(b) = self.peek() {
                if b.is_ascii_hexdigit() || b == b'_' {
                    self.next();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        if self.peek() == Some(b'b') || self.peek() == Some(b'B') {
            self.next(); // b
            while let Some(b) = self.peek() {
                if b == b'0' || b == b'1' || b == b'_' {
                    self.next();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        if self.peek() == Some(b'o') || self.peek() == Some(b'O') {
            self.next(); // o
            while let Some(b) = self.peek() {
                if b >= b'0' && b <= b'7' || b == b'_' {
                    self.next();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        // klassische Oktal: beginnt mit 0, gefolgt von 0–7
        if self.bytes.get(start) == Some(&b'0') {
            while let Some(b) = self.peek() {
                if b >= b'0' && b <= b'7' || b == b'_' {
                    self.next();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        // Dezimal/Gleitkommazahl mit optionalen Unterstrichen
        let mut seen_dot = false;
        let mut seen_exponent = false;

        while let Some(b) = self.peek() {
            match b {
                b'0'..=b'9' | b'_' => {self.next();},
                b'.' if !seen_dot => {
                    seen_dot = true;
                    self.next();
                }
                b'e' | b'E' if !seen_exponent => {
                    seen_exponent = true;
                    self.next();
                    if let Some(b'+' | b'-') = self.peek() {
                        self.next();
                    }
                }
                _ => break,
            }
        }

        let value = unsafe { self.strquick(start, self.byte_offset) };
        self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
    } //muss noch mal angeschaut werden, zudem ist keine Binär oder Hex notation geparsed bisher..&self.bytes[start..self.byte_offset]).unwrap().to_string()), start)
}