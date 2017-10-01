
mod ast;
mod eval;
mod lexer;
mod parser;

use ast::Expr;
use eval::{Context, eval};
use std::io;
use std::io::Write;

/// Produce the string of 1s and 0s representing this number in binary.
fn as_binary_string(x: u16) -> String {
  use std::cmp::min;
  let mut s = String::with_capacity(16);
  for i in (0..16).rev() {
    let digit = min(1, 2_u16.pow(i) & x);
    s.push_str(&format!("{}", digit));
  }
  s
}

fn main() {
  
  println!("Welcome to the bitshift calculator.");
  println!("Numbers are displayed as 16-bit unsigned integers.");
  println!("Assign to variables like so: 'let x = 15'.");
  println!("Type 'exit' when you're done.");
  
  let mut ctx = Context::new();
  loop {
  
    // Get the next line of input.
    let mut input = String::new();  
    print!("$ ");
    io::stdout().flush();
    if let Err(e) = io::stdin().read_line(&mut input) {
      println!("{}", e);
    }
    let input = input.trim();
    if input == "exit" {
      break;
    }
    
    // Lex the program.
    let mut tokens = lexer::lex(input);
    if let Err(e) = tokens {
      println!("{}", e);
      continue;
    }
    let mut tokens = tokens.unwrap();
    
    // Parse the program.
    let prog = parser::parse(&mut tokens);
    if let Err(e) = prog {
      println!("Error: {}", e);
      continue;
    }
    let prog = prog.unwrap();
    
    // Print the result, if there is one.
    let result = eval(&mut ctx, &prog);
    if let Err(e) = result {
      println!("Error: {}", e);
      continue;
    }
    let result = result.unwrap();
    println!("{} ({})", as_binary_string(result), result);
    
  }

}
