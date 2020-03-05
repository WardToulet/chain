#[derive(Debug, PartialEq)]
pub enum LexToken {
    Arrow,
    Colon,
    SemiColon,

    ParanthesisOpen,
    ParanthesisClose,

    BracketOpen,
    BracketClose,

    BraceOpen,
    BraceClose,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    PlusPlus,
    MinusMinus,

    Equals,
    EqualsEquals,

    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    PercentEquals,

    GreatherThen,
    GreatherThenOrEqual,

    SmallerThen,
    SmallerThenOrEuqal,

    I8,
    I16,
    I32,
    I64,

    U8,
    U16,
    U32,
    U64,

    F32,
    F64,

    Boolean,

    Character,

    True,
    False,

    Let,
    If,
    Else,
    For,
    Return,

    Identifire(String),
}
