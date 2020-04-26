// Here we define the semantic of an expression
pub type Number = f32;
pub type InputSpace = f32;

pub enum Operation {
    BinaryOperation(fn (Number, Number) -> Number),
    UnaryOperation(fn (Number) -> Number),
    Constant(Number),
    Variable(fn (InputSpace) -> Number),
}

// We represent an expression in its postfix form
// it is a program to be run in a stack machine
pub struct Expression {
    ops: Vec<Operation>,
}

impl Expression {
    pub fn new(ops: Vec<Operation>) -> Expression {
        Expression {
            ops: ops,
        }
    }
    pub fn eval(&self, input: InputSpace) -> Number {
        let mut stack = Vec::new();

        for op in self.ops.iter() {
            match op {
                Operation::Constant(c) => stack.push(*c),
                Operation::Variable(f) => stack.push(f(input)),
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
