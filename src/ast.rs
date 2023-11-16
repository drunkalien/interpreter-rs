use crate::tokens::Token;

pub struct Program {
    pub statements: Vec<Statement>,
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

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.token_literal(),
            Expression::IntegerLiteral(integer_literal) => integer_literal.token_literal(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.string(),
            Expression::IntegerLiteral(integer_literal) => integer_literal.string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.string(),
        }
    }
}

impl Expression {
    fn expression_node(&self) {}
}

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(let_statement) => let_statement.token_literal(),
            Statement::Return(return_statement) => return_statement.token_literal(),
            Statement::Expression(expression_statement) => expression_statement.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            Statement::Let(let_statement) => let_statement.string(),
            Statement::Return(return_statement) => return_statement.string(),
            Statement::Expression(expression_statement) => expression_statement.string(),
        }
    }
}

impl Statement {
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

impl Expression {
    fn identifier_expression(identifier: Identifier) -> Expression {
        Expression::Identifier(identifier)
    }
}

impl Statement {
    fn let_statement(let_statement: LetStatement) -> Statement {
        Statement::Let(let_statement)
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
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

        if let Some(value) = &self.value {
            out.push_str(&value.string());
        }

        out.push_str(";");

        out
    }
}

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.token_literal());
        out.push_str(" ");

        if let Some(return_value) = &self.return_value {
            out.push_str(&return_value.string());
        }

        out.push_str(";");

        out
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        if let Some(expr) = &self.expression {
            expr.string()
        } else {
            String::new()
        }
    }
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

#[cfg(test)]
mod test {
    use super::Identifier;
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
            value: Some(super::Expression::Identifier(Identifier {
                token: Token {
                    token_type: crate::tokens::TokenType::Ident,
                    literal: "another_var".into(),
                },
                value: "another_var".into(),
            })),
        };

        program
            .statements
            .push(super::Statement::Let(let_statement));

        let test = "let my_var = another_var;";

        if program.string() != test {
            println!("expected={} got={}", test, program.string());
            return Err(());
        }

        Ok(())
    }
}
