use crate::parser::Expr;

// codegen.rs

// This file contains the code generation implementation for Python 2.0 grammar.

pub struct CodeGenerator {
    // Fields for code generation can be added here
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            // Initialize fields if necessary
        }
    }

    pub fn generate(&self, expr: &crate::parser::Expr) -> String {
        match expr {
            Expr::Number(value) => value.to_string(),
            Expr::Identifier(name) => name.clone(),
            Expr::BinaryOp { op, left, right } => {
                let left_code = self.generate(left);
                let right_code = self.generate(right);
                format!("({} {} {})", left_code, op, right_code)
            }
        }
    }
}
