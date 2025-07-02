use crate::token::Token;
use anyhow::Result;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    cur_char: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            cur_char: 0,
        };
        lex.read_char();

        return lex;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.cur_char = 0;
        } else {
            self.cur_char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while Self::is_letter(self.cur_char) {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.cur_char.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn is_letter(ch: u8) -> bool {
        return ch.is_ascii_alphabetic() || ch == b'_';
    }

    pub fn next_token(&mut self) -> Result<Token> {

        self.skip_whitespace();

        let tok = match self.cur_char {
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            b'=' => Token::Equal,
            0 => Token::Eof,
            c => {
                if Self::is_letter(c) {
                    let id = self.read_identifier();
                    return Ok(match id.as_str() {
                        "fn" => Token::Function,
                        "let" => Token::Let,
                        _ => Token::Ident(id),
                    });
                } else if c.is_ascii_digit() {
                    let id = self.read_number();
                    return Ok(Token::Integer(id));
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        return Ok(tok);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() -> Result<()> {
        let input = "=+(){},;";

        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Equal,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn test_tokenize_program() -> Result<()> {
        let input = "let five = 5;\
                     let ten = 10;\
                     let add = fn(x, y) { x + y;}; \
                     let result = add(five, ten);";
        let mut lexer = Lexer::new(input.into());

        let expected = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Equal,
            Token::Integer("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Equal,
            Token::Integer("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Equal,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Equal,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Eof,
        ];

        for token in expected {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
        return Ok(());
    }
}
