use std::collections::HashMap;

use crate::expression::Operation;
use crate::semantics::*;

pub fn default_operator_table() -> OperatorTable<f64> {
    let unary_ops: Vec<UnaryOp<f64>> = vec![
        UnaryOp {
            symbol: "ln",
            semantics: |x| x.ln(),
        },
        UnaryOp {
            symbol: "log",
            semantics: |x| x.log(10.0),
        },
        UnaryOp {
            symbol: "sin",
            semantics: |x| x.sin(),
        },
        UnaryOp {
            symbol: "cos",
            semantics: |x| x.cos(),
        },
        UnaryOp {
            symbol: "tan",
            semantics: |x| x.tan(),
        },
        UnaryOp {
            symbol: "sqrt",
            semantics: |x| x.sqrt(),
        },
        UnaryOp {
            symbol: "-",
            semantics: |x| (-x),
        },
        UnaryOp {
            symbol: "sgn",
            semantics: |x| x.signum(),
        },
        UnaryOp {
            symbol: "abs",
            semantics: |x| x.abs(),
        },
        UnaryOp {
            symbol: "asin",
            semantics: |x| x.asin(),
        },
        UnaryOp {
            symbol: "acos",
            semantics: |x| x.acos(),
        },
        UnaryOp {
            symbol: "atan",
            semantics: |x| x.atan(),
        },
    ];

    let binary_ops: Vec<BinaryOp<f64>> = vec![
        BinaryOp {
            symbol: "+",
            semantics: |x, y| x+y,
            assoc: Assoc::Left,
            prec: 1,
        },
        BinaryOp {
            symbol: "-",
            semantics: |x, y| x-y,
            assoc: Assoc::Left,
            prec: 1,

        },
        BinaryOp {
            symbol: "*",
            semantics: |x, y| x*y,
            assoc: Assoc::Left,
            prec: 2,
        },
        BinaryOp {
            symbol: "/",
            semantics: |x, y| x/y,
            assoc: Assoc::Left,
            prec: 2,
        },
        BinaryOp {
            symbol: "^",
            semantics: |x, y| x.powf(y),
            assoc: Assoc::Left,
            prec: 3,
        },
    ];

    let consts: Vec<ConstantOp<f64>> = vec![
        ConstantOp {
            symbol: "pi",
            semantics: std::f64::consts::PI,
        },
        ConstantOp {
            symbol: "e",
            semantics: std::f64::consts::E,
        },
    ];

    OperatorTable::new(unary_ops, binary_ops, consts)
}

// Description of an operator supported
// For now we support three types of operators:
// - Unary operators, are written in prefix form
// - Binary operators, are written in infix form
// - Constants,
pub struct OperatorTable<Number: Clone + From<f64>> {
    unary_ops: HashMap<&'static str, UnaryOp<Number>>,
    binary_ops: HashMap<&'static str, BinaryOp<Number>>,
    const_ops: HashMap<&'static str, ConstantOp<Number>>,
}

impl<Number: Clone + From<f64>> OperatorTable<Number> {
    // panics if there is any duplicate symbol
    // TODO: Check that constants and unary symbols don't overlap
    pub fn new(unary: Vec<UnaryOp<Number>>, binary: Vec<BinaryOp<Number>>, consts: Vec<ConstantOp<Number>>) -> OperatorTable<Number> {
        let mut unary_table = HashMap::new();
        let mut binary_table = HashMap::new();
        let mut const_table = HashMap::new();

        for op in unary.into_iter() {
            if let Some(_) = unary_table.insert(op.symbol, op) {
                // Duplicate symbols are not allowed
                panic!("Duplicate unary operator symbol")
            }
        }

        for op in binary.into_iter() {
            if let Some(_) = binary_table.insert(op.symbol, op) {
                // Duplicate symbols are not allowed
                panic!("Duplicate binary operator symbol")
            }
        }

        for op in consts.into_iter() {
            if let Some(_) = const_table.insert(op.symbol, op) {
                // Duplicate symbols are not allowed
                panic!("Duplicate constant symbol")
            }
        }

        OperatorTable {
            unary_ops: unary_table,
            binary_ops: binary_table,
            const_ops: const_table,
        }
    }
}

impl<Number: Clone + From<f64>> Semantics for OperatorTable<Number> {
    type Number = Number;

    fn lookup_unary(&self, symbol: &str) -> Option<&UnaryOp<Number>> {
        self.unary_ops.get(symbol)
    }

    fn lookup_binary(&self, symbol: &str) -> Option<&BinaryOp<Number>> {
        self.binary_ops.get(symbol)
    }
    fn lookup_const(&self, symbol: &str) -> Option<&ConstantOp<Number>> {
        self.const_ops.get(symbol)
    }

    fn has_symbol(&self, symbol: &str) -> bool {
        self.unary_ops.contains_key(symbol) ||
        self.binary_ops.contains_key(symbol) ||
        self.const_ops.contains_key(symbol)
    }

    fn number(&self, num: f64) -> Operation<Number> {
        Operation::Constant(Number::from(num))
    }

    fn xvar(&self) -> Operation<Number> {
        Operation::Variable(|input| input.x)
    }

    fn yvar(&self) -> Operation<Number> {
        Operation::Variable(|input| input.y)
    }
}


