use crate::token::Token;
use crate::lexer::Lexer;
use std::io::Write;

pub fn start() {

    print!(">> ");
    std::io::stdout().flush().expect("can't flush stdout");
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut tokenizer = Lexer::new(line);

            loop {
                let token = tokenizer.next_token();
                println!("{} ", token);
                if let Token::Eof = token {
                    break;
                }
            }
        }
        print!(">> ");
        std::io::stdout().flush().expect("can't flush stdout");
    });
}


