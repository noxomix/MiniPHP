use std::sync::Arc;
use crate::lexer::token::{Position, Token};

pub struct Tokenizer {
    pub position: Position,
    pub bytes: Arc<[u8]>,
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(bytes: Arc<[u8]>) -> Tokenizer {
        Self {
            position: Position {
                line: 0, column: 0, byte_offset: 0,
            },
            bytes,
            tokens: vec![],
        }
    }
}

impl BytesOperation for Tokenizer {
    #[inline(always)]
    fn current(&mut self) -> Option<u8> {
        self.look_n(0)
    }

    fn before_consume(&mut self) {
        /* Check current line and byte pos */
        if self.current() == Some(b'\n') {
            self.position.line += 1;
        }
    }

    fn consume(&mut self) -> Option<u8> {
        self.before_consume();
        self.position.byte_offset += 1;
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
}

trait BytesOperation {
    fn current(&mut self) -> Option<u8>;
    fn before_consume(&mut self);
    fn consume(&mut self) -> Option<u8>; //ein byte konsumieren
    fn consume_n(&mut self, n: usize) -> Option<u8>; //mehrere bytes konsumieren
    fn look(&mut self) -> Option<u8>; //nächstes zeichen schonmal anschauen
    fn look_n(&mut self, n: usize) -> Option<u8>; //nächste n zeichen ins Vorausschauen
}