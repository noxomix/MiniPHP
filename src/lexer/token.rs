use strum_macros::AsRefStr;

#[derive(Debug, Clone, AsRefStr)]
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
        //value: String,
        double_quoted: bool, //if its double-quoted (only relevant for debugging)
    },
    HtmlLiteral {
      //  
    },
    Comment {
        //value: String,
        multiline: bool,
    },
    DocComment {
        //value: String,
    },
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
    NumberLiteral(String), //mathematik
    BooleanLiteral(bool),
    NullLiteral,
    Identifier(String),
    Whitespace, //' ' (whitespace sind auch \n und mehrere leerzeichen hintereinander, wir geben es nicht als string mit also muss man es bei bedarf selber parsen)
    Tab, //\t
    //Newline, //\n usw
    NamespaceBackslash, //'\\'
    NumberNan, //NaN
    NumberInfinity, //INF
    Statement(StatementType),
    PhpCloseTag,
    PhpOpenTag {},
    AccessModifier(AccessModifierType),
    StringInterpolation(String)
}
#[derive(Debug, Clone, PartialEq, Eq, AsRefStr)]
pub enum StatementType {
    // Kontrollfluss
    If,
    Else,
    ElseIf,
    Switch,
    Case,
    DefaultCase,
    Match,        // seit PHP 8.0

    // Schleifen
    Do,
    While,
    For,
    Foreach,
    Break,
    Continue,

    // Fehlerbehandlung
    Try,
    Catch,
    Finally,
    Throw,

    // Funktionen & Klassen
    Function,
    Return,
    Class,
    Trait,
    Interface,
    Abstract,
    Final,
    Static,
    Namespace,

    // Spezialblöcke / Meta
    Declare,
    Global,
    Const,
    Use,
    Goto,
    Yield,
    YieldFrom,

    // Ausdrucksbasierte Anweisungen
    Exit,
    Eval,
    Empty,
    Isset,
    Unset,

    // Sonstige häufige Sprachkonstrukte
    Print,
    Echo,
    Include,
    IncludeOnce,
    Require,
    RequireOnce,
    As,
    New,
}

#[derive(Debug, Clone, PartialEq, Eq, AsRefStr)]
pub enum AccessModifierType {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Abstract,
    Readonly,
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