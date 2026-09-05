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

// Create chumsky parser here
