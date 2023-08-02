use std::fmt::Debug;

use crate::token::Token;

pub trait Node: Debug + Clone {
    fn token_literal(&self) -> String;
}

pub enum NodeTypes {
    Epxression(ExpressionTypes),
    Statement(StatementTypes),
    Program(Program),
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

#[derive(Clone, Debug)]
pub struct Expression {
    token: Token,
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct IdentifierExpression {
    token: Token,
}

impl Node for IdentifierExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

// Statements

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct LetStatement {
    name: Expression,
    token: Token,
    value: IdentifierExpression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    statements: Vec<StatementTypes>,
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
