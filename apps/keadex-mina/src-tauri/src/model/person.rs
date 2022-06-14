use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum PersonLocation {
  #[strum(serialize = "INTERNAL")]
  Internal,
  #[strum(serialize = "EXTERNAL")]
  External,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
  pub uuid: String,
  pub alias: String,
  pub name: String,
  pub description: String,
  pub location: PersonLocation,
  pub notes: String
}