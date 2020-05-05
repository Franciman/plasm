// Here we define the semantic of an expression

#[derive(Clone)]
pub struct InputSpace<Number: Clone> {
    pub x: Number,
    pub y: Number,
}

pub enum Operation<Number: Clone + From<f32>> {
    BinaryOperation(fn (Number, Number) -> Number),
    UnaryOperation(fn (Number) -> Number),
    Constant(Number),
    Variable(fn (InputSpace<Number>) -> Number),
}

// We represent an expression in its postfix form
// it is a program to be run in a stack machine
pub struct Expression<Number: Clone + From<f32>> {
    ops: Vec<Operation<Number>>,
    // This flag indicates the need
    // to plot the function in the 3d space
    is_3d: bool,
}

impl<Number: Clone + From<f32>> Expression<Number> {
    pub fn new(ops: Vec<Operation<Number>>, is_3d: bool) -> Expression<Number> {
        Expression {
            ops: ops,
            is_3d: is_3d,
        }
    }

    pub fn is_3d(&self) -> bool {
        self.is_3d
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
