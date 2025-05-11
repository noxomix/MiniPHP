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
                b'/' => match self.look() {
                    Some(b'/') => {
                        self.context.push(LexerContext::InCommentLine);
                        return
                    },
                    Some(b'*') => {
                        self.context.push(LexerContext::InCommentBlock);
                        return
                        //self.handle_comment_block()
                    },
                    Some(b'=') => {
                        let start = self.byte_offset;
                        self.consume();
                        self.push_token(TokenTag::DivAssign, start);
                    }
                    _ => {
                        self.push_token(TokenTag::Division, self.byte_offset);
                    }
                }, //kommentare und diff assign
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
                } //'+..'
                b'-' => {
                    let start = self.byte_offset;
                    match self.look() {
                        Some(b'-') => {
                            self.consume();
                            self.push_token(TokenTag::Decrement, start);
                        }
                        Some(b'=') => {
                            self.consume();
                            self.push_token(TokenTag::SubAssign, start);
                        }
                        _ => {
                            self.push_token(TokenTag::Minus, start);
                        }
                    }
                } //'-..'
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
                } //'*..'
                b'%' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'=') {
                        self.consume();
                        self.push_token(TokenTag::ModAssign, start);
                    } else {
                        self.push_token(TokenTag::Modulo, start);
                    }
                } //'%..' (modulo)
                b'&' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'&') {
                        self.consume();
                        self.push_token(TokenTag::LogicalAnd, start);
                    } else {
                        self.push_token(TokenTag::BitAnd, start);
                    }
                } //'&|&&' logical and bitwise <AND>
                b'|' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'|') {
                        self.consume();
                        self.push_token(TokenTag::LogicalOr, start);
                    } else {
                        self.push_token(TokenTag::BitOr, start);
                    }
                } //'|/||' logical and bitwise <OR>
                b'^' => {
                    self.push_token(TokenTag::BitXor, self.byte_offset);
                } //'^' bitwise <XOR>
                b'=' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'=') {
                        self.consume();
                        if self.look() == Some(b'=') {
                            self.consume();
                            self.push_token(TokenTag::IsIdentical, start);
                        } else {
                            self.push_token(TokenTag::IsEqual, start);
                        }
                    } else {
                        self.push_token(TokenTag::Assign, start);
                    }
                } //'=..'
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
                } //'!..'
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
                } //'<..'
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
                } //'>..'
                b'?' => {
                    let start = self.byte_offset;
                    if self.look() == Some(b'?') {
                        self.consume();
                        if self.look() == Some(b'=') {
                            self.consume();
                            self.push_token(TokenTag::NullCoalesceAssign, start);
                        } else {
                            self.push_token(TokenTag::NullCoalesce, start);
                        }
                    }
                } //'?..' und '??'
                _ => {}
            }

            current = self.consume();
        }
    }
}