use std::str::Chars;
use std::iter::Peekable;
use std::result::Result;

use crate::expression::{Operation, ExprType, Expression};
use crate::semantics::*;

enum Token {
    Operator(String),
    XVar,
    YVar,
    Number(f64),
    LeftParen,
    RightParen,
    Equal,
    Eof,
    Error(&'static str), // Error with explaination of the error
}

struct Tokenizer<'s, S: Semantics> {
    input: Peekable<Chars<'s>>,
    table: &'s S,
}

impl<'s, S: Semantics> Tokenizer<'s, S> {
    fn new(input: &'s str, table: &'s S) -> Tokenizer<'s, S> {
        return Tokenizer {
            input: input.chars().peekable(),
            table: table,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.input.peek().map_or(false, |c| c.is_whitespace()) {
            self.input.next();
        }
    }

    // Returns the lexed number and the number of digits it is composed of
    fn read_integer(&mut self) -> (f64, u32) {
        let mut digits = 0;
        let mut number = 0.0;

        while self.input.peek().map_or(false, |c| c.is_digit(10)) {
            digits += 1;
            number *= 10.0;
            number += self.input.next().unwrap().to_digit(10).unwrap() as f64;
        }

        return (number, digits)
    }

    fn read_number(&mut self) -> Token {
        let (integer_part, _) = self.read_integer();

        // Parse decimal part
        if self.input.peek().map_or(false, |c| *c == '.') {
            self.input.next();
            // Now we need digits for the decimal part, if none is found, it is an error
            if !self.input.peek().map_or(false, |c| c.is_digit(10)) {
                return Token::Error("Missing decimal part in floating point number");
            } else {
                let (decimal_digits, digits) = self.read_integer();

                // Interpret the decimal part correctly, now it is an integer,
                // but we want to divide it by 10^{digits}
                let decimal_part_magnitude = (10 as i32).pow(digits) as f64;
                let decimal_part = decimal_digits / decimal_part_magnitude;

                return Token::Number(integer_part + decimal_part);
            }
        } else {
            return Token::Number(integer_part);
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        // We have three types of identifiers:
        // - special functions: +,-,*,/,^
        // - variables: x, y
        // - functions: log, sin, cos,...
        // The second and third type are words composed only of alphabetic characters,
        // we use this fact to distinguish and correctly recognize identifiers

        if self.input.peek().map_or(false, char::is_ascii_alphabetic) {
            // Ok, this is either a variable or a function
            while self.input.peek().map_or(false, char::is_ascii_alphabetic) {
                identifier.push(self.input.next().unwrap());
            }
        } else {
            // We are sure we are not at EOF
            identifier.push(self.input.next().unwrap());
        }

        // First check for variables
        if identifier == "x" {
            return Token::XVar
        } else if identifier == "y" {
            return Token::YVar
        } else {
            match self.table.has_symbol(&identifier.as_str()) {
                false => Token::Error("Unrecognized identifier"),
                true => return Token::Operator(identifier),
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.input.peek() {
            None => Token::Eof,
            Some(c) => {
                match c {
                    '(' => {
                        self.input.next();
                        return Token::LeftParen
                    },
                    ')' => {
                        self.input.next();
                        return Token::RightParen
                    },
                    '=' => {
                        self.input.next();
                        return Token::Equal
                    },
                    c if c.is_digit(10) => self.read_number(),
                    _ => self.read_identifier(),


                }
            },

        }
    }
}

// Parses an input string and produces
// an Expression, that represents the semantics of the parsed expression
// [it is basically the expression in postfix order]
struct Parser<'s, S: Semantics> {
    tokenizer: Tokenizer<'s, S>,
    table: &'s S,
    look_ahead: Token,

    // Internal state representing the current expression
    // that is being parsed
    operations: Vec<Operation<S::Number>>,
    is_3d: bool,
}

pub fn parse<S: Semantics>(input: & str, table: &S) -> Result<Expression<S::Number>, &'static str>  {
    let mut parser = Parser {
        tokenizer: Tokenizer::new(input, table),
        table: table,
        look_ahead: Token::Eof,
        operations: Vec::new(),
        is_3d: false,
    };

    parser.look_ahead = parser.tokenizer.next_token();

    parser.parse_expr(0)?;
    match parser.look_ahead {
        // 'Tis an implicit function
        Token::Equal => {
            parser.next_token();
            // Parse right-hand side
            parser.parse_expr(0)?;

            // Now we have this equation: lhs = rhs
            // Plotting it is equivalent to plotting lhs - rhs = 0
            // So we represent the function in this way and mark it as implicit function

           // We are sure - is a binary operator
            let op = parser.table.lookup_binary("-").unwrap();
            parser.operations.push(op.operation());
            Ok(Expression::new(parser.operations, ExprType::ExprImplicit))
        }
        Token::Eof => {
            let expr_type = if parser.is_3d { ExprType::Expr3d } else { ExprType::Expr2d };
            Ok(Expression::new(parser.operations, expr_type))
        },
        _ => Err("Unexpected token at end of expression")
    }
}

impl<'s, S: Semantics> Parser<'s, S> {
    fn parse_expr(&mut self, curr_prec: u32) -> Result<(), &'static str> {
        self.parse_prefix()?;

        loop {
            match &self.look_ahead {
                Token::Eof => return Ok(()),
                Token::Equal => return Ok(()),
                Token::RightParen => return Ok(()),
                Token::Operator(name) => {
                    let op: &BinaryOp<S::Number>;
                    let is_implicit_product: bool;
                    match self.table.lookup_binary(name) {
                        None => {
                            // If it is not a binary operator it is a prefix starter,
                            // so here there must be an implicit product [or an error which is going to be caught later]
                            // We are sure * is a binary operator
                            op = self.table.lookup_binary("*").unwrap();
                            is_implicit_product = true;
                        },
                        Some(op_descr) => {
                            op = op_descr;
                            is_implicit_product = false;
                        }
                    }
                    let curr_op_binds_tighter = curr_prec < op.prec
                                              || (curr_prec == op.prec && op.assoc == Assoc::Right);
                    if curr_op_binds_tighter {
                        // This operator has higher precedence,
                        // so it binds tighter
                        self.parse_operation_rhs(op, is_implicit_product)?;
                        continue; // Keep on looping
                    } else {
                        // We are done, this operator shouldn't be consumed here
                        // it binds less tightly
                        return Ok(());
                    }
                },
                Token::Error(err) => return Err(err),
                _ => {
                    // If there is no binary operator, then we can try parsing a prefix
                    // and insert an implicit product here

                    // We are sure * is a binary operator
                    let op: &BinaryOp<S::Number> = self.table.lookup_binary("*").unwrap();

                    let curr_op_binds_tighter = curr_prec < op.prec
                                              || (curr_prec == op.prec && op.assoc == Assoc::Right);
                    if curr_op_binds_tighter {
                        // This operator has higher precedence,
                        // so it binds tighter
                        self.parse_operation_rhs(op, true)?;
                        continue; // Keep on looping
                    } else {
                        // We are done, this operator shouldn't be consumed here
                        // it binds less tightly
                        return Ok(());
                    }
                }
            }
        }
    }

    // If is_implicit_op true, then we don't have to consume next token
    fn parse_operation_rhs(&mut self, op: &BinaryOp<S::Number>, is_implicit_op: bool) -> Result<(), &'static str> {
        if !is_implicit_op {
            self.next_token();
        }
        self.parse_expr(op.prec)?;
        self.operations.push(op.operation());
        Ok(())
    }

    fn parse_prefix(&mut self) -> Result<(), &'static str> {
        match self.look_ahead {
            Token::Operator(ref name) => {
                // Check if the operator is a constant or an unary const.
                // If it is both, it is an error!
                match (self.table.lookup_const(&name), self.table.lookup_unary(&name)) {
                    (None, None) => Err("Unexpected prefix"),
                    (Some(c), None) => {
                        self.next_token();
                        self.operations.push(c.operation());
                        Ok(())
                    },
                    (None, Some(op)) => {
                        self.next_token();
                        self.parse_prefix()?;
                        self.operations.push(op.operation());
                        Ok(())
                    },
                    (_,_) => Err("Ambiguous operator name")
                }
            }
            Token::Number(n) => {
                self.next_token();
                self.operations.push(self.table.number(n));
                Ok(())
            },
            Token::XVar => {
                self.next_token();
                self.operations.push(self.table.xvar());
                Ok(())
            },
            Token::YVar => {
                self.next_token();
                self.operations.push(self.table.yvar());
                self.is_3d = true;
                Ok(())
            },
            Token::LeftParen => {
                self.next_token();
                self.parse_expr(0)?;
                // Make sure parentheses are well balanced
                match self.look_ahead {
                    Token::RightParen => {
                        self.next_token();
                        Ok(())
                    },
                    _ => Err("Missing )"),
                }
            },
            Token::Error(e) => Err(e),
            Token::Eof => Err("Unexpected end of input"),
            _ => Err("Unexpected prefix"),
        }
    }

    fn next_token(&mut self) {
        self.look_ahead = self.tokenizer.next_token();
    }
}
