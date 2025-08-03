use std::fmt;

pub enum Node {
    Program(Vec<Statement>),
    Stmt(Statement),
    //Expr(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Program(stmts) => write!(f, "{}", format_statements(stmts)),
            Node::Stmt(stmt) => write!(f, "{}", stmt),
     //       Node::Expr(expr) => write!(f, "{}", expr),
        }
    }
}


pub type Program = Vec<Statement>;

pub enum Statement {
    Let(String),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(id) => write!(f, "let {};", id),
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

