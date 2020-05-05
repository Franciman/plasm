use crate::expression::{Operation};

// Associativity of a binary operator
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Assoc {
    Left,
    Right,
}

// Precedence level of a binary operator
pub type Prec = u32;

// Description of an operator supported
// For now we support three types of operators:
// - Unary operators, are written in prefix form
// - Binary operators, are written in infix form
// - Constants,
pub struct UnaryOp<Number: Clone + From<f32>> {
    pub symbol: &'static str,
    pub semantics: fn (Number) -> Number,
}

impl<Number: Clone + From<f32>> UnaryOp<Number> {
    pub fn operation(&self) -> Operation<Number> {
        Operation::UnaryOperation(self.semantics)
    }
}

pub struct BinaryOp<Number: Clone + From<f32>> {
    pub symbol: &'static str,
    pub assoc: Assoc,
    pub prec: Prec,

    pub semantics: fn (Number, Number) -> Number,
}


impl<Number: Clone + From<f32>> BinaryOp<Number> {
    pub fn operation(&self) -> Operation<Number> {
        Operation::BinaryOperation(self.semantics)
    }
}

pub struct ConstantOp<Number: Clone + From<f32>> {
    pub symbol: &'static str,

    pub semantics: Number,
}


impl<Number: Clone + From<f32>> ConstantOp<Number> {
    pub fn operation(&self) -> Operation<Number> {
        Operation::Constant(self.semantics.clone())
    }
}

pub trait Semantics {
    type Number: Clone + From<f32>;

    fn has_symbol(&self, name: &str) -> bool;

    fn lookup_binary(&self, name: &str) -> Option<&BinaryOp<Self::Number>>;
    fn lookup_unary(&self, name: &str) -> Option<&UnaryOp<Self::Number>>;
    fn lookup_const(&self, name: &str) -> Option<&ConstantOp<Self::Number>>;

    fn number(&self, num: f32) -> Operation<Self::Number>;
    fn xvar(&self) -> Operation<Self::Number>;
    fn yvar(&self) -> Operation<Self::Number>;
}
