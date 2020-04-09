use std::collections::HashMap;

use crate::expression::Number;

// Associativity of a binary operator
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Assoc {
    Left,
    Right,
}

// Precedence level of a binary operator
type Prec = u32;

// Description of an operator supported
// For now we support three types of operators:
// - Unary operators, are written in prefix form
// - Binary operators, are written in infix form
// - Constants,
pub struct UnaryOp {
    pub symbol: &'static str,
    pub semantics: fn (Number) -> Number,
}

pub struct BinaryOp {
    pub symbol: &'static str,
    pub assoc: Assoc,
    pub prec: Prec,

    pub semantics: fn (Number, Number) -> Number,
}

pub struct ConstantOp {
    pub symbol: &'static str,

    pub semantics: Number,
}

pub struct OperatorTable {
    unary_ops: HashMap<&'static str, UnaryOp>,
    binary_ops: HashMap<&'static str, BinaryOp>,
    const_ops: HashMap<&'static str, ConstantOp>,
}


impl OperatorTable {
    // Returns None if there is any duplicate symbol
    // TODO: Check that constants and unary symbols don't overlap
    pub fn new(unary: Vec<UnaryOp>, binary: Vec<BinaryOp>, consts: Vec<ConstantOp>) -> Option<OperatorTable> {
        let mut unary_table = HashMap::new();
        let mut binary_table = HashMap::new();
        let mut const_table = HashMap::new();

        for op in unary.into_iter() {
            if let Some(_) = unary_table.insert(op.symbol, op) {
                // Duplicate symbols are not allowed
                return None
            }
        }

        for op in binary.into_iter() {
            if let Some(_) = binary_table.insert(op.symbol, op) {
                // Duplicate symbols are not allowed
                return None
            }
        }

        for op in consts.into_iter() {
            if let Some(_) = const_table.insert(op.symbol, op) {
                // Duplicate symbols are not allowed
                return None
            }
        }

        Some(OperatorTable {
            unary_ops: unary_table,
            binary_ops: binary_table,
            const_ops: const_table,
        })
    }

    pub fn lookup_unary(&self, symbol: &str) -> Option<&UnaryOp> {
        self.unary_ops.get(symbol)
    }

    pub fn lookup_binary(&self, symbol: &str) -> Option<&BinaryOp> {
        self.binary_ops.get(symbol)
    }
    pub fn lookup_const(&self, symbol: &str) -> Option<&ConstantOp> {
        self.const_ops.get(symbol)
    }

    pub fn has_symbol(&self, symbol: &str) -> bool {
        self.unary_ops.contains_key(symbol) ||
        self.binary_ops.contains_key(symbol) ||
        self.const_ops.contains_key(symbol)
    }
}


