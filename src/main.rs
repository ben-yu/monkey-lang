mod token;
mod lexer;
mod repl;
mod ast;
mod parser;

fn main() {
    println!("This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start();
}

