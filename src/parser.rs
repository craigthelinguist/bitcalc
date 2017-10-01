
use ast::Expr;
use ast::Prog;
use lexer::{Keyword, Token, Operator};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub struct ParseError {
  msg: String,
}

macro_rules! err {
  ($msg:expr) => (Err(ParseError::new($msg)));
}

impl ParseError {

  fn new(msg:&str) -> ParseError {
    ParseError {
      msg: msg.to_string(),
    }
  }

}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for ParseError {
  fn description(&self) -> &str {
    &self.msg
  }
}

type ParseResult<T> = Result<T, ParseError>;

pub fn parse(tokens: &mut Vec<Token>) -> ParseResult<Prog> {
  let mut parser = Parser::new(&tokens);
  parser.parse()
}

/// Use the shunting yard algorithm to convert infix notation into prefix
/// notation. For example, a + b becomes + a b. 
fn shunting_yard<'l>(tokens: &mut [Token]) -> ParseResult<Vec<Token>> {

  use self::Token::*;
  use self::Operator::*;
  
  // Build a map of operators to their priority. A higher priority means it
  // binds more tightly. The order of precedence is based on C.
  let mut priority = HashMap::new();
  priority.insert(BitOr, 8);
  priority.insert(BitXor, 10); 
  priority.insert(BitAnd, 12);
  priority.insert(BitShRight, 15);
  priority.insert(BitShLeft, 15);
  priority.insert(Plus, 20);
  priority.insert(Minus, 20);
  priority.insert(Times, 30);
  priority.insert(Divide, 30);
  priority.insert(BitNeg, 40);
  
  // We want to treat the entire expression as being enclosed in brackets. To
  // do this, make the stack start with a right bracket on it, and perform one
  // more "pop left bracket" operation after this main loop.
  let mut output: Vec<Token> = Vec::new();
  let mut stack: Vec<Token> = Vec::new();
  stack.push(RightParen);
  
  for token in tokens.iter().rev() {
    match token.clone() {
    
      // These tokens are not allowed in an expression.
      Keyw(k) => return err!(&format!("keyword '{:?}' found while parsing expression.", k)),
      Equals => return err!("equality sign '=' found while parsing expression."),
      
      Ident(_) | Num(_) => output.push(token.clone()),
      
      RightParen => stack.push(token.clone()),
      
      LeftParen => {
        loop {
          let top = stack.pop().expect("Mismatched brackets, expected right paren.");
          match top {
            Oper(_) => output.push(top.clone()),
            RightParen => break,
            _ => return err!("mismatched brackets, expected right paren."),
          };
        };
      },
      
      // Pop all operators of higher precedence.
      Oper(ref op) => {
        while let Some(ref top) = stack.pop() {
          match *top {
            Oper(ref op2) => {
              let p1 = priority.get(op)
                .expect(&format!("No priority given for {:?}", op));
              let p2 = priority.get(op2)
                .expect(&format!("No priority given for {:?}", op2));
              if p2 >= p1 {
                output.push(top.clone());
              } else {
                stack.push(top.clone());
                break;
              };
            },
            
            LeftParen | RightParen => {
              stack.push(top.clone()); break;
            }
            
            _ => return err!("Pushed non-bracket or non-operator on stack."),
          }
        };
        stack.push(token.clone());
      }
    }
  }

  // Pretend there's an extra left paren at the end of the expression.
  loop {
    let top = stack.pop().expect("Mismatched brackets, expected right paren.");
    match top {
      Oper(_) => output.push(top.clone()),
      RightParen => break,
      _ => return err!("mismatched brackets, expected right paren."),
    };
  };

  
  output.reverse();
  Ok(output) 
}

struct Parser {
  tokens: Vec<Token>,
  index: usize,
}

impl Parser {

  fn new(tokens: &Vec<Token>) -> Parser {
    Parser {
      tokens: tokens.clone(),
      index: 0,
    }
  }

  /// Look at the next token, but don't advance the token stream.
  fn peek(&mut self) -> ParseResult<Token> {
    if self.done() {
      err!("Expected token while peeking but found nothing.")
    } else {
      Ok(self.tokens[self.index].clone())
    }
  }
  
  /// Check if the parser is at the end of the token stream.
  fn done(&mut self) -> bool {
    self.index >= self.tokens.len()
  }
  
  /// Get the next token in the token stream, if it exists. Otherwise,
  /// a ParseError is thrown.
  fn next(&mut self) -> ParseResult<Token> {
    if self.done() {
      err!("Expected token but found nothing.")
    } else {
      self.index += 1;
      Ok(self.tokens[self.index - 1].clone())
    }
  }
  
  /// Perform the shunting yard algorithm on the rest of the input to make it
  /// adhere to the order of operations. The input vector will be transformed
  /// in place.
  ///
  /// This is a little inefficient since it does a bit of copying.
  fn shunting_yard(&mut self) -> ParseResult<()> {
  
    // Figure out how to reorder this expression.
    let reordering;
    {
      let tokens_to_parse = &mut self.tokens[self.index..];
      reordering = shunting_yard(tokens_to_parse)?;
    }
    
    // Copy new values over. Note that shunting yard strips the brackets, so
    // reordering may not be the same length as self.tokens[self.index..].
    let num_brackets_stripped = (self.tokens.len() - self.index) - reordering.len();
    for i in 0..reordering.len() {
      self.tokens[self.index + i] = reordering[i].clone();
    }
    
    // Pop off the last few entries. The number to pop is the number of brackets
    // that were stripped by shunting.
    for i in 0..num_brackets_stripped {
      self.tokens.pop();
    }
    Ok(())
  }

  /// Parse a program, which is either a single assignment or an expression.
  fn parse(&mut self) -> ParseResult<Prog> {
    let token = self.peek()?.clone();
    let prog = match token {
    
      // An assignment.
      Token::Keyw(Keyword::Let) => {
        self.next()?;
        let name = self.parse_ident()?;
        if self.peek()? != Token::Equals {
          return err!("Expected '=' while parsing assignment.");
        }
        self.next()?;
        self.shunting_yard()?;
        let expr = self.parse_expr()?;
        Prog::Assign(name, expr)
      },
      
      // An expression.
      _ => {
        self.shunting_yard()?;
        Prog::Expression(self.parse_expr()?)
      },
    
    };

    // Check we are at the end of the program.
    if !self.done() {
      return err!(&format!("Extra token {:?} found after program {:?}",
                  self.peek().unwrap(), prog));
    }
    Ok(prog)
  }
  
  /// Parse an expression, which could be a constant, variable,
  /// a unary operator, or a binary operator.
  fn parse_expr(&mut self) -> ParseResult<Expr> {
    
    let tok = self.peek()?.clone();
    
    match tok {
      
      Token::Ident(ref name) => {
        self.next()?;
        Ok(Expr::Var(name.clone()))
      },
      
      Token::Num(num) => {
        self.next()?;
        Ok(Expr::Const(num))
      },
      
      Token::Oper(ref op) => {
        use self::Operator::*;
        match *op {
          BitNeg => self.parse_uop(),
          
          Plus | Minus | Times | Divide |
          BitAnd | BitOr | BitXor |
          BitShLeft | BitShRight => self.parse_bop(),
        }
      }
      
      Token::LeftParen | Token::RightParen => 
        err!("Found left paren and right paren while parsing, but these /
              should have been eliminated during shunting yard phase."),
      
      Token::Equals =>
        err!("Illegal sign '=' found while parsing expression."),
        
      Token::Keyw(kw) =>
        err!(&format!("Keyword '{:?}' found while parsing expression", kw)),
      
    }
  }
  
  /// Parse the next token as an identifier.
  fn parse_ident(&mut self) -> ParseResult<String> {
    let tok = self.next()?.clone();
    match tok {
      Token::Ident(name) => Ok(name),
      _ => err!(&format!("Wanted identifier but found {:?}", tok)),
    }
  }
  
  /// Parse a unary operator and its arguments.
  fn parse_uop(&mut self) -> ParseResult<Expr> {
    use ast::UnaryOp;
    let tok = self.next()?.clone();
    match tok {
      Token::Oper(op) =>
        match op {
          Operator::BitNeg => {
            let e = self.parse_expr()?;
            Ok(Expr::UnaryOper(UnaryOp::BitNeg, Box::new(e)))
          },
          _ => err!("Non-unary operator found while parsing unary operation."),
        },
      _ => err!("Non-operator found while parsing unary operation."),        
    }
  }
  
  /// Parse a binary operator and its arguments.
  fn parse_bop(&mut self) -> ParseResult<Expr> {
    use ast::BinOp;
    let tok = self.next()?.clone();
    match tok {
    
      Token::Oper(op) => {
        let e1 = Box::new(self.parse_expr()?);
        let e2 = Box::new(self.parse_expr()?);
        match op {
          Operator::Plus =>
            Ok(Expr::BinaryOper(BinOp::Plus, e1, e2)),
          Operator::Minus =>
            Ok(Expr::BinaryOper(BinOp::Minus, e1, e2)),
          Operator::Times =>
            Ok(Expr::BinaryOper(BinOp::Times, e1, e2)),
          Operator::Divide =>
            Ok(Expr::BinaryOper(BinOp::Divide, e1, e2)),
          Operator::BitShLeft =>
            Ok(Expr::BinaryOper(BinOp::BitShLeft, e1, e2)),
          Operator::BitShRight =>
            Ok(Expr::BinaryOper(BinOp::BitShRight, e1, e2)),
          Operator::BitAnd =>
            Ok(Expr::BinaryOper(BinOp::BitAnd, e1, e2)),
          Operator::BitOr =>
            Ok(Expr::BinaryOper(BinOp::BitOr, e1, e2)),
          Operator::BitXor =>
            Ok(Expr::BinaryOper(BinOp::BitXor, e1, e2)),
          _ =>
            err!("Non-binary operator found while parsing binary operator."),
        }
      },
      
      _ => err!("Non-operator found while parsing binary operation."),
    }
  }
  
}

