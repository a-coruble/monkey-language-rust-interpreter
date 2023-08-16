use crate::{ast::Program, lexer::Lexer, token::Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut lexer = lexer.clone();
        let current_token = lexer.next_token().clone();
        let peek_token = lexer.next_token().clone();

        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().clone();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        None
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::{
        ast::{LetStatement, Node},
        lexer::Lexer,
        token::Token,
    };

    use super::Parser;

    fn test_let_statement(statement: LetStatement, name: String) -> bool {
        if statement.token_literal() != Token::LET.to_string() {
            return false;
        }

        if statement.name.value != name {
            return false;
        }

        if statement.name.token_literal() != name {
            return false;
        }

        true
    }

    #[test]
    fn test_parse_let_statements() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;";

        let expected = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);

        let parsed_program = parser.parse_program();

        if let Some(program) = parsed_program {
            assert_eq!(program.statements.len(), 3);
            for (i, name) in expected.into_iter().enumerate() {
                assert!(test_let_statement(
                    program.statements[i] as LetStatement,
                    name
                ));
            }
        } else {
            panic!("Failed to parse program, got None as a result");
        }
    }
}
