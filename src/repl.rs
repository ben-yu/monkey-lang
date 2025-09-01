use crate::token::Token;
use crate::lexer::Lexer;
use crate::parser::*;
use std::io::Write;

pub fn start() {

    print!(">> ");
    std::io::stdout().flush().expect("can't flush stdout");
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            match parse(&line) {
                Ok(node) => println!("{}", node),
                Err(e) => panic!("Parsing Error: {:#?}", e),
            }
        }
        print!(">> ");
        std::io::stdout().flush().expect("can't flush stdout");
    });
}


