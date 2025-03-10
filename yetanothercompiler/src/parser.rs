// parser.rs

// This file contains the parser implementation for Python 2.0 grammar.

use crate::lexer;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Identifier(String),
    BinaryOp {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: Option<String>,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        Parser {
            lexer,
            current_token: None,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.next_token();
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, precedence: i32) -> Option<Expr> {
        let mut left = self.parse_primary()?;

        while let Some(token) = &self.current_token {
            let op_precedence = self.get_precedence(token);
            if op_precedence <= precedence {
                continue;
            } else {
                let op = token.clone();
                self.next_token();
                let right = self.parse_expression(op_precedence)?;
                left = Expr::BinaryOp {
                    op: op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
        }

        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match &self.current_token {
            Some(token) => {
                if let Ok(number) = token.parse::<i32>() {
                    self.next_token();
                    Some(Expr::Number(number))
                } else {
                    Some(Expr::Identifier(token.clone()))
                }
            }
            None => None,
        }
    }

    fn get_precedence(&self, token: &str) -> i32 {
        match token {
            "+" => 1,
            "-" => 1,
            "*" => 2,
            "/" => 2,
            _ => 0,
        }
    }
}
