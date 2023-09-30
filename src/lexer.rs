use crate::tokens::Token;

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
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
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    _ => Token::Ident(ident),
                };
            }
            b'0'..=b'9' => {
                let number = self.read_int();

                return Token::Int(number);
            }
            0 => Token::Eof,
            _ => Token::Illegal,
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
}

#[cfg(test)]
mod test {
    #[test]
    fn get_next_token() -> Result<(), ()> {
        let input = String::from(
            "let five = 5;\n
        let ten = 10;\n
        let add = fn(x, y) {
            x + y;
        };",
        );

        let mut lexer = super::Lexer::new(input);

        let tests = [
            super::Token::Let,
            super::Token::Ident("five".into()),
            super::Token::Assign,
            super::Token::Int(String::from("5")),
            super::Token::Semicolon,
            super::Token::Let,
            super::Token::Ident(String::from("ten")),
            super::Token::Assign,
            super::Token::Int(String::from("10")),
            super::Token::Semicolon,
            super::Token::Let,
            super::Token::Ident(String::from("add")),
            super::Token::Assign,
            super::Token::Function,
            super::Token::LParen,
            super::Token::Ident(String::from("x")),
            super::Token::Comma,
            super::Token::Ident("y".into()),
            super::Token::RParen,
            super::Token::LBrace,
            super::Token::Ident(String::from("x")),
            super::Token::Plus,
            super::Token::Ident(String::from("y")),
            super::Token::Semicolon,
            super::Token::RBrace,
            super::Token::Semicolon,
            super::Token::Eof,
        ];

        for (_, expected) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            if tok != *expected {
                return Err(());
            }
        }

        Ok(())
    }
}
