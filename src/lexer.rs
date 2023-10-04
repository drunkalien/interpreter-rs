use crate::tokens::{Token, TokenType};

pub struct Lexer {
    pub position: usize,
    pub read_position: usize,
    pub ch: u8,
    pub input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };

        l.next_token();

        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::Equal, "==".into())
                } else {
                    Token::new(TokenType::Assign, "=".into())
                }
            }
            b';' => Token::new(TokenType::Semicolon, ";".into()),
            b'(' => Token::new(TokenType::LParen, "(".into()),
            b')' => Token::new(TokenType::RParen, ")".into()),
            b',' => Token::new(TokenType::Comma, ",".into()),
            b'+' => Token::new(TokenType::Plus, "+".into()),
            b'{' => Token::new(TokenType::LBrace, "{".into()),
            b'}' => Token::new(TokenType::RBrace, "}".into()),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return match ident.as_str() {
                    "fn" => Token::new(TokenType::Function, "fn".into()),
                    "let" => Token::new(TokenType::Let, "let".into()),
                    "return" => Token::new(TokenType::Return, "return".into()),
                    "true" => Token::new(TokenType::True, "true".into()),
                    "false" => Token::new(TokenType::False, "false".into()),
                    "if" => Token::new(TokenType::If, "if".into()),
                    "else" => Token::new(TokenType::Else, "else".into()),
                    _ => Token::new(TokenType::Ident, ident),
                };
            }
            b'0'..=b'9' => {
                let number = self.read_int();

                return Token::new(TokenType::Int, number);
            }
            b'-' => Token::new(TokenType::Minus, "-".into()),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::BangEqual, "!=".into())
                } else {
                    Token::new(TokenType::Bang, "!".into())
                }
            }
            b'*' => Token::new(TokenType::Asterisk, "*".into()),
            b'/' => Token::new(TokenType::Slash, "/".into()),
            b'<' => Token::new(TokenType::LessThan, "<".into()),
            b'>' => Token::new(TokenType::GreaterThan, ">".into()),
            0 => Token::new(TokenType::Eof, "".into()),
            _ => Token::new(TokenType::Illegal, "".into()),
        };

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        String::from_utf8(self.input[position..self.position].to_vec()).unwrap()
    }

    fn read_int(&mut self) -> String {
        let position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8(self.input[position..self.position].to_vec()).unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get_next_token() -> Result<(), ()> {
        let input = String::from(
            "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ",
        );

        let mut lexer = super::Lexer::new(input);

        let tests = vec![
            (super::TokenType::Let, "let"),
            (super::TokenType::Ident, "five"),
            (super::TokenType::Assign, "="),
            (super::TokenType::Int, "5"),
            (super::TokenType::Semicolon, ";"),
            (super::TokenType::Let, "let"),
            (super::TokenType::Ident, "ten"),
            (super::TokenType::Assign, "="),
            (super::TokenType::Int, "10"),
            (super::TokenType::Semicolon, ";"),
            (super::TokenType::Let, "let"),
            (super::TokenType::Ident, "add"),
            (super::TokenType::Assign, "="),
            (super::TokenType::Function, "fn"),
            (super::TokenType::LParen, "("),
            (super::TokenType::Ident, "x"),
            (super::TokenType::Comma, ","),
            (super::TokenType::Ident, "y"),
            (super::TokenType::RParen, ")"),
            (super::TokenType::LBrace, "{"),
            (super::TokenType::Ident, "x"),
            (super::TokenType::Plus, "+"),
            (super::TokenType::Ident, "y"),
            (super::TokenType::Semicolon, ";"),
            (super::TokenType::RBrace, "}"),
            (super::TokenType::Semicolon, ";"),
            (super::TokenType::Let, "let"),
            (super::TokenType::Ident, "result"),
            (super::TokenType::Assign, "="),
            (super::TokenType::Ident, "add"),
            (super::TokenType::LParen, "("),
            (super::TokenType::Ident, "five"),
            (super::TokenType::Comma, ","),
            (super::TokenType::Ident, "ten"),
            (super::TokenType::RParen, ")"),
        ];

        for (_, expected) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            println!("expected: {:?}, got: {:?}", expected, tok);
            if tok.token_type != expected.0 {
                return Err(());
            }

            if tok.literal != expected.1 {
                return Err(());
            }
        }

        Ok(())
    }
}
