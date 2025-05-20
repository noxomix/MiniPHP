mod variable;mod number;mod semicolon;mod single_q_string;mod slash;mod whitespace;mod equals;
mod braces;mod parens;mod backslash;mod brackets;mod and;mod strich;mod exclamation;mod than_symbols;mod dot;
mod doppelpunkt;mod questionmark;mod comma;mod minus;mod plus;mod star;mod percent;mod pivot;mod keywords;

use std::process::exit;
use crate::lexer::bytes_operation::BytesOperation;
use crate::lexer::lexer::{Lexer, LexerContext};
use crate::lexer::token::TokenTag;

impl Lexer {
    pub fn context_in_php(&mut self) {
        while let Some(c) = self.current() {
            match c {
                b'$' => {self.match_variable();}, //'$..'
                b'"' => {self.context.push(LexerContext::InString);return;}, //'"' double-quoted string
                b'0'..=b'9' => self.match_number(), // delegiert an eigene Funktion ,
                b'\'' => self.match_single_q_string(), //"'" single-quoted string
                b';' => self.match_semicolon(), //';'
                b'/' => if self.match_slash() == true {return;}, //kommentare und diff assign
                b'#' => {self.context.push(LexerContext::InCommentLine);return;}, //single line kommentar mit '#' (!)
                b' ' | b'\t' | b'\r' | b'\n' => self.match_whitespace(), //' ' (whitespace)
                b'=' => self.match_equals(),
                b'{' => self.match_left_brace(),
                b'}' => self.match_right_brace(),
                b'(' => self.match_left_paren(),
                b')' => self.match_right_paren(),
                b'[' => self.match_left_bracket(),
                b']' => self.match_right_bracket(),
                b'&' => self.match_and_symbol(), //'&|&&' logical and bitwise <AND>
                b'|' => self.match_strich(), //'|/||' logical and bitwise <OR>
                b'!' => self.match_exclamation(), //'!..'
                b'<' => self.match_less_than_symbol(), //'<..'
                b'>' => self.match_greater_than_symbol(), //'>..'
                b'.' => self.match_dot_symbol(), //'.'
                b':' => self.match_double_dot_symbol(), //':'
                b'?' => {if self.match_question_symbol() == true {return}} //'?..'
                b',' => self.match_comma(), //','
                b'-' => self.match_minus_symbol(),
                b'+' => self.match_plus_symbol(), //'+..'
                b'*' => self.match_star_symbol(), //'*..'
                b'%' => self.match_percent_symbol(), //'%..' (modulo)
                b'^' => self.match_pivot(), //'^' bitwise <XOR>
                b'\\' => self.match_backslash(),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.match_keywords(),
                _ => {
                    //debug: nicht gematchte zeichen:
                    println!("Not matched: {:?} - {:?}", c as char, self.byte_offset);
                    //debug some context for the not matched thing:
                    let mut count = 0;

                    // Sammle Identifier aus den letzten 200 Tokens rückwärts
                    for token in self.tokens.iter().rev().take(5000) {
                        count += 1;
                        if let TokenTag::Identifier(_) = token.tag {
                            println!("#{count} from end: {:?}", token);
                        }
                    }
                    println!("{:?}", unsafe {self.strquick(17200, 17300)});
                    exit(1);
                    println!("Context: {:?}", unsafe { self.strquick(self.byte_offset - 50, self.byte_offset + 50) });
                }
            }

            self.next();
        }
    }
}
