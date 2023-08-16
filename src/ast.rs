use std::fmt::{Debug, Display};

use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub enum Statements {
    Let(LetStatement),
}

impl Node for Statements {
    fn token_literal(&self) -> String {
        todo!()
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expressions,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

pub enum Expressions {}

pub struct Program {
    pub statements: Vec<Statements>,
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

impl Node for Expressions {
    fn token_literal(&self) -> String {
        todo!()
    }
}
