mod ast;
mod lexer;
mod repl;
mod tokens;

fn main() {
    repl::Repl::start();
}
