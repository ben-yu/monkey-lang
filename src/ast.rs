use std::fmt;
use crate::token::Token;


pub enum Node {
    Program(Vec<Statement>),
    Stmt(Statement),
    Expr(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Program(stmts) => write!(f, "{}", format_statements(stmts)),
            Node::Stmt(stmt) => write!(f, "{}", stmt),
            Node::Expr(expr) => write!(f, "{}", expr),
        }
    }
}


pub type Program = Vec<Statement>;

pub enum Statement {
    Let(String),
    Return,
    Expr(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(id) => write!(f, "let {} = ;", id),
            Statement::Return => write!(f, "return;"),
            Statement::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

fn format_statements(stmts: &[Statement]) -> String {
    stmts
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub type BlockStatement = Vec<Statement>;

pub enum Expression {
    Ident(String),
    Lit(Literal),
    Prefix(Token, Box<Expression>),
    Infix(Token, Box<Expression>, Box<Expression>),
    If(Box<Expression>, BlockStatement, Option<BlockStatement>),
    Fn(Vec<String>, BlockStatement),

}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Ident(id) => write!(f, "{}", id),
            Expression::Lit(lit) => write!(f, "{}", lit),
            Expression::Prefix(op, expr) => write!(f, "({}{})", op, expr),
            Expression::Infix(op, left_expr, right_expr) => write!(f, "({} {} {})", left_expr, op, right_expr),
            Expression::If(cond, true_block, else_block) => {
                if let Some(else_block) = else_block {
                    write!(
                        f,
                        "if {} {{ {} }} else {{ {} }}",
                        cond,
                        format_statements(true_block),
                        format_statements(else_block)
                    )
                } else {
                    write!(f, "if {} {{ {} }}", cond, format_statements(true_block))
                }
            },
            Expression::Fn(params, _block) => write!(f, "fn({}) {{...}}", params.join(", "),)
        }
    }
}

pub enum Literal {
    Integer(i32),
    Boolean(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Integer(i) => write!(f, "{}", i),
            Literal::Boolean(b) => write!(f, "{}", b),
        }
    }
}

