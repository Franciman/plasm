use crate::semantics::*;
use crate::operator_descr::OperatorTable;
use honestintervals::IntervalSet;
use honestintervals::transc::Transc;

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



pub fn interval_arithmetic_operator_table() -> OperatorTable<IntervalSet<f64>> {
    let unary_ops: Vec<UnaryOp<IntervalSet<f64>>> = vec![
        UnaryOp {
            symbol: "-",
            semantics: |x| (-x),
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
    ];

    let binary_ops: Vec<BinaryOp<IntervalSet<f64>>> = vec![
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
            semantics: |x, y| x.pow(y),
            assoc: Assoc::Left,
            prec: 3,
        },
    ];

    let consts: Vec<ConstantOp<IntervalSet<f64>>> = vec![
        ConstantOp {
            symbol: "pi",
            semantics: IntervalSet::singleton(std::f64::consts::PI),
        },
        ConstantOp {
            symbol: "e",
            semantics: IntervalSet::singleton(std::f64::consts::E),
        },
    ];

    OperatorTable::new(unary_ops, binary_ops, consts)
}