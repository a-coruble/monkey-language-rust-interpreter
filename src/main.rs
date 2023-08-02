pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    repl::start_repl();
}
