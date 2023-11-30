use crate::data::{Ini, IniProperty, IniSection};
use std::{error::Error, fmt::Display};

use super::Range;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum IniParseErrorType {
  EndlessMultiline,
  RepeatedKey,
  UnknownLineInSection,
  UnknownLineBeforeAnySection,
  Unknown,
}

impl Display for IniParseErrorType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Unknown => "Unknown",
        Self::UnknownLineBeforeAnySection => "UnknownLineBeforeAnySection",
        Self::UnknownLineInSection => "UnknownLineInSection",
        Self::RepeatedKey => "RepeatedKey",
        Self::EndlessMultiline => "EndlessMultiline",
      }
    )
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IniParseError {
  pub typ: IniParseErrorType,
  pub range: Range,
  pub msg: String,
}

impl Display for IniParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "type:{}, range:{}, msg:{}",
      self.typ, self.range, self.msg
    )
  }
}

impl Error for IniParseError {}

pub struct IniParser {
  pub error_if_unknown_line_before_any_section: bool,
  pub error_if_unknown_line_in_section: bool,
  pub error_if_repeated_key: bool,
  pub error_if_endless_multiline: bool,
}

impl Default for IniParser {
  fn default() -> Self {
    Self {
      error_if_unknown_line_before_any_section: true,
      error_if_unknown_line_in_section: true,
      error_if_repeated_key: false,
      error_if_endless_multiline: true,
    }
  }
}

impl IniParser {
  pub fn parse(&self, src: impl AsRef<str>) -> Result<Ini, IniParseError> {
    let mut ini = Ini::default();
    let mut current_section: Option<IniSection> = None;
    let mut current_multiline: Option<(String, Range, IniProperty)> = None;
    for (row, line, trimmed_line) in src
      .as_ref()
      .split("\n")
      .enumerate()
      .map(|(index, line)| (index + 1, line, line.trim()))
    {
      if let Some(current_section) = &mut current_section {
        if let Some((end, range, property)) = &mut current_multiline {
          range.end = (row, line.chars().count()).into();
          if trimmed_line.ends_with(end.as_str()) {
            current_section.insert_property((
              property.key().to_owned(),
              property.value().to_owned()
                  + line.trim_end().strip_suffix(end.as_str()).unwrap(),
            ).into());
            current_multiline = None;
          } else {
            property.set_value(property.value().to_owned() + line);
          }
        } else {
          if !trimmed_line.is_empty() && !trimmed_line.starts_with("#") {
            if let Some((key, value)) = line.split_once(':') {
              let trimmed_key = key.trim();
              let trimmed_value = value.trim();
              if trimmed_value.starts_with("\"\"\"") {
                current_multiline = Some((
                  "\"\"\"".to_owned(),
                  Range {
                    start: (
                      row,
                      key.chars().count() + 1 + 1 + value.find("\"\"\"").unwrap(),
                    )
                      .into(),
                    end: (row, line.chars().count()).into(),
                  },
                  (
                    trimmed_key.to_owned(),
                    trimmed_value.strip_prefix("\"\"\"").unwrap().to_owned()
                  ).into(),
                ));
              } else if trimmed_value.starts_with("'''") {
                current_multiline = Some((
                  "'''".to_owned(),
                  Range {
                    start: (
                      row,
                      key.chars().count() + 1 + 1 + value.find("'''").unwrap(),
                    )
                      .into(),
                    end: (row, key.chars().count() + 1 + value.chars().count()).into(),
                  },
                  (
                    trimmed_key.to_owned(),
                    trimmed_value.strip_prefix("'''").unwrap().to_owned(),
                  ).into(),
                ));
              } else {
                if current_section
                  .insert_property((
                    trimmed_key.to_owned(),
                    trimmed_value.to_owned(),
                  ).into())
                  .is_some()
                  && self.error_if_repeated_key
                {
                  return Err(IniParseError {
                    typ: IniParseErrorType::RepeatedKey,
                    range: Range {
                      start: (row, 1).into(),
                      end: (row, line.chars().count()).into(),
                    },
                    msg: "".to_owned(),
                  });
                }
              }
            } else if trimmed_line.starts_with('[') && trimmed_line.ends_with(']') {
              ini
                .insert_section(current_section.clone());
              *current_section = IniSection::new(trimmed_line
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .to_owned()
              );
            } else if self.error_if_unknown_line_in_section {
              return Err(IniParseError {
                typ: IniParseErrorType::UnknownLineInSection,
                range: Range {
                  start: (row, 1).into(),
                  end: (row, line.chars().count()).into(),
                },
                msg: "".to_owned(),
              });
            }
          }
        }
      } else {
        if trimmed_line.starts_with("[") && trimmed_line.ends_with("]") {
          current_section = Some(IniSection::new(trimmed_line
            .strip_prefix("[")
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .to_owned()));
        } else if self.error_if_unknown_line_before_any_section
          && !trimmed_line.is_empty()
          && !trimmed_line.starts_with("#")
        {
          return Err(IniParseError {
            typ: IniParseErrorType::UnknownLineBeforeAnySection,
            range: Range {
              start: (row, 1).into(),
              end: (row, line.chars().count()).into(),
            },
            msg: "".to_owned(),
          });
        }
      }
    }
    if self.error_if_endless_multiline && current_multiline.is_some() {
      return Err(IniParseError {
        typ: IniParseErrorType::EndlessMultiline,
        range: current_multiline.unwrap().1,
        msg: "".to_owned(),
      });
    }
    if let Some(current_section) = current_section {
      ini
        .insert_section(current_section);
    }
    Ok(ini)
  }
}
