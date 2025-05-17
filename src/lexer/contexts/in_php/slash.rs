use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext, Tokenizer};
use crate::lexer::token::TokenTag;

impl Lexer {
    /*
        Wenn diese function true retuned heißt das die main function soll auch returnen. (z.B. Kontextwechsel).
     */
    #[inline(always)]
    pub fn match_slash(&mut self) -> bool {
        match self.look() {
            Some(b'/') => {
                self.context.push(LexerContext::InCommentLine);
                return true;
            }
            Some(b'*') => {
                self.context.push(LexerContext::InCommentBlock);
                //println!("In comment block {:?}", self.byte_offset);
                return true;
                //self.handle_comment_block()
            }
            Some(b'=') => {
                let start = self.byte_offset;
                self.next();
                self.push_token(TokenTag::DivAssign, start);
            }
            _ => {
                self.push_token(TokenTag::Division, self.byte_offset);
            }
        }
        return false;
    }
}