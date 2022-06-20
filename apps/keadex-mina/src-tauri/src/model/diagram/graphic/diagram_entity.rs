/*!
Model representing an entity drawn in a diagram. 
*/

use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum EntityType {
  #[strum(serialize = "PERSON")]
  Person,
  #[strum(serialize = "SOFTWARE_SYSTEM")]
  SoftwareSystem,
  #[strum(serialize = "CONTAINER")]
  Container,
  #[strum(serialize = "COMPONENT")]
  Component,
  #[strum(serialize = "RELATIONSHIP")]
  Relationship,
  #[strum(serialize = "BOUNDARY")]
  Boundary
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagramEntity {
  pub alias: String,
  pub entity_type: EntityType
}