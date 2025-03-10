// main.rs

mod lexer;
mod parser;
mod semantic_analysis;
mod codegen;

fn main() {
    let input = String::from("x + 2");
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);

    // Parse the input
    let expr = parser.parse().expect("Failed to parse expression");

    // Perform semantic analysis
    let mut semantic_analyzer = semantic_analysis::SemanticAnalyzer::new();
    semantic_analyzer.declare_variable("x".to_string(), 5); // Declare variable x
    semantic_analyzer.analyze(&expr).expect("Semantic analysis failed");

    // Generate code
    let codegen = codegen::CodeGenerator::new();
    let generated_code = codegen.generate(&expr);

    println!("Generated Code: {}", generated_code);
}
