/*
    Default handler which will parse PHP context.
*/
use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext, Tokenizer};
use crate::lexer::token::TokenTag;

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
                b'\'' => {
                    let start_offset = self.byte_offset;
                    current = self.consume();
                    loop {
                        match current {
                            Some(b'\'') => {
                                self.push_token(TokenTag::StringLiteral {
                                    value: unsafe { self.strquick(start_offset+1, self.byte_offset-1) },
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
                        Some(b'?') => {
                            self.consume();
                            if self.look() == Some(b'=') {
                                self.consume();
                                self.push_token(TokenTag::NullCoalesceAssign, start);
                            } else {
                                self.push_token(TokenTag::NullCoalesce, start);
                            }
                        }
                        _ => self.push_token(TokenTag::TernaryQuestion, start),
                    }
                },
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
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    if let Some(tag) = self.match_keyword() {
                        self.push_token(tag, self.byte_offset); // start offset ggf. vorher merken
                    }
                    //todo alles andere wie zB funktionsnamen, konstanten
                },
                _ => {
                    //println!("{:?}", current.unwrap() as char)
                }
            }

            current = self.consume();
        }
    }

    //keyword matching
    fn match_keyword(&mut self) -> Option<TokenTag> {
        for &(word, ref tag) in KEYWORDS {
            if self.remaining().starts_with(word) {
                // danach darf kein a-zA-Z0-9_ folgen (sonst z.B. "returnValue")
                if let Some(&b) = self.remaining().get(word.len()) {
                    if b.is_ascii_alphanumeric() || b == b'_' {
                        continue;
                    }
                }
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


}

// Reihenfolge nach Anfangsbuchstabe gruppiert, dann nach Länge sortiert
const KEYWORDS: &[(&[u8], TokenTag)] = &[
    (b"break", TokenTag::BreakStatement),
    (b"case", TokenTag::CaseStatement),
    (b"class", TokenTag::ClassStatement),
    (b"continue", TokenTag::ContinueStatement),
    (b"do", TokenTag::DoStatement),
    (b"else", TokenTag::ElseStatement),
    (b"elseif", TokenTag::ElseIfStatement),
    (b"for", TokenTag::ForStatement),
    (b"foreach", TokenTag::ForeachStatement),
    (b"function", TokenTag::FunctionStatement),
    (b"if", TokenTag::IfStatement),
    (b"return", TokenTag::ReturnStatement),
    (b"switch", TokenTag::SwitchStatement),
    (b"trait", TokenTag::TraitStatement),
    (b"while", TokenTag::WhileStatement),
];
