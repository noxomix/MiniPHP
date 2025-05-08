#[derive(Debug, Clone)]
pub enum TokenTag {
    TodoGleich,  //'='
    IsEqual,  //'=='
    IsIdentical, //'==='
    IsNotIdentical, //'!=='
    IsGreater, //'>'
    IsGreaterOrEqual, //'>='
    ShiftRight, // '>>'
    IsSmaller, //'<'
    IsSmallerOrEqual, //'<='
    ShiftLeft,  // '<<'
    IsNotEqual, // '!='
    StringLiteral {
        value: String,
        dq: bool, //if its double-quoted (only relevant for debugging)
    },
    Comment {
        value: String,
        multiline: bool,
    },
    DocComment {
        value: String,
    },
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize, //zeile
    pub line_byte: usize, //zeichen in zeile (in byte)
    pub byte_offset: usize, //byte position in file
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: TokenTag,
    pub start_position: Position,
    pub end_position: Position,
}

impl Token {
    pub fn new(tag: TokenTag, start_position: Position, end_position: Position) -> Self {
        Self { tag, start_position, end_position }
    }
}