#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Null,

	// Keywords
	Let,
	True,
	False,

	Fn,
	Struct,
	Class,

	If,
	Else,
	For,
	While,
	Loop,
	Return,

	Import,
	Use,
	CImport,
	CppImport,
	Extern,

	/// `;`
    Semi,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBrack,
    /// `]`
    RBrack,
    /// `@`
    At,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `:`
    Colon,
    /// `$`
    Dollar,
    /// `=`
    Eq,
    /// `!`
    Bang,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `-`
    Minus,
    /// `&`
    And,
    /// `|`
    Or,
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,

	// Values
	Ident(String),
	Float(f64),
	Int(i64),
}

impl Token {
	pub fn keyword(indentifier: &str) -> Token {
		match indentifier {
			"fn" => Token::Fn,
			"let" => Token::Let,
			"true" => Token::True,
			"false" => Token::False,
			"if" => Token::If,
			"else" => Token::Else,
			"return" => Token::Return,
			"for" => Token::For,
			"while" => Token::While,
			"loop" => Token::Loop,
			"import" => Token::Import,
			"use" => Token::Use,
			"cimport" => Token::CImport,
			"cppimport" => Token::CppImport,
			"extern" => Token::Extern,
			_ => Token::Ident(indentifier.to_string()),
		}
	}
	pub fn operator(chararacter: char) -> Token {
		match chararacter {
			';' => Token::Semi,
			',' => Token::Comma,
			'.' => Token::Dot,
			'(' => Token::LParen,
			')' => Token::RParen,
			'{' => Token::LBrace,
			'}' => Token::RBrace,
			'[' => Token::LBrack,
			']' => Token::RBrack,
			'@' => Token::At,
			'#' => Token::Pound,
			'~' => Token::Tilde,
			'?' => Token::Question,
			':' => Token::Colon,
			'$' => Token::Dollar,
			'=' => Token::Eq,
			'!' => Token::Bang,
			'<' => Token::Lt,
			'>' => Token::Gt,
			'-' => Token::Minus,
			'&' => Token::And,
			'|' => Token::Or,
			'+' => Token::Plus,
			'*' => Token::Star,
			'/' => Token::Slash,
			'^' => Token::Caret,
			'%' => Token::Percent,
			_ => Token::Null,
		}
	}
}