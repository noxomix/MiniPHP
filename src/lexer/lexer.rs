use std::cell::RefCell;
use std::sync::Arc;
use crate::lexer::token::{Position, Token, TokenTag};

pub enum LexerContext {
    InHtml,           // Außerhalb von <?php, in reinem HTML
    InPhp,            // Innerhalb von <?php ... ?>
    InString,         // Innerhalb eines einfachen/komplexen Strings
    InHeredoc,        // Innerhalb von <<<EOD
    InNowdoc,         // Innerhalb von <<<EOD mit ''
    InCommentLine,    // // oder #
    InCommentBlock,   // /* ... */
    InEncapsulated,   // Innerhalb eines doppelt-quoted Strings mit Variablen-Ersatz
    InInterpolation,  // Innerhalb einer ${...} oder "{$var}"-Interpolation
}

pub struct Lexer {
    pub position: Position,
    pub bytes: Arc<[u8]>,
    pub tokens: Vec<Token>,
    pub context: Vec<LexerContext>
}

/* constructor */
impl Lexer {
    pub fn new(bytes: Arc<[u8]>) -> Lexer {
        Self {
            position: Position {
                line: 1, line_byte: 1, byte_offset: 0,
            },
            bytes,
            tokens: vec![],
            context: vec![],
        }
    }
}

/* Bytes operations */
pub trait BytesOperation {
    fn current(&mut self) -> Option<u8>;
    fn before_consume(&mut self);
    fn consume(&mut self) -> Option<u8>; //ein byte konsumieren
    fn consume_n(&mut self, n: usize) -> Option<u8>; //mehrere bytes konsumieren
    fn look(&mut self) -> Option<u8>; //nächstes zeichen schonmal anschauen
    fn look_n(&mut self, n: usize) -> Option<u8>; //nächste n zeichen ins Vorausschauen

    unsafe fn string_from_range_unchecked(&self, start: usize, end: usize) -> String;
    fn string_from_range(&self, start: usize, end: usize) -> String;
}
impl BytesOperation for Lexer {
    #[inline(always)]
    fn current(&mut self) -> Option<u8> {
        self.look_n(0)
    }

    fn before_consume(&mut self) {
        /* Check current line and byte pos */
        if self.current() == Some(b'\n') {
            self.position.line += 1;
            self.position.line_byte = 0;
        }
    }

    fn consume(&mut self) -> Option<u8> {
        self.before_consume();
        self.position.byte_offset += 1;
        self.position.line_byte += 1;
        self.current()
    }

    fn consume_n(&mut self, n: usize) -> Option<u8> {
        for _ in 1..=n {
            self.consume()?;
        }
        self.current()
    }

    fn look(&mut self) -> Option<u8> {
        self.look_n(1)
    }

    fn look_n(&mut self, n: usize) -> Option<u8> {
        self.bytes.get(self.position.byte_offset+n).cloned()
    }

    unsafe fn string_from_range_unchecked(&self, start: usize, end: usize) -> String { 
        unsafe {
            String::from_utf8_unchecked(self.bytes[start..end].to_vec())
        }
    }

    fn string_from_range(&self, start: usize, end: usize) -> String {
        todo!()
    }
}

/* main tokenizing operation */
pub trait Tokenizer {
    fn tokenize(&mut self) -> Vec<Token>;
    fn handle_context(&mut self);
    fn push_token(&mut self, tag: TokenTag, start_position: Position);
}
impl Tokenizer for Lexer {
    fn tokenize(&mut self) -> Vec<Token> {
        /* Main loop for tokenization */
        loop {
            self.handle_context();
            if self.look() == None {
              break;
            }
        }
        /* <-- datei zuende --> */
        self.tokens.clone()
    }

    fn handle_context(&mut self) {
        match self.context.last() {
            Some(LexerContext::InHtml) => self.handle_html(),
            Some(LexerContext::InPhp) => self.handle_php(),
            Some(LexerContext::InCommentLine) => self.handle_comment_line(),
            Some(LexerContext::InCommentBlock) => self.handle_comment_block(),
            Some(LexerContext::InString) => self.handle_string(),
            _ => {
                self.handle_php()
            }
        }
    }

    fn push_token(&mut self, tag: TokenTag, start_position: Position) {
        self.tokens.push(
            Token::new(tag, start_position, self.position.clone()) //todo: (+) methode die eine individuelle endposition zulässt.
        )
    }
}



































