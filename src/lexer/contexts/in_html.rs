use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext, Tokenizer};
use crate::lexer::token::{TokenTag};

impl Lexer {
    pub fn context_in_html(&mut self) {
        let start_position = self.byte_offset;

        while let Some(b) = self.current() {
            if b == b'<' {
                let remaining = self.remaining();
                let php_start = self.byte_offset;

                if remaining.starts_with(b"<?php") {
                    if start_position < php_start {
                        self.push_token(TokenTag::HtmlLiteral {}, start_position);
                    }
                    self.next_n(4); // "<?php"
                    self.push_token(TokenTag::PhpOpenTag {}, php_start);
                    self.context.push(LexerContext::InPhp);
                    self.next();
                    return;
                } else if remaining.starts_with(b"<?=") { //short form for '<?php echo'
                    if start_position < php_start {
                        self.push_token(TokenTag::HtmlLiteral {}, start_position);
                    }
                    self.next_n(2); // "<?="
                    self.push_token(TokenTag::PhpOpenTag {}, php_start);
                    self.push_token(TokenTag::Identifier("echo".to_string()), php_start+2); //special case wir nutzen hier als position die selbe wie das '<?=' (gleichzeichen davon)
                    self.context.push(LexerContext::InPhp);
                    self.next();
                    return;
                } else if remaining.starts_with(b"<?") {
                    if start_position < php_start {
                        self.push_token(TokenTag::HtmlLiteral {}, start_position);
                    }
                    self.next_n(1); // "<?"
                    self.push_token(TokenTag::PhpOpenTag {}, php_start);
                    self.context.push(LexerContext::InPhp);
                    self.next();
                    return;
                }
            }
            self.next(); // weiter im HTML
        }

        // Falls kein PHP-Start gefunden wurde
        self.push_token(TokenTag::HtmlLiteral {}, start_position);
    }
}