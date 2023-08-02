use std::fmt::Debug;

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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expression {
    pub token: Token,
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

impl Node for IdentifierExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

// Statements

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StatementTypes {
    Let(LetStatement),
}

impl Node for StatementTypes {
    fn token_literal(&self) -> String {
        match self {
            StatementTypes::Let(statement) => statement.token_literal(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LetStatement {
    pub name: IdentifierExpression,
    pub token: Token,
    pub value: Expression,
}

impl Node for LetStatement {
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

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".into()
        }
    }
}
