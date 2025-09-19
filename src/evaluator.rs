use std::fmt;
use crate::ast::*;
use crate::object::*;
use crate::token::*;
use crate::environment::*;
use std::rc::Rc;
use std::cell::RefCell;


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

fn is_truthy(obj: &Object) -> bool {
    match *obj {
        Object::Null => false,
        Object::Boolean(false) => false,
        _ => true,
    }
}

pub fn eval(node: Node, env: &Env) -> Result<Object, EvalError> {
    match node {
        Node::Program(program) => eval_program(&program, env),
        Node::Stmt(stmt) => eval_statement(&stmt, env),
        Node::Expr(expr) => eval_expression(&expr, env),
    }
}

fn eval_program(program: &[Statement], env: &Env) -> Result<Object, EvalError> {
    let mut result = Object::Null;

    for stmt in program {
        let val = eval_statement(stmt, &Rc::clone(env))?;

        match val {
            Object::ReturnValue(_) => return Ok(val),
            _ => result = val,
        }
    }

    Ok(result)
}

fn eval_block_statement(stmts: &[Statement], env: &Env) -> Result<Object, EvalError> {
    let mut result = Object::Null;

    for stmt in stmts {
        let val = eval_statement(stmt, &Rc::clone(env))?;

        match val {
            Object::ReturnValue(_) => return Ok(val),
            _ => result = val,
        }
    }

    Ok(result)
}

fn eval_statement(stmt: &Statement, env: &Env) -> Result<Object, EvalError> {
    match stmt {
        Statement::Let(id, expr) => {
            let val = eval_expression(expr, &Rc::clone(env))?;
            let obj = Rc::new(val.clone());
            env.borrow_mut().set(id.clone(), obj);
            Ok(val)
        }
        Statement::Expr(expr) => eval_expression(expr, env),
        Statement::Return(expr) => {
            let val = eval_expression(expr, env)?;
            Ok(Object::ReturnValue(val.into()))
        }

    }
}


fn eval_expression(expr: &Expression, env: &Env) -> Result<Object, EvalError> {
    match expr {
        Expression::Ident(id) => eval_identifier(&id, env),
        Expression::Lit(lit) => eval_literal(lit),
        Expression::Prefix(op, expr) => {
            let right = eval_expression(expr, env)?;
            eval_prefix_expression(op, &right)
        },
        Expression::Infix(op, left, right) => {
            let left = eval_expression(left, &Rc::clone(env))?;
            let right = eval_expression(right, &Rc::clone(env))?;
            eval_infix_expression(op, &left, &right)
        },
        Expression::If(condition, consequence, alternative) => {
            let condition = eval_expression(condition, &Rc::clone(env))?;

            if is_truthy(&condition) {
                eval_block_statement(consequence, env)
            } else {
                match alternative {
                    Some(alt) => eval_block_statement(alt, env),
                    None => Ok(Object::Null),
                }
            }
        },
        Expression::Function(params, body) => Ok(Object::Function(
            params.clone(),
            body.clone(),
            Rc::clone(&env),
        )),
        Expression::FunctionCall(func, args) => {
            let func = eval_expression(func, &Rc::clone(env))?;
            let args: Result<Vec<Object>, EvalError> = args.iter().map(|arg| eval_expression(arg, env)).collect();
            apply_function(&func, &args?)
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

fn eval_infix_expression(op: &Token, left: &Object, right: &Object) -> Result<Object, EvalError> {
    match (&*left, &*right) {
        (Object::Integer(left_val), Object::Integer(right_val)) => {
            eval_integer_infix_expression(op, *left_val, *right_val)
        }
        (Object::Boolean(left_val), Object::Boolean(right_val)) => {
            eval_boolean_infix_expression(op, *left_val, *right_val)
        }
        _ => Err(EvalError::new(format!(
            "type mismatch: {} {} {}",
            left, op, right
        ))),
    }
}

fn eval_integer_infix_expression(op: &Token, left_val: i32, right_val: i32) -> Result<Object, EvalError> {
    let result = match op {
        Token::Plus => Object::Integer(left_val + right_val),
        Token::Dash => Object::Integer(left_val - right_val),
        Token::Asterisk => Object::Integer(left_val * right_val),
        Token::ForwardSlash => Object::Integer(left_val / right_val),
        Token::LessThan => Object::Boolean(left_val < right_val),
        Token::GreaterThan => Object::Boolean(left_val > right_val),
        Token::Equal => Object::Boolean(left_val == right_val),
        Token::NotEqual => Object::Boolean(left_val != right_val),
        op => {
            return Err(EvalError::new(format!(
                "unknown operator: {} {} {}",
                left_val, op, right_val
            )))
        }
    };

    Ok(result)
}

fn eval_boolean_infix_expression(op: &Token, left_val: bool, right_val: bool) -> Result<Object, EvalError> {
    let result = match op {
        Token::Equal => Object::Boolean(left_val == right_val),
        Token::NotEqual => Object::Boolean(left_val != right_val),
        op => {
            return Err(EvalError::new(format!(
                "unknown operator: {} {} {}",
                left_val, op, right_val
            )))
        }
    };

    Ok(result)
}

fn apply_function(function: &Object, args: &[Object]) -> Result<Object, EvalError> {
    match function {
        Object::Function(params, body, env) => {
            let mut env = Environment::new_enclosed_environment(env);

            if params.len() != args.len() {
                return Err(EvalError::new(format!(
                    "invalid number of arguments: exected={}, got={}",
                    params.len(),
                    args.len()
                )));
            }

            params.iter().enumerate().for_each(|(i, param)| {
                env.set(param.clone(), Rc::new(args[i].clone()));
            });

            let evaluated = eval_block_statement(&body, &Rc::new(RefCell::new(env)))?;
            unwrap_return_value(Rc::new(evaluated))
        }
        f => Err(EvalError::new(format!("not a function: {}", f))),
    }
}

fn unwrap_return_value(obj: Rc<Object>) -> Result<Object, EvalError> {
    if let Object::ReturnValue(val) = &*obj {
        Ok((**val).clone())
    } else {
        Ok((*obj).clone())
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

fn eval_identifier(id: &str, env: &Env) -> Result<Object, EvalError> {
    match env.borrow().get(id) {
        Some(obj) => Ok((*obj).clone()),
        None => Err(EvalError::new(format!("identifier not found: {}", id))),
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
        let env: Env = Rc::new(Default::default());

        for (input, expected) in test_case {
            match parse(input) {
                Ok(node) => match eval(node, &Rc::clone(&env)) {
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
            ("5 + 5 + 5 + 5 - 10", "10"),
            ("2 * 2 * 2 * 2 * 2", "32"),
            ("-50 + 100 + -50", "0"),
            ("5 * 2 + 10", "20"),
            ("5 + 2 * 10", "25"),
            ("20 + 2 * -10", "0"),
            ("50 / 2 * 2 + 10", "60"),
            ("2 * (5 + 10)", "30"),
            ("3 * 3 * 3 + 10", "37"),
            ("3 * (3 * 3) + 10", "37"),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", "50"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_boolean_expressions() {
        let test_case = [
            ("true", "true"),
            ("false", "false"),
            ("1 < 2", "true"),
            ("1 > 2", "false"),
            ("1 < 1", "false"),
            ("1 > 1", "false"),
            ("1 == 1", "true"),
            ("1 != 1", "false"),
            ("1 == 2", "false"),
            ("1 != 2", "true"),
            ("true == true", "true"),
            ("false == false", "true"),
            ("true == false", "false"),
            ("true != false", "true"),
            ("false != true", "true"),
            ("(1 < 2) == true", "true"),
            ("(1 < 2) == false", "false"),
            ("(1 > 2) == true", "false"),
            ("(1 > 2) == false", "true"),
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

    #[test]
    fn test_if_else_expressions() {
        let test_case = [
            ("if (true) { 10 }", "10"),
            ("if (false) { 10 }", "null"),
            ("if (1) { 10 }", "10"),
            ("if (1 < 2) { 10 }", "10"),
            ("if (1 > 2) { 10 }", "null"),
            ("if (1 > 2) { 10 } else { 20 }", "20"),
            ("if (1 < 2) { 10 } else { 20 }", "10"),
        ];
        apply_test(&test_case);
    }

    #[test]
    fn test_return_statements() {
        let test_case = [
            ("return 10;", "10"),
            ("return 10; 9;", "10"),
            ("return 2 * 5; 9;", "10"),
            ("9; return 2 * 5; 9;", "10"),
            (
                "if (10 > 1) { \
                 if (10 > 1) { \
                 return 10; \
                 } \
                 return 1; \
                 }",
                "10",
            ),
        ];
        apply_test(&test_case);
    }

    #[test]
    fn test_error_handling() {
        let test_case = [
            ("5 + true;", "type mismatch: 5 + true"),
            ("5 + true; 5;", "type mismatch: 5 + true"),
            ("-true", "unknown operator: -true"),
            ("true + false;", "unknown operator: true + false"),
            (
                "true + false + true + false;",
                "unknown operator: true + false",
            ),
            ("5; true + false; 5", "unknown operator: true + false"),
            (
                "if (10 > 1) { true + false; )",
                "unknown operator: true + false",
            ),
        ];
        apply_test(&test_case);
    }

    #[test]
    fn test_let_statements() {
        let test_case = [
            ("let a = 5; a;", "5"),
            ("let a = 5 * 5; a;", "25"),
            ("let a = 5; let b = a; b;", "5"),
            ("let a = 5; let b = a; let c = a + b + 5; c;", "15"),
        ];
        apply_test(&test_case);
    }

    #[test]
    fn test_function_object() {
        let test_case = [("fn(x) { x + 2; };", "fn(x) {...}")];
        apply_test(&test_case);
    }

    #[test]
    fn test_function_application() {
        let test_case = [
            ("let identity = fn(x) { x; }; identity(5);", "5"),
            ("let identity = fn(x) { return x; }; identity(5);", "5"),
            ("let double = fn(x) { x * 2; }; double(5);", "10"),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", "10"),
            (
                "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
                "20",
            ),
            ("fn(x) { x; }(5)", "5"),
        ];
        apply_test(&test_case);
    }

    #[test]
    fn test_enclosing_environment() {
        let test_case = [(
            "let first = 10; \
             let second = 10; \
             let third = 10; \
             let ourFunction = fn(first) { \
             let second = 20; \
             first + second + third; \
             }; \
             ourFunction(20) + first + second;",
            "70",
        )];
        apply_test(&test_case);
    }

    #[test]
    fn test_closure() {
        let test_case = [(
            "let newAdder = fn(x) { \
             fn(y) { x + y }; \
             }; \
             let addTwo = newAdder(2); \
             addTwo(2);",
            "4",
        )];
        apply_test(&test_case);
    }
}


