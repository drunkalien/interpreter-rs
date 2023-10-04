mod ast;
mod lexer;
mod parser;
mod repl;
mod tokens;

fn main() {
    repl::Repl::start();
}
