
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

/// This is thrown whenever there is an error during the lexing process.
#[derive(Debug)]
pub struct LexError {
  msg: String,
}

impl LexError {
  fn new(msg:&str) -> LexError {
    LexError {
      msg: msg.to_string(),
    }
  }
}

impl fmt::Display for LexError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for LexError {
  fn description(&self) -> &str {
    &self.msg
  }
}

type LexResult<T> = Result<T, LexError>;

/// Short-hand for generating lexing errors.
macro_rules! err {
  ($msg:expr) => (Err(LexError::new($msg)));
}





#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
  Ident(String), Num(u16), Oper(Operator), LeftParen, RightParen, Keyw(Keyword), Equals
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Keyword {
  Let,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Operator {
  Plus, Minus, Times, Divide,
  BitNeg, BitAnd, BitOr, BitXor,
  BitShLeft, BitShRight
}

pub fn lex(input: &str) -> LexResult<Vec<Token>> {
  let mut lexer = Lexer {
    input: input.chars().peekable(),
    tokens: Vec::new(),
  };
  while !lexer.done() {
    lexer.skip_whitespace()?;
    lexer.lex_token()?;
  }
  Ok(lexer.tokens)
}

struct Lexer<'l> {
  input: Peekable<Chars<'l>>,
  tokens: Vec<Token>,
}

fn is_symbol(c: char) -> bool {
  let symbols = vec!['+', '*', '/', '-', '&', '|', '^', '!', '<', '>'];
  symbols.contains(&c)
}

fn as_keyword(s: &str) -> Option<Keyword> {
  match s {
    "let" => Some(Keyword::Let),
    _ => None,
  }
}

impl<'l> Lexer<'l> {
  fn peek(&mut self) -> Option<&char> {
    self.input.peek()
  }
  
  fn next(&mut self) -> LexResult<char> {
    match self.input.next() {
      Some(ch) => Ok(ch),
      None     => err!("Expected character but there wasn't one."),
    }
  }
  
  fn skip_whitespace(&mut self) -> LexResult<()> {
    while let Some(&ch) = self.peek() {
      if ch.is_whitespace() {
        self.next()?;
      } else {
        break;
      }
    }
    Ok(())
  }
  
  fn done(&mut self) -> bool {
    self.peek().is_none()
  }
  
  fn lex_token(&mut self) -> LexResult<()> {
    if self.done() { return err!("No characters left while lexing token.") };
    let ch = *self.peek().unwrap();
    if ch.is_numeric() {
      self.lex_num()?;
    } else if ch.is_alphabetic() {
      let name = self.lex_ident()?;
    } else if is_symbol(ch) {
      self.lex_operator()?;
    } else if ch == '(' {
      self.tokens.push(Token::LeftParen);
      self.next()?;
    } else if ch == ')' {
      self.tokens.push(Token::RightParen);
      self.next()?;
    } else if ch == '=' {
      self.tokens.push(Token::Equals);
      self.next();
    } else {
      return err!(&format!("Couldn't lex token. Failed on character {}", ch));
    };
    Ok(())
  }
  
  fn lex_num(&mut self) -> LexResult<()> {
  
    // Must have at least one digit in number.
    let ch = self.next()?;
    
    if !ch.is_numeric() {
      return err!("Non-digit found while lexing number.");
    }
    let mut num = String::new();
    num.push(ch);
  
    // Keep adding digits to the number.
    while let Some(&ch) = self.peek() {
      if ch.is_numeric() {
        num.push(ch);
        self.next()?;
      } else if ch.is_alphabetic() {
        return err!(&format!("Expected digit while parsing number but found '{}'", ch));
      } else {
        break;
      }
    }
  
    // Parse as u16.
    match num.parse::<u16>() {
      Ok(val) => self.tokens.push(Token::Num(val)),
      Err(e)  => return err!(&format!("Failed to parse {} as u16: {}", num, e)),
    }
    Ok(())
  
  }
  
  fn lex_ident(&mut self) -> LexResult<()> {
  
    // An identifier must start with an alphabetic character.
    let ch = self.next()?;
    if !ch.is_alphabetic() {
      return err!("An identifier must start with an alphabetic character.");
    }
    let mut iden = String::new();
    iden.push(ch);
    
    // Keep adding characters to the identifier.
    while let Some(&ch) = self.peek() {
      if ch.is_alphabetic() || ch.is_numeric() {
        iden.push(ch);
        self.next()?;
      } else {
        break;
      }
    }
    
    // Check if it is an identifier or a keyword.
    let token = match as_keyword(&iden) {
      Some(kw) => Token::Keyw(kw),
      None => Token::Ident(iden),
    };
    self.tokens.push(token);
    Ok(())
  
  }
  
  fn lex_operator(&mut self) -> LexResult<()> {
    use self::Token::*;
    use self::Operator::*;
    let token = match self.next()? {
    
      '+' => Oper(Plus),
      '-' => Oper(Minus),
      '*' => Oper(Times),
      '/' => Oper(Divide),
      '&' => Oper(BitAnd),
      '|' => Oper(BitOr),
      '^' => Oper(BitXor),
      '!' => Oper(BitNeg),
      
      '<' => {
        if let Some(&'<') = self.peek() {
          self.next()?;
          Oper(BitShLeft)
        } else {
          return err!("Error while lexing '<' (did you mean '<<'?)")
        }
      },
      
      '>' => {
        if let Some(&'>') = self.peek() {
          self.next()?;
          Oper(BitShRight)
        } else {
          return err!("Error while lexing '>' (did you mean '>>'?)");
        }
      },
        
      _ => return err!("Error while lexing operator"),
       
      };
      
    self.tokens.push(token);
    Ok(())
  }
  
}
