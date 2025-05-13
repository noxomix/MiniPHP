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
                    self.push_token(TokenTag::HtmlLiteral {}, start_position);
                    self.push_token(TokenTag::PhpOpenTag {}, php_start);
                    self.consume_n(5); // "<?php"
                    self.context.push(LexerContext::InPhp);
                    return;
                } else if remaining.starts_with(b"<?=") { //short form for '<?php echo'
                    self.push_token(TokenTag::HtmlLiteral {}, start_position);
                    self.push_token(TokenTag::PhpOpenTag {}, php_start);
                    self.push_token(TokenTag::Identifier("echo".to_string()), php_start+2); //special case wir nutzen hier als position die selbe wie das '<?=' (gleichzeichen davon)
                    self.consume_n(3); // "<?="
                    self.context.push(LexerContext::InPhp);
                    return;
                } else if remaining.starts_with(b"<?") {
                    self.push_token(TokenTag::HtmlLiteral {}, start_position);
                    self.push_token(TokenTag::PhpOpenTag {}, php_start);
                    self.consume_n(2); // "<?"
                    self.context.push(LexerContext::InPhp);
                    return;
                }
            }
            self.consume(); // weiter im HTML
        }

        // Falls kein PHP-Start gefunden wurde
        self.push_token(TokenTag::HtmlLiteral {}, start_position);
    }
}