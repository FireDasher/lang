use crate::token::{BinaryOperator, Token};

pub fn lex(input: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = input.chars();
	let mut char = ' ';
	let mut running = true;

	macro_rules! get_char {
	    () => {
	        { match chars.next() { Some(c) => {char = c; true}, None => {running = false; false} } }
	    };
	}

	macro_rules! skip_char {
	    () => {
	        match chars.next() { Some(c) => {char = c}, None => {running = false} }
	    };
	}

	while running {
		if char.is_whitespace() {
			while get_char!() && char.is_whitespace() {}
		}

		else if char.is_alphabetic() {
			let mut identifier = char.to_string();
			while get_char!() && char.is_alphanumeric() {
				identifier.push(char);
			}
			if identifier == "fn" {
				tokens.push(Token::Fn);
			} else if identifier == "extern" {
				tokens.push(Token::Extern);
			} else {
				tokens.push(Token::Identifier(identifier));
			}
		}

		else if char.is_ascii_digit() {
			let mut num_str = char.to_string();
			while get_char!() && (char.is_ascii_digit() || char == '.') {
				num_str.push(char);
			}
			tokens.push(Token::Number(num_str.parse().expect("Error: Invalid number literal!")));
		}

		else if char == '#' {
			while get_char!() && (char != '\n' && char != '\r') {}
		}

		else if char == '+' {
			tokens.push(Token::Operator(BinaryOperator::Add));
			skip_char!();
		} else if char == '-' {
			tokens.push(Token::Operator(BinaryOperator::Sub));
			skip_char!();
		} else if char == '*' {
			tokens.push(Token::Operator(BinaryOperator::Mul));
			skip_char!();
		} else if char == '/' {
			tokens.push(Token::Operator(BinaryOperator::Div));
			skip_char!();
		}

		else {
			// panic!("Invalid character: {char}");
			skip_char!();
		}
	}
	tokens
}