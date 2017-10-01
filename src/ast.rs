
use std::fmt;

pub enum UnaryOp {
  BitNeg,
}

pub enum BinOp {
  BitAnd,
  BitOr,
  BitXor,
  BitShLeft,
  BitShRight,
  Plus,
  Minus,
  Times,
  Divide,
}

#[derive(Debug)]
pub enum Prog {
  Expression(Expr),
  Assign(String, Expr),
}

pub enum Expr {
  Const(u16),
  Var(String),
  BinaryOper(BinOp, Box<Expr>, Box<Expr>),
  UnaryOper(UnaryOp, Box<Expr>),
}

impl fmt::Debug for UnaryOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::UnaryOp::*;
    write!(f, "{}", match *self {
      BitNeg => "!",
    })
  }
}

impl fmt::Debug for BinOp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::BinOp::*;
    write!(f, "{}", match *self {
      BitAnd      => "&",
      BitOr       => "|",
      BitXor      => "^",
      BitShLeft   => "<<",
      BitShRight  => ">>",
      Plus        => "+",
      Minus       => "-",
      Times       => "*",
      Divide      => "/",
    })
  }
}

impl fmt::Debug for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::Expr::*;
    write!(f, "{}", match *self {
      Const(val)
          => format!("{}", val),
      Var(ref name)
          => name.to_string(),
      BinaryOper(ref op, ref e1, ref e2)
          => format!("({:?} {:?} {:?})", op, e1, e2),
      UnaryOper(ref op, ref e)
          => format!("({:?} {:?})", op, e),
    })
  }
}

