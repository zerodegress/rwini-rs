use super::Ini;

mod types;

pub use types::*;

#[derive(Default)]
pub struct Rwini {
  pub core: RwiniSectionCore,
  pub extras: Ini,
}

#[derive(Default)]
pub struct RwiniSectionCore {
  pub name: Option<String>,
  pub mass: Option<i32>,
  pub radius: Option<i32>,
  pub price: Option<RwiniTypePrice>,
  pub class: Option<RwiniTypeClass>,
  pub max_hp: Option<i32>,
}
