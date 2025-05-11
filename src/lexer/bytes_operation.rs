use crate::lexer::lexer::Lexer;

/* Bytes operations */


pub trait BytesOperation {
    fn current(&mut self) -> Option<u8>;
    fn consume(&mut self) -> Option<u8>; //ein byte konsumieren
    fn consume_n(&mut self, n: usize) -> Option<u8>; //mehrere bytes konsumieren
    fn look(&mut self) -> Option<u8>; //nächstes zeichen schonmal anschauen
    fn look_n(&mut self, n: usize) -> Option<u8>; //nächste n zeichen ins Vorausschauen

    unsafe fn strquick(&self, start: usize, end: usize) -> String;
    fn strrng(&self, start: usize, end: usize) -> String;

    fn remaining(&self) -> &[u8];

}
impl BytesOperation for Lexer {
    #[inline(always)]
    fn current(&mut self) -> Option<u8> {
        self.look_n(0)
    }

    #[inline(always)]
    fn consume(&mut self) -> Option<u8> {
        self.byte_offset += 1;
        self.current()
    }

    #[inline(always)]
    fn consume_n(&mut self, n: usize) -> Option<u8> {
        for _ in 1..=n {
            self.consume()?;
        }
        self.current()
    }

    #[inline(always)]
    fn look(&mut self) -> Option<u8> {
        self.look_n(1)
    }

    #[inline(always)]
    fn look_n(&mut self, n: usize) -> Option<u8> {
        self.bytes.get(self.byte_offset+n).cloned()
    }

    #[inline(always)]
    unsafe fn strquick(&self, start: usize, end: usize) -> String {
        if start > end {
            return "".to_string()
        }
        unsafe {
            std::str::from_utf8_unchecked(&self.bytes[start..=end]).to_string()
        }
    }
    
    #[inline(always)]
    fn strrng(&self, start: usize, end: usize) -> String {
        if start > end {
            return "".to_string()
        }
        todo!()
    }

    #[inline(always)]
    fn remaining(&self) -> &[u8] {
        &self.bytes[self.byte_offset..]
    }

}