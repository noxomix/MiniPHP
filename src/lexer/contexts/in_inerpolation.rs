use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn context_string_interpolation(&mut self) {
        let start_offset = self.byte_offset;
 // Fallback auf Identifier, z. B. bei `${foo->bar}`
        // Interpolation startet entweder mit ${
        if self.look() == Some(b'$') && self.look_n(1) == Some(b'{') {
            self.consume(); // $
            self.consume(); // {
            self.push_token(TokenTag::StringInterpolation("${".into()), start_offset);
        }
        // oder mit { (wie in "{$var}")
        else if self.look() == Some(b'{') {
            self.consume(); // {
            self.push_token(TokenTag::StringInterpolation("{".into()), start_offset);
        }
        // oder direkt $ (wie "$var")
        else if self.look() == Some(b'$') {
            self.match_variable(); // verwendet vorhandene match_variable()
            self.context.pop(); // zurück in normalen Stringkontext
            return;
        } else {
            // Ungültig – raus
            self.context.pop();
            return;
        }

        // Jetzt im Inneren von { ... } oder ${ ... }
        loop {
            match self.look() {
                Some(b'}') => {
                    let end = self.byte_offset;
                    self.consume();
                    self.push_token(TokenTag::StringInterpolation("}".into()), end);
                    self.context.pop(); // zurück in String-Kontext
                    return;
                }

                Some(b'$') => {
                    self.match_variable(); // $var, $arr['key'], $obj->field etc.
                }

                Some(b'0'..=b'9') => {
                    self.match_number(); // erlaubt z. B. in ${foo[123]}
                }

                Some(b'\'') => {
                    self.match_single_q_string();
                }

                Some(b'-') => {
                    // negativ? Zahl oder Fehler?
                    if let Some(b'0'..=b'9') = self.look_n(1) {
                        self.match_number();
                    } else {
                        self.consume(); // minus oder ungültig
                    }
                }

                Some(b'[') | Some(b']') | Some(b'(') | Some(b')') |
                Some(b':') | Some(b'|') | Some(b'^') | Some(b'&') |
                Some(b'.') | Some(b',') | Some(b';') | Some(b'+') |
                Some(b'*') | Some(b'/') | Some(b'%') | Some(b'=') |
                Some(b'>') | Some(b'<') | Some(b'?') | Some(b'!') |
                Some(b'@') => {
                    // Diese Zeichen dürfen als Teil von Ausdrücken/Indexierungen auftauchen
                    self.consume();
                }

                Some(b' ' | b'\t' | b'\n' | b'\r') => {
                    self.consume(); // Whitespace innerhalb erlaubt (z. B. ${ $foo })
                }

                Some(_) => {
                    self.match_keywords();
                }

                None => {
                    self.context.pop();
                    return;
                }
            }
        }
    }
}
