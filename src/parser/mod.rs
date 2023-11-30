use std::fmt::Display;

mod ini;
mod rwini;

pub use self::ini::*;
pub use self::rwini::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
  pub row: usize,
  pub col: usize,
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.row, self.col)
  }
}

impl From<(usize, usize)> for Position {
  fn from(value: (usize, usize)) -> Self {
    Self {
      row: value.0,
      col: value.1,
    }
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Range {
  pub start: Position,
  pub end: Position,
}

impl Display for Range {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.start, self.end)
  }
}
#[cfg(test)]
mod test {
  mod ini_parser {
    use crate::parser::IniParser;

    #[test]
    fn test_parse() {
      let parser = IniParser::default();
      assert_eq!(parser.parse("[core]"), Ok([("core", []).into(),].into()));
      assert_eq!(
        parser.parse("[core]\nname: abc\nprice: '''1\n2\n'''"),
        Ok([("core", [("name", "abc"), ("price", "12"),]).into(),].into())
      );
      assert_eq!(
        parser.parse("[core]\nname: abc\nprice: '''1\n2\n'''\n[abc]\ndef:abc"),
        Ok(
          [
            ("core", [("name", "abc"), ("price", "12"),]).into(),
            ("abc", [("def", "abc")]).into(),
          ]
          .into()
        )
      );
    }
  }
}
