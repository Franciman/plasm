use std::str::Chars;
use std::iter::Peekable;
use std::result::Result;

use crate::operator_descr::OperatorTable;
use crate::operator_descr::Assoc;
use crate::expression::{Number, Operation, Expression};

enum Token {
    Operator(String),
    XVar,
    YVar,
    Number(Number),
    LeftParen,
    RightParen,
    Eof,
    Error(&'static str), // Error with explaination of the error
}

struct Tokenizer<'s> {
    input: Peekable<Chars<'s>>,
    table: &'s OperatorTable
}

impl<'s> Tokenizer<'s> {
    fn new(input: &'s str, table: &'s OperatorTable) -> Tokenizer<'s> {
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

    fn read_number(&mut self) -> Token {
        let mut number: Number = 0.0;
        while self.input.peek().map_or(false, |c| c.is_digit(10)) {
            number *= 10.0;
            number += self.input.next().unwrap().to_digit(10).unwrap() as Number;
        }
        return Token::Number(number)
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
pub struct Parser<'s> {
    tokenizer: Tokenizer<'s>,
    table: &'s OperatorTable,
    look_ahead: Token,
}

type ExprInterpretation = Vec<Operation>;

impl<'s> Parser<'s> {
    pub fn new(input: &'s str, table: &'s OperatorTable) -> Parser<'s> {

        let mut res = Parser {
            tokenizer: Tokenizer::new(input, table),
            table: table,
            look_ahead: Token::Eof,
        };

        res.look_ahead = res.tokenizer.next_token();

        return res;
    }

    pub fn parse(&mut self) -> Result<Expression, &'static str> {
        let res = self.parse_expr(0)?;
        match self.look_ahead {
            Token::Eof => Ok(Expression::new(res)),
            _ => Err("Unexpected token at end of expression")
        }
    }

    fn parse_expr(&mut self, curr_prec: u32) -> Result<ExprInterpretation, &'static str> {
        let mut lhs = self.parse_prefix()?;

        loop {
            match &self.look_ahead {
                Token::Eof => return Ok(lhs),
                Token::RightParen => return Ok(lhs),
                Token::Operator(name) => {
                    match self.table.lookup_binary(name) {
                        None => return Err("Expected binary operator"),
                        Some(op) => {
                            let curr_op_binds_tighter = curr_prec < op.prec
                                                      || (curr_prec == op.prec && op.assoc == Assoc::Right);
                            if curr_op_binds_tighter {
                                // This operator has higher precedence,
                                // so it binds tighter
                                self.next_token();
                                let mut rhs = self.parse_expr(op.prec)?;
                                lhs.append(&mut rhs);
                                lhs.push(Operation::BinaryOperation(op.semantics));
                                continue; // Keep on looping
                            } else {
                                // We are done, this operator shouldn't be consumed here
                                // it binds less tightly
                                return Ok(lhs);
                            }
                        }
                    }

                },
                Token::Error(err) => return Err(err),
                _ => return Err("Expected binary operator"),
            }
        }
    }

    fn parse_prefix(&mut self) -> Result<ExprInterpretation, &'static str> {
        match self.look_ahead {
            Token::Operator(ref name) => {
                // Check if the operator is a constant or an unary const.
                // If it is both, it is an error!
                match (self.table.lookup_const(&name), self.table.lookup_unary(&name)) {
                    (None, None) => Err("Unexpected prefix"),
                    (Some(c), None) => {
                        self.next_token();
                        Ok(vec![Operation::Constant(c.semantics)])
                    },
                    (None, Some(op)) => {
                        self.next_token();
                        let mut arg = self.parse_prefix()?;
                        arg.push(Operation::UnaryOperation(op.semantics));
                        Ok(arg)
                    },
                    (_,_) => Err("Ambiguous operator name")
                }
            }
            Token::Number(n) => {
                self.next_token();
                Ok(vec![Operation::Constant(n)])
            },
            Token::XVar => {
                self.next_token();
                Ok(vec![Operation::Variable(|input| input.0)])
            },
            Token::YVar => {
                self.next_token();
                Ok(vec![Operation::Variable(|input| input.1)])
            },
            Token::LeftParen => {
                self.next_token();
                let sub_expr = self.parse_expr(0)?;
                // Make sure parentheses are well balanced
                match self.look_ahead {
                    Token::RightParen => {
                        self.next_token();
                        Ok(sub_expr)
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
