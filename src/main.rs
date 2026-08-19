mod token;
mod ast;
mod lexer;
mod parser;

fn main() {
	let arg = std::env::args().nth(1).expect("You must provide and input file!");
	let source = std::fs::read_to_string(arg).expect("File does not exist!");
	let tokens = lexer::lex(&source);
	println!("Tokens: {:?}", tokens);
}
