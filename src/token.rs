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

	/// `=`
	Assign,

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

	/// `!`
	Not,

	/// `==`
	Eq,
	/// `<`
	Lt,
	/// `>`
	Gt,

	/// `+`
	Add,
	// `+=`
	AddAssign,
	/// `-` Also unary negation
	Sub,
	// `-=`
	SubAssign,
	/// `*`
	Mul,
	// `*=`
	MulAssign,
	/// `/`
	Div,
	// `/=`
	DivAssign,
	/// `%`
	Mod,
	// `%=`
	ModAssign,
	/// `**`
	Pow,
	// `**=`
	PowAssign,

	/// `&`
	And,
	// `&=`
	AndAssign,
	/// `|`
	Or,
	// `|=`
	OrAssign,
	/// `^`
	Xor,
	// `^=`
	XorAssign,

	/// `&&`
	SCAnd,
	/// `||`
	SCOr,
	/// `^^`
	SCXor,

	/// `<<`
	LShift,
	/// `<<=`
	LShiftAssign,
	/// `>>`
	RShift,
	/// `>>=`
	RShiftAssign,

	// Values
	Ident(String),
	Float(f64),
	Int(u64),
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
	pub fn symbol(chararacter: char) -> Token {
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
			'!' => Token::Not,
			'<' => Token::Lt,
			'>' => Token::Gt,
			'-' => Token::Sub,
			'&' => Token::And,
			'|' => Token::Or,
			'+' => Token::Add,
			'*' => Token::Mul,
			'/' => Token::Div,
			'^' => Token::Xor,
			'%' => Token::Mod,
			_ => Token::Null,
		}
	}
}