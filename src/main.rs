use crate::token::Token;

mod token;
mod ast;

fn main() {
	let arg = std::env::args().nth(1).expect("You must provide and input file!");
	let source = std::fs::read_to_string(arg).expect("File does not exist!");
	let tokens = Token::lex(&source);
	println!("Tokens: {:?}", tokens);
}
