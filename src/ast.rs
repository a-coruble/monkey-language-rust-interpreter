use std::fmt::{Debug, Display};

use crate::token::Token;

pub enum NodeTypes {
    Epxression(ExpressionTypes),
    Statement(StatementTypes),
    Program(Program),
}

pub trait Node: Debug + Clone {
    fn token_literal(&self) -> String;
}

// Expressions

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionTypes {
    Identifier(IdentifierExpression),
    Expression(Expression),
}

impl Node for ExpressionTypes {
    fn token_literal(&self) -> String {
        match self {
            ExpressionTypes::Identifier(identifier) => identifier.token_literal(),
            ExpressionTypes::Expression(expression) => expression.token_literal(),
        }
    }
}

impl Display for ExpressionTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionTypes::Identifier(expression) => write!(f, "{}", expression),
            ExpressionTypes::Expression(expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expression {
    pub token: Token,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifierExpression {
    pub token: Token,
}

impl Display for IdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl Node for IdentifierExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

// Statements

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StatementTypes {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Display for StatementTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementTypes::Let(statement) => write!(f, "{}", statement),
            StatementTypes::Return(statement) => write!(f, "{}", statement),
            StatementTypes::Expression(statement) => write!(f, "{}", statement),
        }
    }
}

impl Node for StatementTypes {
    fn token_literal(&self) -> String {
        match self {
            StatementTypes::Let(statement) => statement.token_literal(),
            StatementTypes::Return(statement) => statement.token_literal(),
            StatementTypes::Expression(statement) => statement.token_literal(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LetStatement {
    pub name: IdentifierExpression,
    pub token: Token,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = {};", self.token, self.name, self.value)
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token, self.value)
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: ExpressionTypes,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ExpressionStatement {{ {}, value: {} }}",
            self.token_literal(),
            self.expression,
        )
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

// Program

#[derive(Clone, Debug)]
pub struct Program {
    pub statements: Vec<StatementTypes>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.clone() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".into()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ast::IdentifierExpression, token::Token};

    use super::{Expression, LetStatement, Program, StatementTypes};

    #[test]
    fn test_ast_to_string() {
        let program = Program {
            statements: vec![StatementTypes::Let(LetStatement {
                name: IdentifierExpression {
                    token: Token::IDENT("myVar".into()),
                },
                token: Token::LET,
                value: Expression {
                    token: Token::IDENT("anotherVar".into()),
                },
            })],
        };
        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }
}
