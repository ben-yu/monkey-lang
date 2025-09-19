mod ast;
mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

// Import the `console.log` function from the browser
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to provide `println!(..)`-style syntax
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        log(&format!( $( $t )* ))
    }
}

// Initialize WASM module
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages in the browser
    console_error_panic_hook::set_once();
}

// WASM-exported interpreter state
#[wasm_bindgen]
pub struct MonkeyInterpreter {
    env: environment::Env,
}

#[wasm_bindgen]
impl MonkeyInterpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MonkeyInterpreter {
        MonkeyInterpreter {
            env: Rc::new(RefCell::new(environment::Environment::default())),
        }
    }

    #[wasm_bindgen]
    pub fn eval(&self, input: &str) -> String {
        match parser::parse(input) {
            Ok(node) => match evaluator::eval(node, &self.env) {
                Ok(result) => format!("{}", result),
                Err(err) => format!("Error: {}", err),
            },
            Err(err) => format!("Parse Error: {:#?}", err),
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.env = Rc::new(RefCell::new(environment::Environment::default()));
    }
}

// Standalone function for single expression evaluation
#[wasm_bindgen]
pub fn eval_monkey(input: &str) -> String {
    let interpreter = MonkeyInterpreter::new();
    interpreter.eval(input)
}