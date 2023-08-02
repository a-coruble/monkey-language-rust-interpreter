use std::io::{stdin, stdout, Write};

use crate::lexer::Lexer;
use crate::token::Token;

static PROMPT: &str = ">> ";

pub fn start_repl() {
    let mut read_line = String::new();

    println!("Welcome to the Monkey Language interactive REPL!\nPlease enter a line of code below and press enter to get it evaluated:");
    loop {
        print!("{PROMPT}");
        stdout().flush().unwrap();
        match stdin().read_line(&mut read_line) {
            Ok(_) => handle_user_input(&read_line),
            Err(_) => break,
        }
    }
}

fn handle_user_input(read_line: &String) {
    let mut lexer = Lexer::new(read_line.clone());

    loop {
        let token = lexer.next_token();
        match token {
            Token::ILLEGAL => break,
            Token::EOF => break,
            _ => println!("{token}"),
        }
    }
}
