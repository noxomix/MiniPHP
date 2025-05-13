use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_number(&mut self) {
        let start = self.byte_offset;

        // Sonderformate: 0x, 0b, 0o
        if self.look() == Some(b'x') || self.look() == Some(b'X') {
            self.consume(); // x
            while let Some(b) = self.look() {
                if b.is_ascii_hexdigit() || b == b'_' {
                    self.consume();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        if self.look() == Some(b'b') || self.look() == Some(b'B') {
            self.consume(); // b
            while let Some(b) = self.look() {
                if b == b'0' || b == b'1' || b == b'_' {
                    self.consume();
                } else {
                    break;
                }
            }
            let value = unsafe { self.strquick(start, self.byte_offset) };
            self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
            return;
        }

        if self.look() == Some(b'o') || self.look() == Some(b'O') {
            self.consume(); // o
            while let Some(b) = self.look() {
                if b >= b'0' && b <= b'7' || b == b'_' {
                    self.consume();
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
            while let Some(b) = self.look() {
                if b >= b'0' && b <= b'7' || b == b'_' {
                    self.consume();
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

        while let Some(b) = self.look() {
            match b {
                b'0'..=b'9' | b'_' => {self.consume();},
                b'.' if !seen_dot => {
                    seen_dot = true;
                    self.consume();
                }
                b'e' | b'E' if !seen_exponent => {
                    seen_exponent = true;
                    self.consume();
                    if let Some(b'+' | b'-') = self.look() {
                        self.consume();
                    }
                }
                _ => break,
            }
        }

        let value = unsafe { self.strquick(start, self.byte_offset) };
        self.push_token(TokenTag::NumberLiteral(value.to_string()), start);
    } //muss noch mal angeschaut werden, zudem ist keine Binär oder Hex notation geparsed bisher..&self.bytes[start..self.byte_offset]).unwrap().to_string()), start)
}