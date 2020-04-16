mod expression;
mod operator_descr;
mod parser;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let unary_ops = vec![
        operator_descr::UnaryOp {
            symbol: "ln",
            semantics: |x| x.ln(),
        },
        operator_descr::UnaryOp {
            symbol: "log",
            semantics: |x| x.log(10.0),
        },
        operator_descr::UnaryOp {
            symbol: "sin",
            semantics: |x| x.sin(),
        },
        operator_descr::UnaryOp {
            symbol: "cos",
            semantics: |x| x.cos(),
        },
        operator_descr::UnaryOp {
            symbol: "tan",
            semantics: |x| x.tan(),
        },
        operator_descr::UnaryOp {
            symbol: "sqrt",
            semantics: |x| x.sqrt(),
        },
        operator_descr::UnaryOp {
            symbol: "-",
            semantics: |x| (-x),
        },
    ];

    let binary_ops = vec![
        operator_descr::BinaryOp {
            symbol: "+",
            semantics: |x, y| x+y,
            assoc: operator_descr::Assoc::Left,
            prec: 1,
        },
        operator_descr::BinaryOp {
            symbol: "-",
            semantics: |x, y| x-y,
            assoc: operator_descr::Assoc::Left,
            prec: 1,

        },
        operator_descr::BinaryOp {
            symbol: "*",
            semantics: |x, y| x*y,
            assoc: operator_descr::Assoc::Left,
            prec: 2,
        },
        operator_descr::BinaryOp {
            symbol: "/",
            semantics: |x, y| x/y,
            assoc: operator_descr::Assoc::Left,
            prec: 2,
        },
        operator_descr::BinaryOp {
            symbol: "^",
            semantics: |x, y| x.powf(y),
            assoc: operator_descr::Assoc::Left,
            prec: 3,
        },
    ];

    let consts = vec![
        operator_descr::ConstantOp {
            symbol: "pi",
            semantics: std::f64::consts::PI,
        },
        operator_descr::ConstantOp {
            symbol: "e",
            semantics: std::f64::consts::E,
        },
    ];
    let table = operator_descr::OperatorTable::new(unary_ops, binary_ops, consts);

    let mut input = String::new();

    loop {
        input.clear();
        print!("f(x,y) = ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        if input == "quit\n" {
            break;
        }


        let expr = parser::Parser::new(input.as_str(), &table).parse().unwrap();
        let res = expr.eval((10.0, 10.0));
        println!("f(10, 10) = {}", res);
    }
    Ok(())
}
