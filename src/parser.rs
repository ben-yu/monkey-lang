use crate::token::Token;
use crate::lexer::Lexer;
use crate::ast::*;
use std::fmt;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let errors = Vec::new();

        Parser {
            lexer,
            current_token,
            peek_token,
            errors,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Result<Program, ParserErrors> {
        let mut program = Program::new();

        while !self.current_token_is(&Token::Eof) {
            match self.parse_statement() {
                Ok(statement) => program.push(statement),
                Err(e) => self.errors.push(e),
            }
            self.next_token();
        }

        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let ident = match &self.peek_token {
            Token::Ident(id) => id.clone(),
            t => {
                return Err(self.error_no_identifier(t));
            }
        };

        // Consume identifier
        self.next_token();

        self.expect_peek_token(&Token::Assign)?;

        self.next_token();

        // skip expressions for now

        while !self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        // consume the semicolon
        self.next_token();

        Ok(Statement::Let(ident))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        // skip expressions for now

        while !self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        // consume the semicolon
        self.next_token();

        Ok(Statement::Return)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.parse_expression()?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Expr(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let mut left_expr = match self.current_token {
            Token::Ident(ref id) => Ok(Expression::Ident(id.clone())),
            Token::Integer(i) => Ok(Expression::Lit(Literal::Integer(i))),
            _ => {
                return Err(ParserError::new(format!(
                    "No prefix parse function for {} is found",
                    self.current_token
                )));
            }
        };

        left_expr
    }

    fn current_token_is(&self, token: &Token) -> bool {
        self.current_token == *token
    }

    fn peek_token_is(&self, token: &Token) -> bool {
        self.peek_token == *token
    }

    fn expect_peek_token(&mut self, token: &Token) -> Result<(), ParserError> {
        if self.peek_token_is(&token) {
            self.next_token();
            Ok(())
        } else {
            Err(ParserError::new(format!(
                "expected next token to be {}, but got {} instead",
                token, self.peek_token
            )))
        }
    }

    fn error_no_identifier(&self, token: &Token) -> ParserError {
        ParserError::new(format!("Expected an identifier but got {}", token.clone()))
    }
}

pub fn parse(input: &str) -> Result<Node, ParserErrors> {
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    Ok(Node::Program(program))
}

pub type ParserErrors = Vec<ParserError>;

#[derive(Debug, Clone)]
pub struct ParserError(String);

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ParserError {
    pub fn new(msg: String) -> Self {
        ParserError(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply_test(test_case: &[(&str, &str)]) {
        for (input, expected) in test_case {
            match parse(input) {
                Ok(node) => assert_eq!(expected, &format!("{}", node)),
                Err(e) => panic!("Parsing Error: {:#?}", e),
            }
        }
    }

    #[test]
    fn test_let_statement() {
        let test_case = [
            ("let x = 5;", "let x = ;"),
            ("let y = true;", "let y = ;"),
            ("let foobar = 124214;", "let foobar = ;"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_return_statement() {
        let test_case = [
            ("return 5;", "return;"),
            ("return true;", "return;"),
            ("return foobar;", "return;"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_identifier_expression() {
        let test_case = [("foobar;", "foobar")];

        apply_test(&test_case);
    }

    #[test]
    fn test_integer_literal_expression() {
        let test_case = [("5;", "5")];

        apply_test(&test_case);
    }
}
