use std::collections::HashMap;
use std::marker::PhantomData;

use crate::expression::Operation;
use crate::semantics::*;

pub fn default_operator_table() -> OperatorTable<PointStyle> {
    let unary_ops = vec![
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

    let binary_ops = vec![
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

    let consts = vec![
        ConstantOp {
            symbol: "pi",
            semantics: std::f32::consts::PI,
        },
        ConstantOp {
            symbol: "e",
            semantics: std::f32::consts::E,
        },
    ];

    OperatorTable::new(unary_ops, binary_ops, consts)
}

// Description of an operator supported
// For now we support three types of operators:
// - Unary operators, are written in prefix form
// - Binary operators, are written in infix form
// - Constants,
pub struct UnaryOp {
    pub symbol: &'static str,
    pub semantics: fn (f32) -> f32,
}

impl UnaryOpTrait<f32> for UnaryOp {
    fn symbol(&self) -> &'static str {
        self.symbol
    }
    fn semantics(&self) -> Operation<f32> {
        Operation::UnaryOperation(self.semantics)
    }
}

pub struct BinaryOp {
    pub symbol: &'static str,
    pub assoc: Assoc,
    pub prec: Prec,

    pub semantics: fn (f32, f32) -> f32,
}

impl BinaryOpTrait<f32> for BinaryOp {
    fn symbol(&self) -> &'static str {
        self.symbol
    }

    fn semantics(&self) -> Operation<f32> {
        Operation::BinaryOperation(self.semantics)
    }

    fn assoc(&self) -> Assoc {
        self.assoc
    }

    fn prec(&self) -> Prec {
        self.prec
    }
}

pub struct ConstantOp {
    pub symbol: &'static str,

    pub semantics: f32,
}

impl ConstantTrait<f32> for ConstantOp {
    fn symbol(&self) -> &'static str {
        self.symbol
    }

    fn semantics(&self) -> Operation<f32> {
        Operation::Constant(self.semantics)
    }
}

pub struct OperatorTable<SemanticsStyle> {
    unary_ops: HashMap<&'static str, UnaryOp>,
    binary_ops: HashMap<&'static str, BinaryOp>,
    const_ops: HashMap<&'static str, ConstantOp>,

    // This just suppresses the error that the generic param is not used
    _marker: PhantomData<SemanticsStyle>,
}

pub struct PointStyle { }
pub struct IntervalStyle { }

impl<SemanticsStyle> OperatorTable<SemanticsStyle> {
    // panics if there is any duplicate symbol
    // TODO: Check that constants and unary symbols don't overlap
    pub fn new(unary: Vec<UnaryOp>, binary: Vec<BinaryOp>, consts: Vec<ConstantOp>) -> OperatorTable<SemanticsStyle> {
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

            _marker: PhantomData,
        }
    }
}

impl Semantics for OperatorTable<PointStyle> {
    type Number = f32;
    type UnaryOp = UnaryOp;
    type BinaryOp = BinaryOp;
    type Constant = ConstantOp;

    fn lookup_unary(&self, symbol: &str) -> Option<&UnaryOp> {
        self.unary_ops.get(symbol)
    }

    fn lookup_binary(&self, symbol: &str) -> Option<&BinaryOp> {
        self.binary_ops.get(symbol)
    }
    fn lookup_const(&self, symbol: &str) -> Option<&ConstantOp> {
        self.const_ops.get(symbol)
    }

    fn has_symbol(&self, symbol: &str) -> bool {
        self.unary_ops.contains_key(symbol) ||
        self.binary_ops.contains_key(symbol) ||
        self.const_ops.contains_key(symbol)
    }

    fn number(&self, num: f32) -> Operation<f32> {
        Operation::Constant(num)
    }

    fn xvar(&self) -> Operation<f32> {
        Operation::Variable(|input| input.0)
    }

    fn yvar(&self) -> Operation<f32> {
        Operation::Variable(|input| input.1)
    }
}


