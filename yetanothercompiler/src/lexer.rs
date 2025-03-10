// lexer.rs

// This file contains the lexer implementation for Python 2.0 grammar.

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<String> {
        // Skip whitespace
        while self.position < self.input.len() {
            if let Some(c) = self.input.chars().nth(self.position) {
                if c.is_whitespace() {
                    self.position += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if self.position >= self.input.len() {
            return None;
        }

        let current_char = self.input.chars().nth(self.position).unwrap();

        if current_char.is_digit(10) {
            // Parse number
            let mut number = String::new();
            while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_digit(10) {
                number.push(self.input.chars().nth(self.position).unwrap());
                self.position += 1;
            }

            return Some(number);
        } else if current_char == '+' || current_char == '-' || current_char == '*' || current_char == '/' {
            // Parse operator
            let op = String::from(current_char.to_string());
            self.position += 1;

            // Skip whitespace after operator
            while self.position < self.input.len() {
                if let Some(c) = self.input.chars().nth(self.position) {
                    if c.is_whitespace() {
                        self.position += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            return Some(op);
        } else if current_char.is_alphabetic() {
            // Parse identifier or keyword
            let mut identifier = String::new();
            while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_alphanumeric() {
                identifier.push(self.input.chars().nth(self.position).unwrap());
                self.position += 1;
            }

            return Some(identifier);
        } else {
            // Unknown token
            let unknown = String::from(current_char.to_string());
            self.position += 1;
            return Some(unknown);
        }
    }
}
