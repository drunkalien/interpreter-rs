use crate::ast::{
    Expression, ExpressionStatement, Identifier, LetStatement, Node, Program, ReturnStatement,
    Statement,
};
use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use std::collections::HashMap;

type prefixParseFn = fn(&mut Parser) -> Box<dyn Expression>;
type infixParseFn = fn(dyn Expression) -> Box<dyn Expression>;

// Precedence constants
const LOWEST: u8 = 1;
const EQUALS: u8 = 2; // ==
const LESS_GREATER: u8 = 3; // > or <
const SUM: u8 = 4; // +
const PRODUCT: u8 = 5; // *
const PREFIX: u8 = 6; // -X or !X
const CALL: u8 = 7; // my_function(X)

pub struct Parser {
    pub lexer: Lexer,
    pub cur_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,

    pub prefixParseFns: HashMap<TokenType, prefixParseFn>,
    pub infixParseFns: HashMap<TokenType, infixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::new(TokenType::Illegal, "".into()),
            peek_token: Token::new(TokenType::Illegal, "".into()),
            errors: Vec::new(),
            prefixParseFns: HashMap::new(),
            infixParseFns: HashMap::new(),
        };
        parser.next_token();
        parser.next_token();
        parser.register_prefix(TokenType::Ident, Parser::parse_identifier);
        parser
    }

    fn parse_identifier(&mut self) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn errors(&mut self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.cur_token.token_type
        );

        self.errors.push(msg);
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.token_type != TokenType::Eof {
            let stmt = self.parse_statement();

            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let expr = self.parse_expression(LOWEST);
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: expr,
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: u8) -> Option<Box<dyn Expression>> {
        let prefix = match self.prefixParseFns.get(&self.cur_token.token_type) {
            Some(pref) => pref,
            None => return None,
        };

        let left_expr = prefix(self);
        Some(left_expr)
    }

    pub fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let name = Identifier {
            token: self.peek_token.clone(),
            value: self.peek_token.literal.clone(),
        };

        let stmt = LetStatement {
            token: self.cur_token.clone(),
            name,
            value: None,
        };

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while !self.cur_token_is(TokenType::Semicolon) && !self.cur_token_is(TokenType::Eof) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    pub fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            return_value: Some(Box::new(Identifier {
                token: self.peek_token.clone(),
                value: self.peek_token.literal.clone(),
            })),
        };

        self.next_token();
        while !self.cur_token_is(TokenType::Semicolon) && !self.cur_token_is(TokenType::Eof) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn cur_token_is(&mut self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn register_prefix(&mut self, token_type: TokenType, func: prefixParseFn) {
        self.prefixParseFns.insert(token_type, func);
    }

    fn register_infix(&mut self, token_type: TokenType, func: infixParseFn) {
        self.infixParseFns.insert(token_type, func);
    }
}

#[cfg(test)]
mod test {
    use crate::ast::ExpressionStatement;

    use super::*;

    #[test]

    pub fn test_let_statements() -> Result<(), ()> {
        let input = String::from(
            "let x = 5;
		let y = 10;
		let foobar = 838383;",
        );

        let lexer = Lexer::new(input);

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        check_parser_errors(&mut parser)?;

        if program.statements.len() != 3 {
            println!(
                "program.statements does not contain 3 statements. got={}",
                program.statements.len()
            );
            return Err(());
        }

        let tests = vec!["x", "y", "foobar"];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];

            if !test_let_statement(stmt, tt) {
                return Err(());
            }
        }

        fn test_let_statement(s: &Box<dyn Statement>, name: &str) -> bool {
            if s.token_literal() != "let" {
                println!("s.token_literal not 'let'. got={}", s.token_literal());
                return false;
            }

            let let_stmt: &LetStatement = match s.downcast_ref::<LetStatement>() {
                Some(stmt) => stmt,
                None => {
                    println!("s is not LetStatement. got={}", s.token_literal());
                    return false;
                }
            };

            if let_stmt.name.value != name {
                println!(
                    "let_stmt.name.value not '{}'. got={}",
                    name, let_stmt.name.value
                );
                return false;
            }

            true
        }

        Ok(())
    }

    #[test]
    fn test_return_statements() -> Result<(), ()> {
        let input = String::from(
            "return 5;
			return 10;
			return 993322;",
        );

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&mut parser)?;

        if program.statements.len() != 3 {
            println!(
                "program.statements does not contain 3 statements. got={}",
                program.statements.len()
            );
            return Err(());
        }

        for (_, stmt) in program.statements.iter().enumerate() {
            if stmt.downcast_ref::<ReturnStatement>().is_none() {
                println!("stmt not ReturnStatement. got={}", stmt.token_literal());
                return Err(());
            }

            if stmt.token_literal() != "return" {
                println!("s.token_literal not 'return'. got={}", stmt.token_literal());
                return Err(());
            }
        }

        Ok(())
    }

    fn check_parser_errors(p: &mut Parser) -> Result<(), ()> {
        let errors = p.errors();

        if errors.len() == 0 {
            return Ok(());
        }

        println!("parser has {} errors", errors.len());

        for msg in errors {
            println!("parser error: {}", msg);
        }
        Err(())
    }

    #[test]
    fn test_identifier_expression() -> Result<(), ()> {
        let input = String::from("foobar");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let _ = check_parser_errors(&mut parser);

        if program.statements.len() != 1 {
            println!(
                "Program doesn't have enough statements. Got {}",
                program.statements.len()
            );
            return Err(());
        }

        let stmt = match program.statements[0].downcast_ref::<ExpressionStatement>() {
            Some(stmt) => stmt,
            None => {
                println!("Statement not ast expression");
                return Err(());
            }
        };

        let expr = stmt.expression.as_ref().unwrap();

        let ident = match expr.downcast_ref::<Identifier>() {
            Some(ident) => ident,
            None => {
                println!("Statement not ast identifier");
                return Err(());
            }
        };

        if ident.value != String::from("foobar") {
            println!("ident.value is not {}. Got {}", "foobar", ident.value);
            return Err(());
        }

        if ident.token_literal() != String::from("foobar") {
            println!(
                "ident.token_literal is not {}. Got: {}",
                "foobar",
                ident.token_literal()
            );
            return Err(());
        }

        Ok(())
    }

    #[test]
    fn parse_literal() -> Result<(), ()> {
        let input = String::from("5;");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if program.statements.len() != 1 {
            println!(
                "program doesn't have enough statements, got: {}",
                program.statements.len()
            );
            return Err(());
        }

        let stmt = match program.statements[0].downcast_ref::<ExpressionStatement>() {
            Some(stmt) => stmt,
            None => {
                println!("Statement is not ast ExpressionStatement");
                return Err(());
            }
        };

        Ok(())
    }
}
