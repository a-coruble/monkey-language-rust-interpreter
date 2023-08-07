use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
            Token::ASSIGN => write!(f, "="),
            Token::COMMA => write!(f, ","),
            Token::EOF => write!(f, "\0"),
            Token::FUNCTION => write!(f, "fn"),
            Token::IDENT(string) => write!(f, "{}", string),
            Token::ILLEGAL => write!(f, "ILLEGAL"),
            Token::INT(int) => write!(f, "{}", int),
            Token::LBRACE => write!(f, "{{"),
            Token::LET => write!(f, "let"),
            Token::LPAREN => write!(f, "("),
            Token::PLUS => write!(f, "+"),
            Token::RBRACE => write!(f, "}}"),
            Token::RPAREN => write!(f, ")"),
            Token::SEMICOLON => write!(f, ";"),
            Token::MINUS => write!(f, "-"),
            Token::BANG => write!(f, "!"),
            Token::ASTERISK => write!(f, "*"),
            Token::SLASH => write!(f, "/"),
            Token::LT => write!(f, "<"),
            Token::GT => write!(f, ">"),
            Token::TRUE => write!(f, "true"),
            Token::FALSE => write!(f, "false"),
            Token::IF => write!(f, "if"),
            Token::ELSE => write!(f, "else"),
            Token::RETURN => write!(f, "return"),
            Token::EQ => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
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
