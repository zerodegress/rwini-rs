pub enum RwiniIdentifierNamespace {
  UnitName,
  UnitTag,
  MemoryName,
  SectionActionName,
}

pub enum RwiniTypePrice {
  Credit(i32),
  Custom(String, i32),
}

pub enum RwiniTypePriceParseError {
  CutomValueInvalid,
  CreditValueInvalid,
}

impl RwiniTypePrice {
  pub fn parse(text: &str) -> Result<Self, RwiniTypePriceParseError> {
    if let Some((k, v)) = text.split_once("=") {
      if let Ok(v) = v.trim_start().parse::<i32>() {
        Ok(Self::Custom(k.to_owned(), v))
      } else {
        Err(RwiniTypePriceParseError::CutomValueInvalid)
      }
    } else {
      if let Ok(v) = text.parse::<i32>() {
        Ok(Self::Credit(v))
      } else {
        Err(RwiniTypePriceParseError::CreditValueInvalid)
      }
    }
  }
}

pub enum RwiniTypeSpeedParseError {
  NormalizedPerTickValueInvalid,
  SecondsValueInvalid,
}

pub enum RwiniTypeSpeed {
  NormalizedPerTick(f64),
  Seconds(f64),
}

impl RwiniTypeSpeed {
  pub fn parse(text: &str) -> Result<RwiniTypeSpeed, RwiniTypeSpeedParseError> {
    if let Some(text) = text.strip_suffix("s") {
      if let Ok(v) = text.trim_end().parse::<f64>() {
        Ok(Self::Seconds(v))
      } else {
        Err(RwiniTypeSpeedParseError::SecondsValueInvalid)
      }
    } else {
      if let Ok(v) = text.parse::<f64>() {
        Ok(Self::NormalizedPerTick(v))
      } else {
        Err(RwiniTypeSpeedParseError::NormalizedPerTickValueInvalid)
      }
    }
  }
}

pub enum RwiniTypeClassParseError {
  UnknownClass,
}

pub enum RwiniTypeClass {
  CustomUnitMetaData,
}

impl RwiniTypeClass {
  pub fn parse(text: &str) -> Result<RwiniTypeClass, RwiniTypeClassParseError> {
    match text {
      "CustomUnitMetaData" => Ok(Self::CustomUnitMetaData),
      _ => Err(RwiniTypeClassParseError::UnknownClass),
    }
  }
}