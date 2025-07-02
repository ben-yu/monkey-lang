#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Integer(String),
    Illegal,
    Eof,
    Equal,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

