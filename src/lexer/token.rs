#[derive(Debug)]
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
    }
}

#[derive(Debug)]
pub struct Position {
    pub line: usize, //zeile
    pub column: usize, //zeichen in zeile (in byte)
    pub byte_offset: usize, //byte position in file
}

#[derive(Debug)]
pub struct Token {
    pub tag: TokenTag,
    pub position: Position,
}