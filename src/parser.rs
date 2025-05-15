use std::{collections::HashMap, fmt::Display};

use crate::lexer::{self, Lexer, Token};
use anyhow::{Result, bail};

pub enum Expression {
    Atom(String),
    Op(char, Box<(Expression, Expression)>),
}

impl Expression {
    pub fn from_str(input: &str) -> Result<Self> {
        let mut lexer = Lexer::new(input)?;
        parse_to_expression(&mut lexer, 0)
    }

    pub fn is_assignment(&self) -> bool {
        match self {
            Expression::Op(op, _) => *op == '=',
            Expression::Atom(_) => false,
        }
    }

    pub fn assign(&self, map: &mut HashMap<char, f64>) -> Result<()> {
        match self {
            Expression::Atom(_) => bail!("Invalid variable"),
            Expression::Op(_, operands) => {
                if !self.is_assignment() {
                    bail!("Not being an assignment")
                };

                match &operands.0 {
                    Expression::Op(_, _) => bail!("Invalid variable"),
                    Expression::Atom(it) => {
                        if it.len() > 1 {
                            bail!("Invalid variable")
                        };
                        if it.chars().find(|x| x.is_ascii_digit()).is_some() {
                            bail!("Invalid variable")
                        };
                        let key = it.clone().pop().unwrap();
                        map.insert(key, operands.1.eval(map)?);
                    }
                };
            }
        };

        Ok(())
    }

    pub fn eval(&self, map: &HashMap<char, f64>) -> Result<f64> {
        match self {
            Self::Atom(it) => {
                if it.len() > 1 || it.chars().find(|x| x.is_ascii_digit()).is_some() {
                    return Ok(it.parse::<f64>()?);
                }
                let key = it.clone().pop().unwrap();
                match map.get(&key) {
                    Some(it) => Ok(*it),
                    None => bail!("Variable is not assigned: {}", key),
                }
            }
            Self::Op(op, operands) => match op {
                '+' => Ok(operands.0.eval(map)? + operands.1.eval(map)?),
                '-' => Ok(operands.0.eval(map)? - operands.1.eval(map)?),
                '*' => Ok(operands.0.eval(map)? * operands.1.eval(map)?),
                '/' => Ok(operands.0.eval(map)? / operands.1.eval(map)?),
                '^' => Ok(operands.0.eval(map)?.powf(operands.1.eval(map)?)),
                _ => bail!("Unknown operator: {}", op),
            },
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Atom(it) => write!(f, "{}", it),
            Expression::Op(op, operands) => {
                write!(f, "({}", op)?;
                write!(f, "({} {})", operands.0, operands.1)?;
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

fn parse_to_expression(lexer: &mut Lexer, min_bq: u8) -> Result<Expression> {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => {
            let mut it = it.to_string();
            handle_atom_lhs(lexer, &mut it);
            Expression::Atom(it)
        }
        Token::Op(op) => {
            if op != '+' || op != '-' {
                bail!("Bad token: {}", op)
            }
            let mut it = op.to_string();
            handle_op_lhs(lexer, &mut it)?;
            Expression::Atom(it)
        }
        t => bail!("Bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => bail!("Bad token: {:?}", t),
        };

        let bq = lexer::get_bq(op)?;
        if bq <= min_bq {
            break;
        }
        lexer.next();
        let rhs = parse_to_expression(lexer, bq)?;
        lhs = Expression::Op(op, Box::new((lhs, rhs)));
    }
    Ok(lhs)
}

fn handle_atom_lhs(lexer: &mut Lexer, it: &mut String) {
    match lexer.peek() {
        Token::Atom(_) => {
            if let Token::Atom(a) = lexer.next() {
                it.push(a);
            }
            handle_atom_lhs(lexer, it);
        }
        _ => return,
    }
}

fn handle_op_lhs(lexer: &mut Lexer, it: &mut String) -> Result<()> {
    match lexer.peek() {
        Token::Atom(_) => {
            if let Token::Atom(a) = lexer.next() {
                it.push(a);
            };
            handle_atom_lhs(lexer, it);
            Ok(())
        }
        t => bail!("Bad token: {:?}", t),
    }
}

#[cfg(test)]
mod test_parser {
    use super::*;

    fn test_pattern(input: &str, expected: &str) {
        let expected = expected.to_string();
        let result = Expression::from_str(input).unwrap().to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_1() {
        test_pattern("1", "1");
    }

    #[test]
    fn test_2() {
        test_pattern("-1", "-1");
    }

    #[test]
    fn test_3() {
        test_pattern("1+2", "(+(1 2))");
    }

    #[test]
    fn test_4() {
        test_pattern("1+2*4+3^3", "(+((+(1 (*(2 4)))) (^(3 3))))");
    }

    #[test]
    fn test_5() {
        test_pattern("-1--2*4+-3", "(+((-(-1 (*(-2 4)))) -3))");
    }

    #[test]
    fn test_6() {
        test_pattern("1.1", "1.1");
    }

    #[test]
    fn test_7() {
        test_pattern("111", "111");
    }

    #[test]
    fn test_8() {
        test_pattern("a=1+4+9", "(=(a (+((+(1 4)) 9))))");
    }

    #[test]
    fn test_9() {
        test_pattern("a", "a");
    }
}
