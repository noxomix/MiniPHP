use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn context_in_comment_line(&mut self) {
        let start_offset = self.byte_offset;

        // Konsumiere die Kommentar-Einleitung
        if self.current() == Some(b'#') {
            self.next(); // '#'
        } else {
            self.next_n(2); // "//"
        }

        while let Some(b) = self.peek() {
            match b {
                b'\n' | b'\r' => break, // ← Nur gucken, nicht konsumieren
                _ => {
                    self.next(); // ← alles andere konsumieren
                }
            }
        }

        self.push_token(TokenTag::Comment {
            multiline: false,
        }, start_offset);

        self.context.pop();
        self.next(); //letztes zeichen trotzdem noch konsumieren damit der context_in_php matcher jetzt bei \n weiter macht.
    }
}