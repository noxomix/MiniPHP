use std::cell::RefCell;
use std::sync::Arc;
use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::token::{DebugPosition, Token, TokenTag};

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
    //pub position: DebugPosition,
    pub byte_offset: usize, //direkt im struct, weil DebugPosition auch später noch errechnet werden kann.
    pub bytes: Arc<[u8]>,
    pub tokens: Vec<Token>,
    pub context: Vec<LexerContext>
}

/* constructor */
impl Lexer {
    pub fn new(bytes: Arc<[u8]>) -> Lexer {
        Self {
            byte_offset: 0,
            bytes,
            tokens: vec![],
            context: vec![],
        }
    }
}

/* main tokenizing operation */
pub trait Tokenizer {
    fn tokenize(&mut self) -> Vec<Token>;
    fn handle_context(&mut self);
    fn push_token(&mut self, tag: TokenTag, start_position: usize);
}
impl Tokenizer for Lexer {
    fn tokenize(&mut self) -> Vec<Token> {
        /* Main loop for tokenization */
        loop {
            self.handle_context();
            if self.look() == None {
              break;
            }
            //todo: if self.errors ... break and debug
        }
        /* <-- datei zuende --> */
        self.tokens.clone()
    }
    
    fn handle_context(&mut self) {
        match self.context.last() {
            Some(LexerContext::InHtml) => self.context_in_html(),
            Some(LexerContext::InPhp) => self.context_in_php(),
            Some(LexerContext::InCommentLine) => self.context_in_comment_line(),
            Some(LexerContext::InCommentBlock) => self.context_in_comment_block(),
            Some(LexerContext::InString) => self.context_in_dq_string(),
            _ => {
                self.context_in_html()
            }
        }
    }

    fn push_token(&mut self, tag: TokenTag, start_position: usize) {
        self.tokens.push(
            Token::new(tag, start_position, self.byte_offset) //todo: (+) methode die eine individuelle endposition zulässt.
        )
    }
}



































