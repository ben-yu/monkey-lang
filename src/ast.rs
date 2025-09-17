use std::fmt;
use crate::token::Token;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expr(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(id, expr) => write!(f, "let {} = {};", id, expr),
            Statement::Return(expr) => write!(f, "return {};", expr),
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

#[derive(Debug)]
pub enum Expression {
    Ident(String),
    Lit(Literal),
    Prefix(Token, Box<Expression>),
    Infix(Token, Box<Expression>, Box<Expression>),
    If(Box<Expression>, BlockStatement, Option<BlockStatement>),
    Function(Vec<String>, BlockStatement),
    FunctionCall(Box<Expression>, Vec<Expression>),
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
            Expression::Function(params, _block) => write!(f, "fn({}) {{...}}", params.join(", "),),
            Expression::FunctionCall(fn_expr, args) => {
                write!(f, "{}({})", fn_expr, format_expressions(args))
            }
        }
    }
}

fn format_expressions(expressions: &[Expression]) -> String {
    expressions
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

#[derive(Debug)]
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

