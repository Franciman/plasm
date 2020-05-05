use crate::expression::{Operation};

// Associativity of a binary operator
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Assoc {
    Left,
    Right,
}

// Precedence level of a binary operator
pub type Prec = u32;


pub trait UnaryOpTrait<Number: Clone> {
    fn symbol(&self) -> &'static str;
    fn semantics(&self) -> Operation<Number>;
}

pub trait BinaryOpTrait<Number: Clone> {
    fn symbol(&self) -> &'static str;
    fn semantics(&self) -> Operation<Number>;

    fn assoc(&self) -> Assoc;
    fn prec(&self) -> Prec;
}

pub trait ConstantTrait<Number: Clone> {
    fn symbol(&self) -> &'static str;
    fn semantics(&self) -> Operation<Number>;
}

pub trait Semantics {
    type Number: Clone;
    type UnaryOp: UnaryOpTrait<Self::Number>;
    type BinaryOp: BinaryOpTrait<Self::Number>;
    type Constant: ConstantTrait<Self::Number>;

    fn has_symbol(&self, name: &str) -> bool;

    fn lookup_binary(&self, name: &str) -> Option<&Self::BinaryOp>;
    fn lookup_unary(&self, name: &str) -> Option<&Self::UnaryOp>;
    fn lookup_const(&self, name: &str) -> Option<&Self::Constant>;

    fn number(&self, num: f32) -> Operation<Self::Number>;
    fn xvar(&self) -> Operation<Self::Number>;
    fn yvar(&self) -> Operation<Self::Number>;
}
