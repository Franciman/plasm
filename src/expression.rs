// Here we define the semantic of an expression

#[derive(Clone)]
pub struct InputSpace<Number: Clone> {
    pub x: Number,
    pub y: Number,
}

pub enum Operation<Number: Clone + From<f64>> {
    BinaryOperation(fn (Number, Number) -> Number),
    UnaryOperation(fn (Number) -> Number),
    Constant(Number),
    Variable(fn (InputSpace<Number>) -> Number),
}

pub enum ExprType {
    Expr2d,
    Expr3d,
    ExprImplicit,
}

// We represent an expression in its postfix form
// it is a program to be run in a stack machine
pub struct Expression<Number: Clone + From<f64>> {
    ops: Vec<Operation<Number>>,

    expr_type: ExprType,
}

impl<Number: Clone + From<f64>> Expression<Number> {
    pub fn new(ops: Vec<Operation<Number>>, expr_type: ExprType) -> Expression<Number> {
        Expression {
            ops: ops,
            expr_type: expr_type,
        }
    }

    pub fn expr_type(&self) -> &ExprType {
        &self.expr_type
    }

    // Evaluate the projection of the expression on the xz plane [ this is what you want for 2d functions]
    // i.e. the second coordinate is always set to 0
    pub fn eval_2d(&self, x: Number) -> Number {
        self.eval(InputSpace {
            x: x,
            y: Number::from(0.0),
        })
    }

    pub fn eval_3d(&self, x: Number, y: Number) -> Number {
        self.eval(InputSpace {
            x: x,
            y: y,
        })
    }

    pub fn eval_implicit(&self, x: Number, y: Number) -> Number {
        self.eval(InputSpace {
            x: x,
            y: y,
        })
    }

    fn eval(&self, input: InputSpace<Number>) -> Number {
        let mut stack = Vec::new();

        for op in self.ops.iter() {
            match op {
                Operation::Constant(c) => stack.push(c.clone()),
                Operation::Variable(f) => stack.push(f(input.clone())),
                Operation::UnaryOperation(f) => {
                    let arg = stack.pop().unwrap();
                    stack.push(f(arg))
                },
                Operation::BinaryOperation(f) => {
                    let arg2 = stack.pop().unwrap();
                    let arg1 = stack.pop().unwrap();
                    stack.push(f(arg1, arg2))
                }
            }
        }
        stack.pop().unwrap()
    }
}
