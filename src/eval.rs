
use ast::Expr;
use ast::Prog;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EvalError {
  msg: String,
}

macro_rules! err {
  ($msg:expr) => (Err(EvalError::new($msg)));
}

impl EvalError {
  fn new(msg:&str) -> EvalError {
    EvalError {
      msg: msg.to_string(),
    }
  }
}

impl fmt::Display for EvalError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for EvalError {
  fn description(&self) -> &str {
    &self.msg
  }
}

type EvalResult<T> = Result<T, EvalError>;

/// A context tracks what value a variable is bound to.
pub struct Context {
  vars: HashMap<String, u16>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      vars: HashMap::new(),
    }
  }
}

impl Context {

  pub fn insert(&mut self, var: &str, val: u16) {
    self.vars.insert(var.to_string(), val);
  }
  
  pub fn lookup(&self, var: &str) -> EvalResult<u16> {
    match self.vars.get(var) {
      Some(&ch) => Ok(ch),
      None => err!(&format!("Variable '{}' not found.", var)),
    }
  }
  
}

pub fn eval(ctx: &mut Context, prog: &Prog) -> EvalResult<u16> {
  match *prog {
    Prog::Expression(ref expr) => {
      let v = eval_expr(ctx, expr)?;
      Ok(v)
    },
    Prog::Assign(ref name, ref expr) => {
      let v = eval_expr(ctx, expr)?;
      ctx.insert(name, v);
      Ok(v)
    },
  }
}

pub fn eval_expr(ctx: &mut Context, expr: &Expr) -> EvalResult<u16> {
  use self::Expr::*;
  match *expr {
  
    Const(val) => Ok(val),
    
    Var(ref name) => Ok(ctx.lookup(name)?),
               
    BinaryOper(ref op, ref e1, ref e2) => {
      use ast::BinOp::*;
      let e1 = eval_expr(ctx, e1)?;
      let e2 = eval_expr(ctx, e2)?;
      let result = match *op {
        BitAnd      => e1 & e2,
        BitOr       => e1 | e2,
        BitXor      => e1 ^ e2,
        BitShLeft   => e1 << e2,
        BitShRight  => e1 >> e2,
        Plus        => e1 + e2,
        Minus       => e1 - e2,
        Times       => e1 * e2,
        Divide      => e1 / e2,
      };
      Ok(result)
    },
    
    UnaryOper(ref op, ref e) => {
      use ast::UnaryOp::*;
      let e = eval_expr(ctx, e)?;
      let result = match *op {
        BitNeg  => !e,
      };
      Ok(result)
    },
  }
}
