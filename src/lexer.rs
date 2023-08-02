use crate::token::{lookup_identifier, Token};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: b'\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.read_position])
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut literal = String::new();

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            literal.push(self.ch.into());
            self.read_char();
        }

        literal
    }

    fn read_int(&mut self) -> Token {
        let mut literal = String::new();

        while self.ch.is_ascii_digit() {
            literal.push(self.ch.into());
            self.read_char();
        }

        Token::INT(literal)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Token::EQ
                }
                _ => Token::ASSIGN,
            },
            b'!' => match self.peek_char() {
                Some(b'=') => {
                    self.read_char();
                    Token::NotEq
                }
                _ => Token::BANG,
            },
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'/' => Token::SLASH,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'*' => Token::ASTERISK,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return lookup_identifier(self.read_identifier());
            }
            b'0'..=b'9' => return self.read_int(),
            b'\0' => Token::EOF,
            _ => Token::ILLEGAL,
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod test {
    use crate::token::Token;

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = "
let five = 5;
let ten = 10;
   let add = fn(x, y) {
     x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10; 10 != 9;"
            .to_string();
        let expected_tokens = vec![
            Token::LET,
            Token::IDENT("five".into()),
            Token::ASSIGN,
            Token::INT("5".into()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".into()),
            Token::ASSIGN,
            Token::INT("10".into()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".into()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".into()),
            Token::COMMA,
            Token::IDENT("y".into()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".into()),
            Token::PLUS,
            Token::IDENT("y".into()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".into()),
            Token::ASSIGN,
            Token::IDENT("add".into()),
            Token::LPAREN,
            Token::IDENT("five".into()),
            Token::COMMA,
            Token::IDENT("ten".into()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".into()),
            Token::SEMICOLON,
            Token::INT("5".into()),
            Token::LT,
            Token::INT("10".into()),
            Token::GT,
            Token::INT("5".into()),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5".into()),
            Token::LT,
            Token::INT("10".into()),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT("10".into()),
            Token::EQ,
            Token::INT("10".into()),
            Token::SEMICOLON,
            Token::INT("10".into()),
            Token::NotEq,
            Token::INT("9".into()),
            Token::SEMICOLON,
        ];

        let mut lexer = Lexer::new(input);
        for expected in expected_tokens {
            let token = lexer.next_token();
            println!("Expected: {} - Got {}", expected, token);
            assert_eq!(expected, token);
        }
    }
}
