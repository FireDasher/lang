use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	// Keywords
	#[token("let")]
	Let,
	#[token("fn")]
	Fn,
	#[token("struct")]
	Struct,
	#[token("class")]
	Class,

	#[token("if")]
	If,
	#[token("else")]
	Else,
	#[token("elif")]
	Elif,
	#[token("for")]
	For,
	#[token("while")]
	While,
	#[token("loop")]
	Loop,
	#[token("return")]
	Return,

	#[token("import")]
	Import,
	#[token("use")]
	Use,
	#[token("cimport")]
	CImport,
	#[token("cppimport")]
	CppImport,
	#[token("extern")]
	Extern,

	#[token("=")]
	Assign,

	#[token(";")]
	Semi,
	#[token(",")]
	Comma,
	#[token(".")]
	Dot,

	#[token("(")]
	LParen,
	#[token(")")]
	RParen,
	#[token("{")]
	LBrace,
	#[token("}")]
	RBrace,
	#[token("[")]
	LBrack,
	#[token("]")]
	RBrack,

	#[token("@")]
	At,
	#[token("#")]
	Pound,
	#[token("~")]
	Tilde,
	#[token("?")]
	Question,
	#[token(":")]
	Colon,
	#[token("$")]
	Dollar,

	#[token("==")]
	Eq,
	#[token("!=")]
	Ne,
	#[token("<")]
	Lt,
	#[token("<=")]
	Le,
	#[token(">")]
	Gt,
	#[token(">=")]
	Ge,
	#[token("<=>")]
	Cmp,

	#[token("+")]
	Add,
	#[token("+=")]
	AddAssign,
	#[token("-")]
	Sub,
	#[token("-=")]
	SubAssign,
	#[token("*")]
	Mul,
	#[token("*=")]
	MulAssign,
	#[token("/")]
	Div,
	#[token("/=")]
	DivAssign,
	#[token("%")]
	Rem,
	#[token("%=")]
	RemAssign,

	#[token("&")]
	And,
	#[token("&=")]
	AndAssign,
	#[token("|")]
	Or,
	#[token("|=")]
	OrAssign,
	#[token("^")]
	Xor,
	#[token("^=")]
	XorAssign,
	#[token("!")]
	Not,

	#[token("&&")]
	SCAnd,
	#[token("||")]
	SCOr,

	#[token("<<")]
	LShift,
	#[token("<<=")]
	LShiftAssign,
	#[token(">>")]
	RShift,
	#[token(">>=")]
	RShiftAssign,

	// Literals
	#[regex("[[:alpha:]][[:alnum:]]*", |lex| lex.slice().to_string())]
	Ident(String),

	#[regex(r"\d+", |lex| lex.slice().parse::<u64>().expect("Invalid integer literal"))]
	Int(u64),
	#[regex(r"\d+\.\d+(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().expect("Invalid float literal"))]
	Float(f64),

	#[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice().to_string())]
	String(String),

	#[token("false", |_| false)]
	#[token("true", |_| true)]
	Bool(bool),
}