use crate::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::new();

	let chars: Vec<char> = input.chars().collect();
	let mut index = 0;

	loop {
		if index >= chars.len() {
			return tokens
		}
		let char = chars[index];
		if char.is_whitespace() {
			index += 1;
		} else if char.is_alphabetic() {
			let mut identifier = String::new();
			while let char = chars[index] && char.is_alphanumeric() {
				identifier.push(char);
				index += 1;
			}
			tokens.push(Token::keyword(&identifier));
		} else if char.is_ascii_digit() {
			let mut num_str = String::new();
			let mut is_float = false;
			while let char = chars[index] && (char.is_ascii_digit() || char == '.') {
				num_str.push(char);
				if char == '.' {
					is_float = true;
				}
				index += 1;
			}
			if is_float {
				tokens.push(Token::Float(num_str.parse().expect("Error: Invalid float literal!")));
			} else {
				tokens.push(Token::Int(num_str.parse().expect("Error: Invalid integer literal!")));
			}
		} else {
			let op = Token::operator(char);
			if op != Token::Null {
				tokens.push(op);
			}
			index += 1;
		}
	}
}