use crate::ast::*;
use crate::environment::*;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
    ReturnValue(Rc<Object>),
    Function(Vec<String>, BlockStatement, Env)
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::Function(params, _body, _env) => {
                let params = params.join(",");
                write!(f, "fn({}) {{...}}", params,)
            }

        }
    }
}
