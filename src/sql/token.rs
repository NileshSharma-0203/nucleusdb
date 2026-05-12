#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Select,
    From,
    Where,

    Insert,
    Into,
    Values,

    Create,
    Table,

    IntType,
    TextType,
    BoolType,
    FloatType,

    Identifier(String),
    Integer(i64),
    String(String),

    Star,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    Equals,
    Explain,
    Begin,
Commit,
Rollback,

    EOF,
}