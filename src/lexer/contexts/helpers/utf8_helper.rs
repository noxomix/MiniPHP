use crate::lexer::lexer::Lexer;

impl Lexer {
    #[inline(always)]
    pub(crate) fn utf8_char_len(first: u8) -> usize {
        match first {
            0x00..=0x7F => 1, // ASCII
            0xC2..=0xDF => 2, // 2-byte, gültig ab C2
            0xE0..=0xEF => 3, // 3-byte
            0xF0..=0xF4 => 4, // 4-byte (maximal F4 für gültiges Unicode)
            _ => 0,           // ungültiger Startbyte
        }
    }
}
