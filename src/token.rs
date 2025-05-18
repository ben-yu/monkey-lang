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
    Lparen,
    Rparen,
    LBrace,
    RBrace,
    Function,
    Let,
}

