use std::{fmt::Display, error::Error};

// #region Parse error
#[derive(Debug)]
pub enum ParseError {
  InvalidInput(String),
	ParseIntError(std::num::ParseIntError)
}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidInput(s) => write!(f, "{}", s),
			Self::ParseIntError(err) => err.fmt(f)
    }
  }
}

impl Error for ParseError {}

#[macro_export]
macro_rules! parse_err {
  ($($arg:tt)*) => {
    Err(ParseError::InvalidInput(format!($($arg)*)))
  };
}
#[macro_export]
macro_rules! box_parse_err {
  ($($arg:tt)*) => {
    Err(Box::new(ParseError::InvalidInput(format!($($arg)*))))
  };
}
