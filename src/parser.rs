use anyhow::Error;

use crate::{
    ast::{Expression, IdentifierExpression, LetStatement, Program, StatementTypes},
    lexer::Lexer,
    token::Token,
};

use std::result::Result;
use std::{fmt::Display, mem::discriminant};

#[derive(Debug)]
pub struct ParserError {
    pub details: String,
}

impl ParserError {
    fn new(details: String) -> Self {
        Self { details }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ParserError] - {}", self.details)
    }
}

pub struct Parser {
    pub lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
    pub errors: Vec<ParserError>,
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
            errors: Vec::new(),
        };
        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, Error> {
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
        if self.peek_token_is(token.clone()) {
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
            Token::LET => match self.parse_let_statement() {
                Ok(let_statement) => Some(StatementTypes::Let(let_statement)),
                Err(parser_error) => {
                    self.errors.push(parser_error);
                    None
                }
            },
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, ParserError> {
        let let_token = self.current_token.clone();

        if !self.expect_peek(Token::IDENT("whatever".into())) {
            return Err(self.peek_error(Token::IDENT("whatever".into())));
        }

        let name = IdentifierExpression {
            token: self.current_token.clone(),
        };

        if !self.expect_peek(Token::ASSIGN) {
            return Err(self.peek_error(Token::ASSIGN));
        }

        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Ok(LetStatement {
            name,
            token: let_token,
            value: Expression {
                token: Token::ILLEGAL,
            },
        }) // TODO: Replace the Token::ILLEGAL usage by real computed value once we know how to parse expressions
    }

    fn peek_error(&mut self, token: Token) -> ParserError {
        ParserError {
            details: format!("Expected Token: {} -- Got: {} ", token, self.peek_token),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Expression, IdentifierExpression, LetStatement, StatementTypes},
        token::Token,
    };

    use super::Parser;

    fn test_parser_error_presence(parser: Parser) {
        if parser.errors.len() == 0 {
            return;
        }

        println!("Parser got some errors:");
        for error in parser.errors {
            println!("{}", error);
        }
        panic!();
    }

    #[test]
    fn test_parse_let_statements() {
        let input = "
let x 5;
let = 10;
let 838383;
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
        test_parser_error_presence(parser);
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
