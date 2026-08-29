/// Binary operators
pub enum Biop {
	Add,
	Sub,
	Mul,
	Div,
	Rem,

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
	Le,
	Ge,
	Cmp,
}

/// Binary assignments
pub enum Assign {
	Assign,

	AddAssign,
	SubAssign,
	MulAssign,
	DivAssign,
	RemAssign,

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
	pub statements: Vec<Stmt>,
	pub end: Option<BExpr>,
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
		default: Option<BExpr>,
	},
	Stmt(Stmt), // Varaibles not allowed here
}
pub enum DeclareMode {
	Let, // Mutable
	Const, // Evaluated at compile time
	Static, // Mutable & lazy loaded like in C++
}

pub enum Expr {
	/// a.b
	Access(BExpr, BExpr),

	/// a[b]
	Index(BExpr, BExpr),

	/// An operator
	Biop(Biop, BExpr, BExpr),
	Unop(Unop, BExpr),

	// Var as Type converts it
	As(BExpr, BExpr),

	/// fUnction callation
	Call(BExpr, Vec<Expr>),

	/// Braces notation
	Init(BExpr, Vec<(String, Expr)>),

	/// jUSt a Block
	Block(Block),

	Ident(String),
	/// Literals, varaibles accessars are ident regardeless of their type
	String(String),
	Int(u64),
	Float(f64),

	// Control flow
	If {condition: BExpr, then: Block},
	Elif {condition: BExpr, then: Block},
	Else {then: Block},
	// for var in expr {...} can return something using break
	For {var: String, within: BExpr, doing: Block},
	/// INFINITE loop (requires break)
	Loop {doing: Block},
	/// while expr {...} can also return something using break
	While {condition: BExpr, doing: Block},
}
type BExpr = Box<Expr>;

pub enum Stmt {
	Import {path: String},

	/// For both C and C++
	CDefine {name: String, value: String},

	CImport {path: String},
	CppImport {path: String},

	/// Declares a variable/constant
	Declare {mode: DeclareMode, name: String, value: Option<BExpr>},

	Assign {mode: Assign, to: BExpr, value: BExpr},

	/// Traditional returnment, bypasses blocks ending with expressions
	Return {value: BExpr},

	/// continue:label;
	Continue {label: Option<String>},
	/// break:label return_value;
	Break {label: Option<String>, value: BExpr},

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