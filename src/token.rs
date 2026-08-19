#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Fn,
	Extern,
	Identifier(String),
	Number(f64),
	Operator(BinaryOperator),
}