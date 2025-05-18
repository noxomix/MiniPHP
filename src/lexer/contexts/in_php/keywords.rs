use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::{AccessModifierType, StatementType, TokenTag};

impl Lexer {
    #[inline(always)]
    pub fn match_keywords(&mut self) {
        let start = self.byte_offset;

        if let Some(tag) = self._find_keyword() {
            self.push_token(tag, start);
            return;
        }

        let mut lookahead = 0;
        let mut last_valid = None;

        while let Some(b) = self.peek_n(lookahead) {
            if b.is_ascii_alphanumeric() || b == b'_' {
                last_valid = Some(self.byte_offset + lookahead);
                lookahead += 1;
            } else if b >= 0x80 {
                let len = Self::utf8_char_len(b);
                if len == 0 || self.byte_offset + lookahead + len > self.bytes.len() {
                    break;
                }
                last_valid = Some(self.byte_offset + lookahead + len - 1);
                lookahead += len;
            } else {
                break;
            }
        }

        if let Some(end) = last_valid {
            let ident = unsafe { self.strquick(start, end) }.to_string();
            self.next_n(end - self.byte_offset); // nur bis **vor** dem letzten Zeichen
            self.push_token(TokenTag::Identifier(ident), start);
            // self.current() zeigt jetzt korrekt auf das letzte gültige Zeichen
        }
    }

    fn _find_keyword(&mut self) -> Option<TokenTag> {
        let remaining = self.remaining();

        for &(word, ref tag) in KEYWORDS {
            if remaining.len() < word.len() {
                continue;
            }

            // Zeichen danach darf kein a-zA-Z0-9_ sein → sonst z. B. "returnValue"
            if let Some(&b) = remaining.get(word.len()) {
                if b.is_ascii_alphanumeric() || b == b'_' {
                    continue;
                }
            }

            let candidate = &remaining[..word.len()];

            let equal = candidate
                .iter()
                .zip(word.iter())
                .all(|(a, b)| a.to_ascii_lowercase() == *b);

            if equal {
                self.next_n(word.len()-1); //nicht das letzte zeichen schon konsumieren :)
                return Some(tag.clone());
            }
        }
        None
    }
}

// Reihenfolge nach Anfangsbuchstabe gruppiert, dann nach Länge sortiert
pub const KEYWORDS: &[(&[u8], TokenTag)] = &[
    // Literals
    (b"null", TokenTag::NullLiteral),
    (b"false", TokenTag::BooleanLiteral(false)),
    (b"true", TokenTag::BooleanLiteral(true)),

    // Statements
    (b"break", TokenTag::Statement(StatementType::Break)),
    (b"case", TokenTag::Statement(StatementType::Case)),
    (b"class", TokenTag::Statement(StatementType::Class)),
    (b"new", TokenTag::Statement(StatementType::New)),
    (b"continue", TokenTag::Statement(StatementType::Continue)),
    (b"do", TokenTag::Statement(StatementType::Do)),
    (b"else", TokenTag::Statement(StatementType::Else)),
    (b"elseif", TokenTag::Statement(StatementType::ElseIf)),
    (b"for", TokenTag::Statement(StatementType::For)),
    (b"foreach", TokenTag::Statement(StatementType::Foreach)),
    (b"function", TokenTag::Statement(StatementType::Function)),
    (b"if", TokenTag::Statement(StatementType::If)),
    (b"return", TokenTag::Statement(StatementType::Return)),
    (b"switch", TokenTag::Statement(StatementType::Switch)),
    (b"trait", TokenTag::Statement(StatementType::Trait)),
    (b"while", TokenTag::Statement(StatementType::While)),
    (b"default", TokenTag::Statement(StatementType::DefaultCase)),
    (b"namespace", TokenTag::Statement(StatementType::Namespace)),
    (b"as", TokenTag::Statement(StatementType::As)),
    (b"use", TokenTag::Statement(StatementType::Use)),
    (b"global", TokenTag::Statement(StatementType::Global)),
    (b"const", TokenTag::Statement(StatementType::Const)),
    (b"goto", TokenTag::Statement(StatementType::Goto)),
    (b"yield", TokenTag::Statement(StatementType::Yield)),
    (b"yield from", TokenTag::Statement(StatementType::YieldFrom)),
    (b"eval", TokenTag::Statement(StatementType::Eval)),
    (b"empty", TokenTag::Statement(StatementType::Empty)),
    (b"isset", TokenTag::Statement(StatementType::Isset)),
    (b"unset", TokenTag::Statement(StatementType::Unset)),
    (b"exit", TokenTag::Statement(StatementType::Exit)),
    (b"print", TokenTag::Statement(StatementType::Print)),
    (b"echo", TokenTag::Statement(StatementType::Echo)),
    (b"include", TokenTag::Statement(StatementType::Include)),
    (b"include_once", TokenTag::Statement(StatementType::IncludeOnce)),
    (b"require", TokenTag::Statement(StatementType::Require)),
    (b"require_once", TokenTag::Statement(StatementType::RequireOnce)),
    (b"match", TokenTag::Statement(StatementType::Match)),
    (b"try", TokenTag::Statement(StatementType::Try)),
    (b"catch", TokenTag::Statement(StatementType::Catch)),
    (b"finally", TokenTag::Statement(StatementType::Finally)),
    (b"throw", TokenTag::Statement(StatementType::Throw)),

    // Access Modifier (getrennt behandelt)
    (b"public", TokenTag::AccessModifier(AccessModifierType::Public)),
    (b"private", TokenTag::AccessModifier(AccessModifierType::Private)),
    (b"protected", TokenTag::AccessModifier(AccessModifierType::Protected)),
    (b"static", TokenTag::AccessModifier(AccessModifierType::Static)),
    (b"final", TokenTag::AccessModifier(AccessModifierType::Final)),
    (b"abstract", TokenTag::AccessModifier(AccessModifierType::Abstract)),
    (b"readonly", TokenTag::AccessModifier(AccessModifierType::Readonly)),
    
    //Special Numbers
    (b"inf", TokenTag::NumberLiteral),
    (b"nan", TokenTag::NumberLiteral),
];