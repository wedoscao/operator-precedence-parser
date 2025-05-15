use anyhow::{Result, bail};

#[derive(Clone, Copy, Debug)]
pub enum Token {
    Atom(char),
    Op(char),
    Eof,
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Result<Self> {
        let mut tokens = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '.' => Ok(Token::Atom(c)),
                '+' | '-' | '*' | '/' | '^' | '=' => Ok(Token::Op(c)),
                c => bail!("Bad character: {}", c),
            })
            .collect::<Result<Vec<Token>>>()?;
        tokens.reverse();
        Ok(Self { tokens })
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

pub fn get_bq(op: char) -> Result<u8> {
    match op {
        '=' => Ok(1),
        '+' | '-' => Ok(2),
        '*' | '/' => Ok(3),
        '^' => Ok(4),
        _ => bail!("Unknown operator: {}", op),
    }
}
