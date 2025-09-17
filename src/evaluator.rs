use std::fmt;
use crate::ast::*;
use crate::object::*;
use crate::token::*;

#[derive(Debug, Clone)]
pub struct EvalError(String);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl EvalError {
    pub fn new(msg: String) -> Self {
        EvalError(msg)
    }
}


pub fn eval(node: Node) -> Result<Object, EvalError> {
    match node {
        Node::Program(program) => eval_program(&program),
        Node::Expr(expr) => eval_expression(&expr),
        _ => Err(EvalError::new(format!(
            "unknown node: {}",
            node
        ))),

    }
}

fn eval_program(program: &[Statement]) -> Result<Object, EvalError> {
    let mut result = Object::Null;

    for stmt in program {
        result = eval_statement(stmt)?;
    }

    Ok(result)
}

fn eval_statement(stmt: &Statement) -> Result<Object, EvalError> {
    match stmt {
        Statement::Expr(expr) => eval_expression(expr),
        _ => Err(EvalError::new(format!(
            "unknown statement: {}",
            stmt
        ))),

    }
}


fn eval_expression(expr: &Expression) -> Result<Object, EvalError> {
    match expr {
        Expression::Lit(lit) => eval_literal(lit),
        Expression::Prefix(op, expr) => {
            let right = eval_expression(expr)?;
            eval_prefix_expression(op, &right)
        }
        _ => Err(EvalError::new(format!(
            "unknown expression: {}",
            expr
        ))),
    }
}

fn eval_prefix_expression(op: &Token, expr: &Object) -> Result<Object, EvalError> {
    match op {
        Token::Bang => eval_bang_operator(expr),
        Token::Dash => eval_minus_prefix_operator(expr),
        _ => Err(EvalError::new(format!(
            "unknown operator: {}{}",
            op, expr
        ))),
    }
}

fn eval_bang_operator(expr: &Object) -> Result<Object, EvalError> {
    match *expr {
        Object::Boolean(b) => Ok(Object::Boolean(!b)),
        Object::Null => Ok(Object::Boolean(true)),
        _ => Ok(Object::Boolean(false)),
    }
}

fn eval_minus_prefix_operator(expr: &Object) -> Result<Object, EvalError> {
    match *expr {
        Object::Integer(b) => Ok(Object::Integer(-b)),
        _ => Err(EvalError::new(format!(
            "unknown operator: -{}",
            expr
        ))),
    }
}


fn eval_literal(lit: &Literal) -> Result<Object, EvalError> {
    match lit {
        Literal::Integer(i) => Ok(Object::Integer(*i)),
        Literal::Boolean(b) => Ok(Object::Boolean(*b)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    fn apply_test(test_case: &[(&str, &str)]) {
        for (input, expected) in test_case {
            match parse(input) {
                Ok(node) => match eval(node) {
                    Ok(eval_result) => assert_eq!(expected, &format!("{}", eval_result)),
                    Err(err) => assert_eq!(expected, &format!("{}", err)),
                },
                Err(e) => panic!("Parsing Error: {:#?}", e),
            }
        }
    }

    #[test]
    fn test_integer_expressions() {
        let test_case = [
            ("5", "5"),
            ("10", "10"),
            ("-5", "-5"),
            ("-10", "-10"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_boolean_expressions() {
        let test_case = [
            ("true", "true"),
            ("false", "false"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_bang_operator() {
        let test_case = [
            ("!true", "false"),
            ("!false", "true"),
            ("!5", "false"),
            ("!!true", "true"),
            ("!!false", "false"),
            ("!!5", "true"),
        ];
        apply_test(&test_case);
    }

}


