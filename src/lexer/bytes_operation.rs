use crate::lexer::lexer::Lexer;

/* Bytes operations */


pub trait BytesOperation {
    fn current(&mut self) -> Option<u8>;
    fn next(&mut self) -> Option<u8>; //ein byte konsumieren
    fn next_n(&mut self, n: usize) -> Option<u8>; //mehrere bytes konsumieren
    fn peek(&mut self) -> Option<u8>; //nächstes zeichen schonmal anschauen
    fn peek_n(&mut self, n: usize) -> Option<u8>; //nächste n zeichen ins Vorausschauen
    fn exclusive_pos(&self) -> usize;
    fn reset_to(&mut self, pos: usize) -> Option<u8>;

    unsafe fn strquick(&self, start: usize, end: usize) -> String;
    fn strrng(&self, start: usize, end: usize) -> String;

    fn remaining(&self) -> &[u8];

}
impl BytesOperation for Lexer {
    #[inline(always)]
    fn current(&mut self) -> Option<u8> {
        self.peek_n(0)
    }

    #[inline(always)]
    fn next(&mut self) -> Option<u8> {
        self.next_n(1)
    }

    #[inline(always)]
    fn next_n(&mut self, n: usize) -> Option<u8> {
        self.byte_offset += n;
        self.current()
    }

    #[inline(always)]
    fn peek(&mut self) -> Option<u8> {
        self.peek_n(1)
    }

    #[inline(always)]
    fn peek_n(&mut self, n: usize) -> Option<u8> {
        let s = self.bytes.get(self.byte_offset+n);
        if s == None {
            return None;
        }
        s.cloned()
    }

    fn exclusive_pos(&self) -> usize {
        self.byte_offset+1
    }

    fn reset_to(&mut self, pos: usize) -> Option<u8> {
        self.byte_offset = pos;
        self.current()
    }

    #[inline(always)]
    unsafe fn strquick(&self, start: usize, end: usize) -> String {
        if start > end {
            return "".to_string()
        }
        unsafe {
            std::str::from_utf8_unchecked(&self.bytes[start..end]).to_string() //nicht mehr inklusive (achtung)
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