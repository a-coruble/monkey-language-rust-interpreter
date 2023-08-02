use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(String),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NotEq,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ASSIGN => write!(f, "Token::ASSIGN"),
            Token::COMMA => write!(f, "Token::COMMA"),
            Token::EOF => write!(f, "Token::EOF"),
            Token::FUNCTION => write!(f, "Token::FUNCTION"),
            Token::IDENT(string) => write!(f, "Token::IDENT({})", string),
            Token::ILLEGAL => write!(f, "Token::ILLEGAL"),
            Token::INT(int) => write!(f, "Token::INT({})", int),
            Token::LBRACE => write!(f, "Token::LBRACE"),
            Token::LET => write!(f, "Token::LET"),
            Token::LPAREN => write!(f, "Token::LPAREN"),
            Token::PLUS => write!(f, "Token::PLUS"),
            Token::RBRACE => write!(f, "Token::RBRACE"),
            Token::RPAREN => write!(f, "Token::RPAREN"),
            Token::SEMICOLON => write!(f, "Token::SEMICOLON"),
            Token::MINUS => write!(f, "Token::MINUS"),
            Token::BANG => write!(f, "Token::BANG"),
            Token::ASTERISK => write!(f, "Token::ASTERISK"),
            Token::SLASH => write!(f, "Token::SLASH"),
            Token::LT => write!(f, "Token::LT"),
            Token::GT => write!(f, "Token::GT"),
            Token::TRUE => write!(f, "Token::TRUE"),
            Token::FALSE => write!(f, "Token::FALSE"),
            Token::IF => write!(f, "Token::IF"),
            Token::ELSE => write!(f, "Token::ELSE"),
            Token::RETURN => write!(f, "Token::RETURN"),
            Token::EQ => write!(f, "Token::EQ (==)"),
            Token::NotEq => write!(f, "Token::NotEq (!=)"),
        }
    }
}

pub fn lookup_identifier(identifier: String) -> Token {
    match identifier.as_str() {
        "let" => Token::LET,
        "fn" => Token::FUNCTION,
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENT(identifier),
    }
}
