use std::{error::Error, fmt::Display};

use crate::data::Rwini;

use super::{IniParseError, IniParser, Range};

pub struct RwiniParser {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RwiniParseErrorType {
  SyntaxError(IniParseError),
  Unknown,
}

impl Display for RwiniParseErrorType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Unknown => "Unknown",
        Self::SyntaxError(_) => "SyntaxError",
      }
    )
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RwiniParseError {
  pub typ: RwiniParseErrorType,
  pub range: Range,
  pub msg: String,
}

impl Display for RwiniParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "type:{}, range:{}, msg:{}",
      self.typ, self.range, self.msg
    )
  }
}

impl Error for RwiniParseError {}

impl RwiniParser {
  pub fn parse(&self, src: impl AsRef<str>) -> Result<Rwini, RwiniParseError> {
    match IniParser::default().parse(src) {
      Err(e) => Err(RwiniParseError {
        range: e.range,
        typ: RwiniParseErrorType::SyntaxError(e),
        msg: "".to_owned(),
      }),
      Ok(ini) => {
        
        Ok(Rwini { extras: ini, ..Default::default() })
      },
    }
  }
}
