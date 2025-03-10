// semantic_analysis.rs

// This file contains the semantic analysis implementation for Python 2.0 grammar.

use std::collections::HashMap;
use crate::parser::Expr;

#[derive(Debug)]
pub struct SemanticAnalyzer {
    variables: HashMap<String, i32>, // Store variable names and their types
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            variables: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Number(_) => Ok(()),
            Expr::Identifier(name) => {
                if !self.variables.contains_key(name) {
                    return Err(format!("Undefined variable: {}", name));
                }
                Ok(())
            }
            Expr::BinaryOp { left, right, .. } => {
                self.analyze(left)?;
                self.analyze(right)?;
                Ok(())
            }
        }
    }

    pub fn declare_variable(&mut self, name: String, value: i32) {
        self.variables.insert(name, value);
    }
}
