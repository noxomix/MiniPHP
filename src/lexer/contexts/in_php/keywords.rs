use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, Tokenizer};
use crate::lexer::token::{AccessModifierType, StatementType, TokenTag};

impl Lexer {
    #[inline(always)]
    pub fn match_keywords(&mut self) {
        let start_position = self.byte_offset;
        if let Some(tag) = self._find_keyword() {
            self.push_token(tag, start_position);
        } else {
            let mut len = 1; // wir wissen: das erste Zeichen ist gültig
            // Zähle alle folgenden gültigen Identifier-Zeichen
            while let Some(b) = self.look_n(len) {
                if b.is_ascii_alphanumeric() || b == b'_' {
                    len += 1;
                } else if b >= 0x80 {
                    let utf_len = Self::utf8_char_len(b);
                    if utf_len == 0 {
                        break;
                    }

                    // look_n gibt nur 1 Byte – prüfe, ob genug Daten da sind
                    if self.byte_offset + len + utf_len - 1 >= self.bytes.len() {
                        break;
                    }

                    len += utf_len;
                } else {
                    break;
                }
            }
            // Jetzt `len` gültige Bytes → konsumieren
            self.consume_n(len);
            // `byte_offset` zeigt auf das **letzte gültige Byte** → perfekt für inclusive strquick
            let ident = unsafe { self.strquick(start_position, self.byte_offset - 1) };
            self.push_token(TokenTag::Identifier(ident.to_string()), start_position);
            //println!("{:?}", ident.to_string());
        }
    }
    fn _find_keyword(&mut self) -> Option<TokenTag> {
        let remaining = self.remaining();

        for &(word, ref tag) in KEYWORDS {
            if remaining.len() < word.len() {
                continue;
            }

            // prüfe auf Bezeichnerende: nächstes Zeichen darf kein Buchstabe/Ziffer/_ sein
            if let Some(&b) = remaining.get(word.len()) {
                if b.is_ascii_alphanumeric() || b == b'_' {
                    continue;
                }
            }

            let candidate = &remaining[..word.len()];

            // effizient vergleichen: ASCII case-insensitive
            let equal = candidate
                .iter()
                .zip(word.iter())
                .all(|(a, b)| a.to_ascii_lowercase() == *b);

            if equal {
                self.consume_n(word.len());
                return Some(tag.clone());
            }
        }

        // Dynamische Sonderfälle (nicht in KEYWORDS enthalten)
        const DYNAMIC_NUMBERS: &[&[u8]] = &[b"inf", b"infinity", b"nan"];

        for &word in DYNAMIC_NUMBERS {
            if remaining.len() < word.len() {
                continue;
            }

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
                let literal = std::str::from_utf8(word).unwrap().to_ascii_lowercase();
                self.consume_n(word.len());
                return Some(TokenTag::NumberLiteral(literal));
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
];