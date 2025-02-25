use std::env;
use std::io;

// Import parser and evaluator
mod parsemath;
use parsemath::ast;
use parsemath::parser::{ParseError, Parser};

// Function to evaluate an arithmetic expression
fn evaluate(expr: &str) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>(); // Remove whitespace
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    Ok(ast::eval(ast)?)
}

// Main CLI function
fn main() {
    println!("Hello! Welcome to Arithmetic expression evaluator.");
    println!("You can calculate value for expression such as 2*3+(4-5)+2^3/4.");
    println!("Allowed numbers: positive, negative and decimals.");
    println!("Supported operations: Add, Subtract, Multiply, Divide, PowerOf(^).");
    println!("Enter your arithmetic expression below:");

    // Check if an expression is passed as a command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let expr = args[1..].join(" ");
        match evaluate(&expr) {
            Ok(val) => println!("The computed number is {}\n", val),
            Err(_) => println!("Error in evaluating expression. Please enter valid expression\n"),
        }
        return; // Exit after evaluation
    }

    // Otherwise, run interactive mode
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match evaluate(input.trim()) {
                Ok(val) => println!("The computed number is {}\n", val),
                Err(_) => println!("Error in evaluating expression. Please enter valid expression\n"),
            },
            Err(error) => println!("error: {}", error),
        }
    }
}
