use std::{str::Chars, iter::Peekable};

use crate::token::{Token, lookup_identifier};

pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input: input.chars().peekable()}
    }

     fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }
    
    fn read_identifier(&mut self, first_char: char) -> String {
        let mut identifier = String::new();
        identifier.push(first_char);
        while let Some(&ch) = self.input.peek() {
            if is_letter(ch) {
                identifier.push(self.read_char().unwrap())
            } else {
                break;
            }
        }
        identifier
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.read_char() {
            Some('=') => match self.input.peek() {
                Some('=') => {
                    self.read_char();
                    Token::EQ
                },
                _ => Token::ASSIGN,
            },
            Some('+') => Token::PLUS,
            Some(',') => Token::COMMA,
            Some(';') => Token::SEMICOLON,
            Some('(') => Token::LPAREN,
            Some(')') => Token::RPAREN,
            Some('{') => Token::LBRACE,
            Some('}') => Token::RBRACE,
            Some('-') => Token::MINUS,
            Some('*') => Token::ASTERISK,
            Some('!') => match self.input.peek() {
                Some('=') => {
                    self.read_char();
                    Token::NotEq
                },
                _ => Token::BANG,
            },
            Some('<') => Token::LT,
            Some('>') => Token::GT,
            Some('/') => Token::SLASH,
            Some(ch) => {
                if is_letter(ch) {
                    lookup_identifier(self.read_identifier(ch))
                } else if ch.is_numeric() {
                    Token::INT(self.read_int(ch))
                } else {
                    Token::ILLEGAL
                }
            },
            None => Token::EOF,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_int(&mut self, first_digit: char) -> i64 {
        let mut int = String::new();
        int.push(first_digit);
        while let Some(&ch) = self.input.peek() {
            if ch.is_numeric() {
                int.push(self.read_char().unwrap());
            } else {
                break;
            }
        }

        int.parse().unwrap()
    }
}