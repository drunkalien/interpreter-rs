use crate::lexer::Lexer;
use crate::tokens::Token;
use std::io::Write;
use std::io::{stdin, stdout};

const PROMPT: &str = ">> ";

pub struct Repl;

impl Repl {
    pub fn start() {
        loop {
            print!("{}", PROMPT);
            stdout().flush().expect("Error flushing stdout");

            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Error reading from stdin");

            let mut lexer = Lexer::new(input);

            loop {
                let token = lexer.next_token();

                if token == Token::Eof {
                    break;
                }

                println!("{:?}", token);
            }
        }
    }
}
