use crate::tokens::{Token, TokenType};
use downcast_rs::{impl_downcast, Downcast};

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node + Downcast {
    fn statement_node(&self);
}

impl_downcast!(Statement);

pub trait Expression: Node + Downcast {
    fn expression_node(&self);
}

impl_downcast!(Expression);

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();

        for s in &self.statements {
            out.push_str(&s.string());
        }

        out
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.string());
        out.push_str(" = ");

        if let Some(value) = self.value.as_ref() {
            out.push_str(&value.string());
        }

        out.push_str(";");

        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.token_literal());
        out.push_str(" ");

        if let Some(return_value) = self.return_value.as_ref() {
            out.push_str(&return_value.string());
        }

        out.push_str(";");

        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        if let Some(expr) = self.expression.as_ref() {
            expr.string()
        } else {
            String::new()
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

struct IntegerLiteral {
    token: Token,
    value: i64,
}

impl Node for IntegerLiteral {
    fn string(&self) -> String {
        self.token.literal.clone()
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Node;
    use crate::tokens::Token;

    use super::LetStatement;

    #[test]
    fn test_string() -> Result<(), ()> {
        let mut program = crate::ast::Program {
            statements: Vec::new(),
        };

        let let_statement = LetStatement {
            token: Token {
                token_type: crate::tokens::TokenType::Let,
                literal: "let".into(),
            },
            name: super::Identifier {
                token: Token {
                    token_type: crate::tokens::TokenType::Ident,
                    literal: "my_var".into(),
                },
                value: "my_var".into(),
            },
            value: Some(Box::new(super::Identifier {
                token: Token {
                    token_type: crate::tokens::TokenType::Ident,
                    literal: "another_var".into(),
                },
                value: "another_var".into(),
            })),
        };

        program.statements.push(Box::new(let_statement));

        let test = "let my_var = another_var;";

        if program.string() != test {
            println!("expected={} got={}", test, program.string());
            return Err(());
        }

        Ok(())
    }
}
