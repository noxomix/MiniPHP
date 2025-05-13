use crate::lexer::lexer::Lexer;

impl Lexer {
    pub(crate) fn utf8_char_len(first: u8) -> usize {
        match first {
            0b0000_0000..=0b0111_1111 => 1, // ASCII
            0b1100_0000..=0b1101_1111 => 2,
            0b1110_0000..=0b1110_1111 => 3,
            0b1111_0000..=0b1111_0111 => 4,
            _ => 0, // ungültig (z. B. Fortsetzungsbytes 0b10xxxxxx)
        }
    }
}