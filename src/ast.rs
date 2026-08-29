/// Binary operators
pub enum Biop {
	Add,
	Sub,
	Mul,
	Div,
	Mod,

	And,
	Or,
	Xor,

	LShift,
	RShift,

	SCAnd,
	SCOr,

	Eq,
	Lt,
	Gt,
	Lte,
	Gte,
}

/// Binary assignments
pub enum Assign {
	Assign,

	AddAssign,
	SubAssign,
	MulAssign,
	DivAssign,
	ModAssign,

	AndAssign,
	OrAssign,
	XorAssign,

	LShiftAssign,
	RShiftAssign,
}

/// Unary operators
pub enum Unop {
	Not,
	Negate,
}

/// Embraced code
pub struct Block {
	pub statements: Vec<Statement>,
	pub end: Option<Box<Expr>>,
}

/// Function paramater
pub struct Param {
	pub name: String,
	pub ty: String,
}

pub enum AccessSpecifier {
	/// This field can only be accessed by the class/struct it's in
	Private,
	/// Only from the class it's in OR inheriters
	Protected,
	/// It can be accessed anywhere
	Public,
}
/// Class/struct stuff
pub enum Stuff {
	Field {
		access: AccessSpecifier,
		name: String,
		ty: String,
		default: Option<Expr>,
	},
	Statement(Statement), // Varaibles not allowed here
}
pub enum DeclareMode {
	Let, // Mutable
	Const, // Evaluated at compile time
	Static, // Mutable & lazy loaded like in C++
}

pub enum Expr {
	/// Dot operator
	Access(Box<Expr>, Box<Expr>),

	/// An operator
	Biop(Biop, Box<Expr>, Box<Expr>),
	Unop(Unop, Box<Expr>),

	As(Box<Expr>, String),

	/// fUnction callation
	Call(Box<Expr>, Vec<Expr>),

	/// Braces notation
	Init(Box<Expr>, Vec<(String, Expr)>),

	/// jUSt a Block
	Block(Block),

	Ident(String),
	/// Literals, varaibles accessars are ident regardeless of their type
	String(String),
	Int(u64),
	Float(f64),

	// Control flow
	If(Box<Expr>, Block),
	Elif(Box<Expr>, Block),
	Else(Block),
	// for var in expr {...} can return something using break
	For(String, Box<Expr>, Block),
	/// INFINITE loop (requires break)
	Loop(Block),
	/// while expr {...} can also return something using break
	While(Box<Expr>, Block),
}

pub enum Statement {
	Import(String),

	/// For both C and C++; first is what to define second is what to define to, blank for nothing
	CDefine(String, String),

	CImport(String),
	CppImport(String),

	/// The identifier of the variable to declare, and the expression to initialize itt with or None for no initializement
	Declare(DeclareMode, String, Option<Expr>),
	/// The operator, the identifier to be assigned to, and the expression to assign
	Assign(Assign, String, Expr),

	/// Traditional returnment, bypasses blocks ending with expressions
	Return(Expr),

	/// continue:label;
	Continue(Option<String>),
	/// break:label return_value;
	Break(Option<String>, Expr),

	Func {
		access: AccessSpecifier,
		name: String,
		params: Vec<Param>,
		return_type: String,
		code: Block,
	},

	Struct {
		access: AccessSpecifier,
		name: String,
		contents: Vec<Stuff>,
	},

	Class {
		access: AccessSpecifier,
		name: String,
		inherits: Option<String>,
		contents: Vec<Stuff>,
	},
}