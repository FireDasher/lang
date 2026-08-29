use logos::Logos;

use crate::token::Token;

mod token;

fn main() {
	let arg = std::env::args().nth(1).expect("You must provide and input file!");
	let source = std::fs::read_to_string(arg).expect("File does not exist!");
	let lex = Token::lexer(&source);

	for token in lex {
		if let Ok(tok) = token {
			println!("{tok:?}");
		}
	}
}
