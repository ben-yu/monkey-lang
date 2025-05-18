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

    pub fn next_token(&mut self) -> Result<Token> {
        let tok = match self.cur_char {
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            b'=' => Token::Equal,
            0 => Token::Eof,
            _ => todo!("Implement later")
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
            Token::Lparen,
            Token::Rparen,
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
}
