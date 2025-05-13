/*
    Default handler which will parse PHP context.
*/
use std::string::ToString;
use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext, Tokenizer};
use crate::lexer::token::StatementType::{Abstract, As, Break, Case, Class, Continue, DefaultCase, Do, Else, ElseIf, For, Foreach, Function, If, Namespace, Return, Switch, Trait, While};
use crate::lexer::token::TokenTag;
use crate::lexer::token::TokenTag::Identifier;

impl Lexer {
    pub fn handle_php(&mut self) {
        let mut current = self.current();
        while let Some(c) = current {
            match c {
                b'$' => {
                    let start = self.byte_offset;
                    self.consume(); // '$'
                    while let Some(&b) = self.bytes.get(self.byte_offset) {
                        if b.is_ascii_alphanumeric() || b == b'_' {
                            self.consume();
                        } else if b >= 0x80 {
                            // UTF-8 Mehrbyte-Sequenz erkennen & überspringen
                            let len = Self::utf8_char_len(b);
                            if len == 0 || self.byte_offset + len > self.bytes.len() {
                                break; // ungültig oder unvollständig
                            }
                            self.consume_n(len);
                        } else {
                            break;
                        }
                    }
                    self.push_token(TokenTag::Variable(std::str::from_utf8(&self.bytes[start..self.byte_offset]).unwrap().to_string()), start);
                }, //'$..'
                b'"' => {
                    self.context.push(LexerContext::InString);
                    return;
                }, //'"' double-quoted string
                b'0'..=b'9' => {
                    self.match_number(); // delegiert an eigene Funktion
                },
                b'\'' => {
                    let start_offset = self.byte_offset;
                    current = self.consume();
                    loop {
                        match current {
                            Some(b'\'') => {
                                self.push_token(TokenTag::StringLiteral {
                                    //value: unsafe { self.strquick(start_offset+1, self.byte_offset-1) },
                                    double_quoted: false
                                }, start_offset);
                                break;
                            }
                            Some(b'\\') => {
                                match self.look() {
                                    Some(b'\'') | Some(b'\\') => {
                                        self.consume(); // skip escape
                                    }
                                    _ => {}
                                }
                            }
                            None => return,
                            _ => {}
                        }
                        current = self.consume();
                    }
                }, //"'" single-quoted string
                b';' => {
                    self.push_token(TokenTag::Semicolon, self.byte_offset)
                },
                b'/' => {
                    match self.look() {
                        Some(b'/') => {
                            self.context.push(LexerContext::InCommentLine);
                            return;
                        }
                        Some(b'*') => {
                            self.context.push(LexerContext::InCommentBlock);
                            return;
                            //self.handle_comment_block()
                        }
                        Some(b'=') => {
                            let start = self.byte_offset;
                            self.consume();
                            self.push_token(TokenTag::DivAssign, start);
                        }
                        _ => {
                            self.push_token(TokenTag::Division, self.byte_offset);
                        }
                    }
                }, //kommentare und diff assign
                b'#' => {
                    self.context.push(LexerContext::InCommentLine);
                    return;
                }, //single line kommentar mit '#' (!)
                b' ' => { self.push_token(TokenTag::Whitespace, self.byte_offset)} //' ' (whitespace)
                b'=' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'>') => {
                            self.consume();
                            self.push_token(TokenTag::FatArrow, start);
                        }
                        Some(b'=') => {
                            self.consume();
                            if self.look() == Some(b'=') {
                                self.consume();
                                self.push_token(TokenTag::IsIdentical, start);
                            } else {
                                self.push_token(TokenTag::IsEqual, start);
                            }
                        }
                        _ => self.push_token(TokenTag::Assign, start),
                    }
                },
                b'{' => {
                    self.push_token(TokenTag::LeftBrace, self.byte_offset)
                },
                b'}' => {
                    self.push_token(TokenTag::RightBrace, self.byte_offset)
                },
                b'(' => {
                    self.push_token(TokenTag::LeftParen, self.byte_offset)
                },
                b')' => {
                    self.push_token(TokenTag::RightParen, self.byte_offset)
                },
                b'\t' => {
                    self.push_token(TokenTag::Tab, self.byte_offset);
                },
                b'\n' => {
                    self.push_token(TokenTag::Newline, self.byte_offset);
                },
                b'\r' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'\n') {
                        self.consume(); // skip '\n' in \r\n
                    }
                    self.push_token(TokenTag::Newline, start);
                },
                b'[' => {
                    self.push_token(TokenTag::LeftBracket, self.byte_offset)
                },
                b']' => {
                    self.push_token(TokenTag::RightBracket, self.byte_offset)
                },
                b'&' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'&') {
                        self.consume();
                        self.push_token(TokenTag::LogicalAnd, start);
                    } else {
                        self.push_token(TokenTag::BitAnd, start);
                    }
                }, //'&|&&' logical and bitwise <AND>
                b'|' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'|') {
                        self.consume();
                        self.push_token(TokenTag::LogicalOr, start);
                    } else {
                        self.push_token(TokenTag::BitOr, start);
                    }
                }, //'|/||' logical and bitwise <OR>
                b'!' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'=') {
                        self.consume();
                        if self.look() == Some(b'=') {
                            self.consume();
                            self.push_token(TokenTag::IsNotIdentical, start);
                        } else {
                            self.push_token(TokenTag::IsNotEqual, start);
                        }
                    }
                }, //'!..'
                b'<' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'=') => {
                            self.consume();
                            self.push_token(TokenTag::IsSmallerOrEqual, start);
                        }
                        Some(b'<') => {
                            self.consume();
                            self.push_token(TokenTag::ShiftLeft, start);
                        }
                        _ => {
                            self.push_token(TokenTag::IsSmaller, start);
                        }
                    }
                }, //'<..'
                b'>' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'=') => {
                            self.consume();
                            self.push_token(TokenTag::IsGreaterOrEqual, start);
                        }
                        Some(b'>') => {
                            self.consume();
                            self.push_token(TokenTag::ShiftRight, start);
                        }
                        _ => {
                            self.push_token(TokenTag::IsGreater, start);
                        }
                    }
                }, //'>..'
                b'.' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'=') => {
                            self.consume();
                            self.push_token(TokenTag::ConcatAssign, start);
                        }
                        _ => self.push_token(TokenTag::Dot, start),
                    }
                },
                b':' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b':') => {
                            self.consume();
                            self.push_token(TokenTag::DoubleColon, start);
                        }
                        _ => self.push_token(TokenTag::Colon, start),
                    }
                },
                b'?' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'>') => {
                            self.consume(); // consume '>'
                            self.push_token(TokenTag::PhpCloseTag, start);
                            self.context.pop();
                            return
                        }
                        Some(b'?') => {
                            self.consume(); // consume 2nd '?'
                            if self.look() == Some(b'=') {
                                self.consume();
                                self.push_token(TokenTag::NullCoalesceAssign, start);
                            } else {
                                self.push_token(TokenTag::NullCoalesce, start);
                            }
                        }
                        _ => {
                            self.push_token(TokenTag::TernaryQuestion, start);
                        }
                    }
                }
                b',' => {
                    self.push_token(TokenTag::Comma, self.byte_offset)
                },
                b'-' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'>') => {
                            self.consume();
                            self.push_token(TokenTag::Arrow, start);
                        }
                        Some(b'-') => {
                            self.consume();
                            self.push_token(TokenTag::Decrement, start);
                        }
                        Some(b'=') => {
                            self.consume();
                            self.push_token(TokenTag::SubAssign, start);
                        }
                        _ => self.push_token(TokenTag::Minus, start),
                    }
                },
                b'+' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'+') => {
                            self.consume();
                            self.push_token(TokenTag::Increment, start);
                        }
                        Some(b'=') => {
                            self.consume();
                            self.push_token(TokenTag::AddAssign, start);
                        }
                        _ => {
                            self.push_token(TokenTag::Plus, start);
                        }
                    }
                }, //'+..'
                b'*' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'=') => {
                            self.consume();
                            self.push_token(TokenTag::MulAssign, start);
                        }
                        Some(b'*') => {
                            self.consume();
                            if self.look() == Some(b'=') {
                                self.consume();
                                self.push_token(TokenTag::PowerAssign, start);
                            } else {
                                self.push_token(TokenTag::Power, start);
                            }
                        }
                        _ => {
                            self.push_token(TokenTag::Multiply, start);
                        }
                    }
                }, //'*..'
                b'%' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'=') {
                        self.consume();
                        self.push_token(TokenTag::ModAssign, start);
                    } else {
                        self.push_token(TokenTag::Modulo, start);
                    }
                }, //'%..' (modulo)
                b'^' => {
                    self.push_token(TokenTag::BitXor, self.byte_offset);
                }, //'^' bitwise <XOR>
                b'\\' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'\\') {
                        self.consume();
                        self.push_token(TokenTag::NamespaceBackslash, start);
                    }
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    let start_position = self.byte_offset;

                    if let Some(tag) = self.match_keyword() {
                        self.push_token(tag, start_position);
                    } else {
                        let mut len = 1; // wir wissen: das erste Zeichen ist gültig
                        // Zähle alle folgenden gültigen Identifier-Zeichen
                        while let Some(b) = self.look_n(len) {
                            if b.is_ascii_alphanumeric() || b == b'_' {
                                len += 1;
                            } else if b >= 0x80 {
                                let utf_len = Self::utf8_char_len(b);
                                if utf_len == 0 {
                                    break;
                                }

                                // look_n gibt nur 1 Byte – prüfe, ob genug Daten da sind
                                if self.byte_offset + len + utf_len - 1 >= self.bytes.len() {
                                    break;
                                }

                                len += utf_len;
                            } else {
                                break;
                            }
                        }
                        // Jetzt `len` gültige Bytes → konsumieren
                        self.consume_n(len);
                        // `byte_offset` zeigt auf das **letzte gültige Byte** → perfekt für inclusive strquick
                        let ident = unsafe { self.strquick(start_position, self.byte_offset - 1) };
                        self.push_token(TokenTag::Identifier(ident.to_string()), start_position);
                        //println!("{:?}", ident.to_string());
                    }
                },
                _ => {
                    println!("{:?} - {:?}", current.unwrap() as char, self.byte_offset);
                }
            }

            current = self.consume();
        }
    }

    //keyword matching
    fn match_keyword(&mut self) -> Option<TokenTag> {
        for &(word, ref tag) in KEYWORDS {
            let remaining = self.remaining();
            if remaining.len() < word.len() {
                continue;
            }

            // prüfe auf Wortende
            if let Some(&b) = remaining.get(word.len()) {
                if b.is_ascii_alphanumeric() || b == b'_' {
                    continue;
                }
            }

            let candidate = &remaining[..word.len()];

            // effizient: Eingabe lowercasen (1×), gegen lowercase-Keyword vergleichen
            let equal = candidate
                .iter()
                .zip(word.iter())
                .all(|(a, b)| a.to_ascii_lowercase() == *b);

            if equal {
                self.consume_n(word.len());
                return Some(tag.clone());
            }
        }
        None
    }


    fn utf8_char_len(first: u8) -> usize {
        match first {
            0b0000_0000..=0b0111_1111 => 1, // ASCII
            0b1100_0000..=0b1101_1111 => 2,
            0b1110_0000..=0b1110_1111 => 3,
            0b1111_0000..=0b1111_0111 => 4,
            _ => 0, // ungültig (z. B. Fortsetzungsbytes 0b10xxxxxx)
        }
    }

    fn match_number(&mut self) {
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
    } //muss noch mal angeschaut werden, zudem ist keine Binär oder Hex notation geparsed bisher..

}

// Reihenfolge nach Anfangsbuchstabe gruppiert, dann nach Länge sortiert
const KEYWORDS: &[(&[u8], TokenTag)] = &[
    (b"null", TokenTag::NullLiteral),
    (b"break", TokenTag::Statement(Break)),
    (b"case", TokenTag::Statement(Case)),
    (b"class", TokenTag::Statement(Class)),
    (b"continue", TokenTag::Statement(Continue)),
    (b"do", TokenTag::Statement(Do)),
    (b"else", TokenTag::Statement(Else)),
    (b"elseif", TokenTag::Statement(ElseIf)),
    (b"for", TokenTag::Statement(For)),
    (b"foreach", TokenTag::Statement(Foreach)),
    (b"as", TokenTag::Statement(As)),
    (b"function", TokenTag::Statement(Function)),
    (b"if", TokenTag::Statement(If)),
    (b"return", TokenTag::Statement(Return)),
    (b"switch", TokenTag::Statement(Switch)),
    (b"trait", TokenTag::Statement(Trait)),
    (b"while", TokenTag::Statement(While)),
    (b"while", TokenTag::Statement(While)),
    (b"abstract", TokenTag::Statement(Abstract)),
    (b"default", TokenTag::Statement(DefaultCase)),
    (b"namespace", TokenTag::Statement(Namespace)),

    //spcial nummern:
    (b"nan", TokenTag::NumberNan),
    (b"inf", TokenTag::NumberInfinity),
    (b"infinity", TokenTag::NumberInfinity),

    //booleans
    (b"false", TokenTag::BooleanLiteral(false)),
    (b"true", TokenTag::BooleanLiteral(true)),
];