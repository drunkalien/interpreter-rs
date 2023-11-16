#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    Equal,
    Return,
    True,
    False,
    Minus,
    Bang,
    BangEqual,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    If,
    Else,
    String,
}
