use crate::token::Token;
use crate::lexer::Lexer;
use crate::parser::*;
use crate::evaluator::*;
use std::io::Write;

const MONKEY_FACE: &str = r#"            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

pub fn start() {

    print!(">> ");
    std::io::stdout().flush().expect("can't flush stdout");
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            match parse(&line) {
                Ok(node) => match eval(node) {
                        Ok(evaluated) => {
                            println!("{}", evaluated)
                        }
                        Err(err) => eprintln!("{}", err),
                },
                Err(errors) => {
                    eprintln!("{}", MONKEY_FACE);
                    for e in errors {
                        eprintln!("{}", e);
                    }
                }
            }
        }
        print!(">> ");
        std::io::stdout().flush().expect("can't flush stdout");
    });
}


