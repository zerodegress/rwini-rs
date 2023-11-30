use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  ops::Index, mem::swap,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Ini {
  pub sections: HashMap<String, HashMap<String, String>>,
}

impl Ini {
  pub fn insert_section(&mut self, section: IniSection) -> Option<IniSection> {
    self.sections
      .insert(section.name.clone(), section.properties)
      .map(|sec| (section.name, sec).into())
  }

  pub fn remove_section(&mut self, name: &str) -> Option<IniSection> {
    self.sections
      .remove(name)
      .map(|sec| (name.to_owned(), sec).into())
  }
}

impl ToString for Ini {
  fn to_string(&self) -> String {
    format!(
      "{}",
      self
        .sections
        .iter()
        .map(
          |(name, props)| 
            format!(
              "[{}]\n{}", 
              name, 
              props.iter()
                .map(|(k, v)| format!("{}:{}", k, v))
                  .collect::<Vec<_>>()
                  .join("\n"),
            )
        )
        .collect::<Vec<_>>()
        .join("\n")
    )
  }
}

impl<const N: usize> From<[IniSection; N]> for Ini {
  fn from(value: [IniSection; N]) -> Self {
    Self {
      sections: HashMap::from_iter(
        value.into_iter()
          .map(|sec| (sec.name, sec.properties))
      )
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct IniSection {
  name: String,
  properties: HashMap<String, String>,
}

impl IniSection {
  pub fn new(name: String) -> Self {
    Self {
      name,
      properties: HashMap::default(),
    }
  }

  pub fn insert_property(&mut self, property: IniProperty) -> Option<IniProperty> {
    self.properties
      .insert(property.key.clone(), property.value)
      .map(|prop| (property.key, prop).into())
  }

  pub fn remove_property(&mut self, key: &str) -> Option<IniProperty> {
    self.properties
      .remove(key)
      .map(|prop| (key.to_owned(), prop).into())
  }

  pub fn contains_key(&self, key: &str) -> bool {
    self.properties
      .contains_key(key)
  }

  pub fn value(&self, key: &str) -> Option<&String> {
    self.properties
      .get(key)
  }

  pub fn value_mut(&mut self, key: &str) -> Option<&mut String> {
    self.properties
      .get_mut(key)
  }
}

impl ToString for IniSection {
  fn to_string(&self) -> String {
    format!(
      "[{}]\n{}",
      self.name,
      self
        .properties
        .values()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
    )
  }
}

impl<const N: usize> From<(&str, [(&str, &str); N])> for IniSection {
  fn from(value: (&str, [(&str, &str); N])) -> Self {
    Self {
      name: value.0.to_owned(),
      properties: HashMap::from_iter(value.1.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned()))),
    }
  }
}

impl From<(String, HashMap<String, String>)> for IniSection {
  fn from(value: (String, HashMap<String, String>)) -> Self {
    IniSection { 
      name: value.0, 
      properties: value.1,
    }
  }
}

impl Into<(String, HashMap<String, String>)> for IniSection {
  fn into(self) -> (String, HashMap<String, String>) {
    (
      self.name,
      self.properties,
    )
  }
}

impl Index<&str> for IniSection {
  type Output = String;
  fn index(&self, index: &str) -> &Self::Output {
    &self.properties[index]
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct IniProperty {
  key: String,
  value: String,
}

impl IniProperty {
  pub fn key(&self) -> &str {
    &self.key
  }

  pub fn key_mut(&mut self) -> &mut String {
    &mut self.key
  }

  pub fn set_key(&mut self, key: String) -> String {
    let mut key = key;
    swap(&mut self.key, &mut key);
    key
  }

  pub fn value(&self) -> &str {
    &self.value
  }

  pub fn value_mut(&mut self) -> &mut String {
    &mut self.value
  }

  pub fn set_value(&mut self, value: String) -> String {
    let mut value = value;
    swap(&mut self.value, &mut value);
    value
  }
}

impl ToString for IniProperty {
  fn to_string(&self) -> String {
    format!("{}:{}", self.key, self.value)
  }
}

impl From<(String, String)> for IniProperty {
  fn from(value: (String, String)) -> Self {
    Self {
      key: value.0,
      value: value.1,
    }
  }
}

impl From<(&str, &str)> for IniProperty {
  fn from(value: (&str, &str)) -> Self {
    Self {
      key: value.0.to_owned(),
      value: value.1.to_owned(),
    }
  }
}

impl Into<(String, String)> for IniProperty {
  fn into(self) -> (String, String) {
    (self.key, self.value)
  }
}
