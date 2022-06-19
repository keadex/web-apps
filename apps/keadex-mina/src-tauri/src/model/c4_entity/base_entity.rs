/*!
Model representing the data shared between all the diagram's entities (Container, Person, Software System, etc.).
*/

use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum EntityLocation {
  #[strum(serialize = "INTERNAL")]
  Internal,
  #[strum(serialize = "EXTERNAL")]
  External,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseEntity {
  pub uuid: String,
  pub alias: String,
  pub name: String,
  pub description: String,
  pub notes: String
}