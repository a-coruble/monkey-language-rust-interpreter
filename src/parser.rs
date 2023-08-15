use anyhow::Error;

use crate::{
    ast::{
        Expression, ExpressionStatement, ExpressionTypes, IdentifierExpression, LetStatement,
        Program, ReturnStatement, StatementTypes,
    },
    lexer::Lexer,
    token::Token,
};

use std::result::Result;
use std::{fmt::Display, mem::discriminant};

#[derive(Debug)]
pub enum PrecedenceOrder {
    LOWEST = 0,
    EQUALS = 1,      // ==
    LESSGREATER = 2, // > or <
    SUM = 3,         //+
    PRODUCT = 4,     //*
    PREFIX = 5,      //-Xor!X
    CALL = 6,        // myFunction(X)
}

type ExpressionParserResult = Result<ExpressionTypes, ParserError>;
// type ParseFn = dyn Fn() -> ExpressionParserResult;

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

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
    pub errors: Vec<ParserError>,
    // prefix_parse_fns: HashMap<Token, ParseFn>,
    // suffix_parse_fns: HashMap<Token, ParseFn>,
}

impl Parser {
    pub fn new(raw_input: String) -> Self {
        println!("[Parser::New] - {}", &raw_input);
        let mut lexer = Lexer::new(raw_input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        // let prefix_parse_fns: HashMap<Token, ParseFn> = HashMap::new();
        // let suffix_parse_fns: HashMap<Token, ParseFn> = HashMap::new();

        let parser = Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
            // prefix_parse_fns,
            // suffix_parse_fns,
        };

        // parser.register_prefix_parse_fn(Token::IDENT("whatever".into()), parser.parse_identifier);
        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, Error> {
        println!("[Parser::parse_program]");
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
        println!("[Parser::current_token_is] - {:?}", discriminant(&token));
        discriminant(&self.current_token) == discriminant(&token)
    }

    fn peek_token_is(&self, token: Token) -> bool {
        println!("[Parser::peek_token_is] - {:?}", discriminant(&token));
        discriminant(&self.peek_token) == discriminant(&token)
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        println!("[Parser::expect_peek] - {:?}", discriminant(&token));
        if self.peek_token_is(token.clone()) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn next_token(&mut self) {
        println!("[Parser::next_token]");
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // fn register_prefix_parse_fn(&mut self, token: Token, parse_fn: ParseFn) {
    //     self.prefix_parse_fns.insert(token, parse_fn);
    // }

    // fn register_suffix_parse_fn(&mut self, token: Token, parse_fn: ParseFn) {
    //     self.suffix_parse_fns.insert(token, parse_fn);
    // }

    fn parse_statement(&mut self) -> Option<StatementTypes> {
        println!("[Parser::parse_statement]");
        match self.current_token {
            Token::LET => match self.parse_let_statement() {
                Ok(let_statement) => Some(StatementTypes::Let(let_statement)),
                Err(parser_error) => {
                    self.errors.push(parser_error);
                    None
                }
            },
            Token::RETURN => match self.parse_return_statement() {
                Ok(let_statement) => Some(StatementTypes::Return(let_statement)),
                Err(parser_error) => {
                    self.errors.push(parser_error);
                    None
                }
            },
            _ => match self.parse_expression_statement() {
                Ok(statement) => Some(StatementTypes::Expression(statement)),
                Err(parse_error) => {
                    self.errors.push(parse_error);
                    None
                }
            },
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, ParserError> {
        println!("[Parser::parse_let_statement]");
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

        self.next_token();

        let let_statement = LetStatement {
            name,
            token: let_token,
            value: match self.parse_expression(PrecedenceOrder::LOWEST) {
                Some(Ok(expression)) => expression,
                Some(Err(err)) => {
                    return Err(ParserError::new(
                        format!(
                            "Wrong Expression parsed after let statement, got error {}",
                            err
                        )
                        .into(),
                    ));
                }
                None => ExpressionTypes::Expression(Expression {
                    token: self.current_token.clone(),
                }),
            },
        };

        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Ok(let_statement) // TODO: Replace the Token::ILLEGAL usage by real computed value once we know how to parse expressions
    }

    fn peek_error(&mut self, token: Token) -> ParserError {
        println!("[Parser::peek_error] - {:?}", discriminant(&token));
        ParserError::new(format!(
            "Expected Token: {} -- Got: {} ",
            token, self.peek_token
        ))
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParserError> {
        println!("[Parser::parse_return_statement]");
        let return_token = self.current_token.clone();

        self.next_token();

        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Ok(ReturnStatement {
            token: return_token,
            value: Expression {
                token: Token::ILLEGAL, // TODO: Replace the Token::ILLEGAL usage by real computed value once we know how to parse expressions
            },
        })
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, ParserError> {
        println!("[Parser::parse_expression_statement]");
        let expression_statement = ExpressionStatement {
            token: self.current_token.clone(),
            expression: match self.parse_expression(PrecedenceOrder::LOWEST) {
                Some(expression_result) => match expression_result {
                    Ok(expression) => expression,
                    Err(_) => todo!(),
                },
                None => ExpressionTypes::Expression(Expression {
                    token: Token::ILLEGAL,
                }),
            },
        };

        if self.peek_token_is(Token::SEMICOLON) {
            // println!("PEEK TOKEN IS SEMI");
            self.next_token();
        }

        Ok(expression_statement)
    }

    fn parse_expression(&mut self, precedence: PrecedenceOrder) -> Option<ExpressionParserResult> {
        println!(
            "[Parser::parse_expression] - {:?} - Current token: {}",
            &precedence, self.current_token
        );
        match &self.current_token {
            Token::IDENT(_) => Some(self.parse_identifier()),
            Token::SEMICOLON => None,
            x  => {
                println!("Couldn't find a parsing function associated to {}", x);
                Some(Err(ParserError::new(format!("Couldn't find a parsing function associated to {}", x))))
            }
            // Some(parse_fn) => match parse_fn() {
            //     Ok(expression) => Ok(expression),
            //     Err(_) => todo!(),
            // },
            // None => todo!(),
        }
    }

    pub fn parse_identifier(&mut self) -> ExpressionParserResult {
        println!("[Parser::parse_identifier] - {}", self.current_token);
        Ok(ExpressionTypes::Identifier(IdentifierExpression {
            token: self.current_token.clone(),
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{
            Expression, ExpressionStatement, ExpressionTypes, IdentifierExpression, LetStatement,
            ReturnStatement, StatementTypes,
        },
        token::Token,
    };

    use super::Parser;

    fn test_parser_error_presence(parser: Parser) {
        if parser.errors.len() == 0 {
            return;
        }

        println!("Parser got some errors:");
        for error in &parser.errors {
            println!("{}", error);
        }

        panic!();
    }

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
                value: ExpressionTypes::Expression(Expression {
                    token: Token::INT("5".into()),
                }),
            }),
            StatementTypes::Let(LetStatement {
                name: IdentifierExpression {
                    token: Token::IDENT("y".into()),
                },
                token: Token::LET,
                value: ExpressionTypes::Expression(Expression {
                    token: Token::INT("10".to_string()),
                }),
            }),
            StatementTypes::Let(LetStatement {
                name: IdentifierExpression {
                    token: Token::IDENT("foobar".into()),
                },
                token: Token::LET,
                value: ExpressionTypes::Expression(Expression {
                    token: Token::INT("838383".into()),
                }),
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

    #[test]
    fn test_parse_return_statements() {
        let input = "
return 5;
return 10;
return 993322;
        "
        .to_string();
        let expected_output: Vec<StatementTypes> = vec![
            StatementTypes::Return(ReturnStatement {
                token: Token::RETURN,
                value: Expression {
                    token: Token::ILLEGAL,
                },
            }),
            StatementTypes::Return(ReturnStatement {
                token: Token::RETURN,
                value: Expression {
                    token: Token::ILLEGAL,
                },
            }),
            StatementTypes::Return(ReturnStatement {
                token: Token::RETURN,
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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();
        let mut parser = Parser::new(input);
        let program = parser.parse_program();
        test_parser_error_presence(parser);

        let expected_output: Vec<StatementTypes> =
            vec![StatementTypes::Expression(ExpressionStatement {
                token: Token::IDENT("foobar".into()),
                expression: ExpressionTypes::Identifier(IdentifierExpression {
                    token: Token::IDENT("foobar".into()),
                }),
            })];
        match program {
            Ok(program) => {
                assert_eq!(program.statements.len(), 1);
                for (i, expected) in expected_output.into_iter().enumerate() {
                    assert_eq!(expected, program.statements[i])
                }
            }
            Err(_) => todo!(),
        }
    }
}
