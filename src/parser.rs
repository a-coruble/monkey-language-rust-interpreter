use crate::{
    ast::{Expression, IdentifierExpression, LetStatement, Program, StatementTypes},
    lexer::Lexer,
    token::Token,
};

use anyhow::Result;
use std::mem::discriminant;

pub struct Parser {
    pub lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(raw_input: String) -> Self {
        let mut lexer = Lexer::new(raw_input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        let parser = Parser {
            lexer,
            current_token,
            peek_token,
        };
        parser
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut program = Program::new();

        while self.current_token != Token::EOF {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }
        Ok(program)
    }

    fn current_token_is(&self, token: Token) -> bool {
        discriminant(&self.current_token) == discriminant(&token)
    }

    fn peek_token_is(&self, token: Token) -> bool {
        discriminant(&self.peek_token) == discriminant(&token)
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Option<StatementTypes> {
        match self.current_token {
            Token::LET => Some(StatementTypes::Let(self.parse_let_statement())),
            _ => todo!(),
        }
    }

    fn parse_let_statement(&mut self) -> LetStatement {
        let let_token = self.current_token.clone();

        if !self.expect_peek(Token::IDENT("whatever".into())) {
            unreachable!("[Parser::parse_let_statement] The next token after a LET token should be an IDENT token");
        }

        let name = IdentifierExpression {
            token: self.current_token.clone(),
        };

        if !self.expect_peek(Token::ASSIGN) {
            unreachable!("[Parser::parse_let_statement] The next token after an IDENT token should be an ASSIGN token");
        }

        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        LetStatement {
            name,
            token: let_token,
            value: Expression {
                token: Token::ILLEGAL,
            },
        } // TODO: Replace the Token::ILLEGAL usage by real computed value once we know how to parse expressions
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Expression, IdentifierExpression, LetStatement, StatementTypes},
        token::Token,
    };

    use super::Parser;

    #[test]
    fn test_parse_let_statements() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
        "
        .to_string();
        let expected_output: Vec<StatementTypes> = vec![
            StatementTypes::Let(LetStatement {
                name: IdentifierExpression {
                    token: Token::IDENT("x".into()),
                },
                token: Token::LET,
                value: Expression {
                    token: Token::ILLEGAL,
                },
            }),
            StatementTypes::Let(LetStatement {
                name: IdentifierExpression {
                    token: Token::IDENT("y".into()),
                },
                token: Token::LET,
                value: Expression {
                    token: Token::ILLEGAL,
                },
            }),
            StatementTypes::Let(LetStatement {
                name: IdentifierExpression {
                    token: Token::IDENT("foobar".into()),
                },
                token: Token::LET,
                value: Expression {
                    token: Token::ILLEGAL,
                },
            }),
        ];

        let mut parser = Parser::new(input);
        let program = parser.parse_program();
        match program {
            Ok(program) => {
                assert_eq!(program.statements.len(), 3);
                for (i, expected) in expected_output.into_iter().enumerate() {
                    assert_eq!(expected, program.statements[i])
                }
            }
            Err(_) => panic!(),
        }
    }
}
