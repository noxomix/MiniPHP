use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn match_variable(&mut self) {
        let start = self.byte_offset;

        let mut first = true;
        let mut valid = false;

        while let Some(b) = self.look() {
            if b < 0x80 {
                if Self::is_variable_break_byte(b) {
                    break;
                }

                // ASCII prüfen
                if (b == b'_' || b.is_ascii_alphabetic()) || (!first && b.is_ascii_digit()) {
                    self.next();
                    valid = true;
                    first = false;
                    continue;
                } else {
                    break;
                }
            } else {
                // Multibyte → Unicode-Zeichen analysieren
                let slice = &self.bytes[self.byte_offset+1..]; //hier +1 weil wir ja mit look() in der zukunft sind. (@bug richtig lange gdauert zu fixen :))
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

        let end = self.byte_offset;

        if valid {
            let value = unsafe { self.strquick(start, end) };
            self.push_token(TokenTag::Variable(value), start);
        } else {
            println!("Big error at {}", self.byte_offset);
        }
    }

    #[inline(always)]
    fn is_variable_break_byte(b: u8) -> bool {
        matches!(
            b,
            b' ' | b'\t' | b'\r' | b'\n' |
            b'(' | b')' | b'{' | b'}' |
            b'[' | b']' | b',' | b'.' |
            b';' | b':' | b'=' | b'+' |
            b'-' | b'*' | b'/' | b'%' |
            b'<' | b'>' | b'!' | b'&' |
            b'|' | b'^' | b'~' | b'?' |
            b'"' | b'\'' | b'`' | b'#' |
            b'$'
        )
    }

    #[inline(always)]
    fn decode_utf8_char(bytes: &[u8]) -> Option<(u32, usize)> {
        let b0 = *bytes.get(0)?;
        match b0 {
            0x00..=0x7F => Some((b0 as u32, 1)),
            0xC2..=0xDF if bytes.len() >= 2 => {
                Some((((b0 & 0x1F) as u32) << 6 | (bytes[1] & 0x3F) as u32, 2))
            }
            0xE0..=0xEF if bytes.len() >= 3 => {
                Some((((b0 & 0x0F) as u32) << 12 |
                          ((bytes[1] & 0x3F) as u32) << 6 |
                          (bytes[2] & 0x3F) as u32, 3))
            }
            0xF0..=0xF4 if bytes.len() >= 4 => {
                Some((((b0 & 0x07) as u32) << 18 |
                          ((bytes[1] & 0x3F) as u32) << 12 |
                          ((bytes[2] & 0x3F) as u32) << 6 |
                          (bytes[3] & 0x3F) as u32, 4))
            }
            _ => None,
        }
    }

    #[inline(always)]
    fn is_php_identifier_char(cp: u32, is_first: bool) -> bool {
        match cp {
            0x0041..=0x005A | // A-Z
            0x0061..=0x007A | // a-z
            0x00C0..=0x00D6 |
            0x00D8..=0x00F6 |
            0x00F8..=0x02FF |
            0x0370..=0x03FF | // Greek
            0x0400..=0x04FF | // Cyrillic
            0x0530..=0x058F | // Armenian
            0x0590..=0x05FF | // Hebrew
            0x0600..=0x06FF | // Arabic
            0x0900..=0x097F | // Devanagari
            0x4E00..=0x9FFF | // CJK
            0xAC00..=0xD7AF   // Hangul
            => true,
            0x0030..=0x0039 if !is_first => true, // Ziffern nur wenn nicht am Anfang
            _ => false,
        }
    }
}
