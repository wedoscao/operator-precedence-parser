use std::fmt::Display;

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

    pub fn eval(&self) -> Result<f64> {
        match self {
            Self::Atom(it) => Ok(it.parse::<f64>()?),
            Self::Op(op, operands) => match op {
                '+' => Ok(operands.0.eval()? + operands.1.eval()?),
                '-' => Ok(operands.0.eval()? - operands.1.eval()?),
                '*' => Ok(operands.0.eval()? * operands.1.eval()?),
                '/' => Ok(operands.0.eval()? / operands.1.eval()?),
                '^' => Ok(operands.0.eval()?.powf(operands.1.eval()?)),
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
}
