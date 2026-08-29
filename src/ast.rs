#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
	Add,
	Sub,
	Mul,
	Div,
}

pub trait ExprAST {}

pub struct NumberAST {
	pub val: f64,
} impl ExprAST for NumberAST {}

pub struct VariableAST {
	pub name: String,
} impl ExprAST for VariableAST {}

pub struct BinaryAST {
	pub op: BinaryOperator,
	pub lhs: Box<dyn ExprAST>,
	pub rhs: Box<dyn ExprAST>,
} impl ExprAST for BinaryAST {}

pub struct CallAST {
	pub callee: String,
	pub args: Vec<Box<dyn ExprAST>>,
} impl ExprAST for CallAST {}

pub struct PrototypeAST {
	pub name: String,
	pub args: Vec<String>,
}

pub struct FunctionAST {
	pub proto: Box<PrototypeAST>,
	pub body: Box<dyn ExprAST>,
}