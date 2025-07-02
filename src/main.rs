mod token;
mod lexer;
mod repl;

fn main() {
    println!("This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start();
}

