#[derive(Debug, Clone)]
pub enum TokenTag {
    Assign,  //'='
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
        double_quoted: bool, //if its double-quoted (only relevant for debugging)
    },
    Comment {
        value: String,
        multiline: bool,
    },
    DocComment {
        value: String,
    },
    IfStatement, //if
    ElseIfStatement, //else if, elseif
    ElseStatement, //else
    DoStatement, //do
    WhileStatement, //while
    ForStatement, //for
    ForeachStatement, //foreach
    BreakStatement, //break
    ContinueStatement, //continue
    ReturnStatement, //return
    SwitchStatement, //switch
    CaseStatement, //case
    FunctionStatement, //function
    ClassStatement, //class
    TraitStatement, //trait
    Variable(String), //'$hallo'
    Plus, //'+'
    Minus, //'-'
    Division, //'/'
    Multiply, //'*'
    Modulo, //'%'
    Power, //'**'
    Increment, //'++'
    Decrement, // --
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
    ModAssign, // %=
    PowerAssign, // **=
    LogicalAnd,          // &&
    LogicalOr,           // ||
    AndKeyword,          // and
    OrKeyword,           // or
    XorKeyword,          // xor
    BitAnd,              // &
    BitOr,               // |
    BitXor,              // ^
    NullCoalesce,        // ??
    NullCoalesceAssign,  // ??=
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }
    LeftBracket,   // [
    RightBracket,  // ]
    Semicolon,     // ;
    Comma,         // ,
    Dot,           // .
    Colon,         // :
    DoubleColon,   // ::
    Arrow,         // ->
    FatArrow,      // =>
    ConcatAssign,  // .=
    TernaryQuestion, // ?
    TernaryColon,    // :
    NumberLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
    Identifier(String),
    Whitespace,
    Tab,
    Newline,
}

#[derive(Debug, Clone)]
pub struct DebugPosition {
    pub line: usize, //zeile
    pub line_byte: usize, //zeichen in zeile (in byte)
    pub byte_offset: usize, //byte position in file
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: TokenTag,
    pub start_position: usize,
    pub end_position: usize,
}

impl Token {
    #[inline(always)]
    pub fn new(tag: TokenTag, start_position: usize, end_position: usize) -> Self {
        Self { tag, start_position, end_position }
    }
}