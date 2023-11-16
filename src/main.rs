mod ast;
// mod ast_old;
mod lexer;
mod parser;
// mod parser_old;
mod repl;
mod tokens;

fn main() {
    repl::Repl::start();
}
