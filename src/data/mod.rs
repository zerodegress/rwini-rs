mod ini;
mod rwini;

pub use self::ini::*;
pub use self::rwini::*;

#[cfg(test)]
mod test {
  use crate::data::Ini;

  #[test]
  fn test_ini_to_string() {
    assert_eq!(
      (Into::<Ini>::into([("core", [("name", "1"), ("key", "value"),]).into()])).to_string(),
      "[core]\nkey:value\nname:1"
    );
    assert_eq!(
      (Into::<Ini>::into([("core", [("name", "1"), ("key", "value"),]).into()])).to_string(),
      (Into::<Ini>::into([("core", [("name", "1"), ("key", "value"),]).into()])).to_string(),
    );
  }
}
