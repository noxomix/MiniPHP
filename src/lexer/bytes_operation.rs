use crate::lexer::lexer::Lexer;

/* Bytes operations */


pub trait BytesOperation {
    fn current(&mut self) -> Option<u8>;
    fn before_consume(&mut self);
    fn consume(&mut self) -> Option<u8>; //ein byte konsumieren
    fn consume_n(&mut self, n: usize) -> Option<u8>; //mehrere bytes konsumieren
    fn look(&mut self) -> Option<u8>; //nächstes zeichen schonmal anschauen
    fn look_n(&mut self, n: usize) -> Option<u8>; //nächste n zeichen ins Vorausschauen

    unsafe fn strquick(&self, start: usize, end: usize) -> String;
    fn strrng(&self, start: usize, end: usize) -> String;
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

    unsafe fn strquick(&self, start: usize, end: usize) -> String {
        if start > end {
            return "".to_string()
        }
        unsafe {
            String::from_utf8_unchecked(self.bytes[start..=end].to_vec())
        }
    }

    fn strrng(&self, start: usize, end: usize) -> String {
        if start > end {
            return "".to_string()
        }
        todo!()
    }
}