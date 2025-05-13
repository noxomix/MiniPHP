use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    #[inline(always)]
    pub fn match_variable(&mut self) {
        let start = self.byte_offset;
        self.consume(); // '$' sofort konsumieren

        let mut end = self.byte_offset;

        while let Some(b) = self.look() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.consume();
                end = self.byte_offset; // merken
            } else if b >= 0x80 {
                let len = Self::utf8_char_len(b);
                if len == 0 || self.byte_offset + len > self.bytes.len() {
                    break;
                }
                self.consume_n(len);
                end = self.byte_offset;
            } else {
                break;
            }
        }

        if end > start {
            // end zeigt auf das erste ungültige Zeichen → wir wollen das letzte gültige mitnehmen (also end-1)
            let value = unsafe { self.strquick(start, end - 1) }.to_string();
            self.push_token(TokenTag::Variable(value), start);
        }
    }
}