/*!
Model representing an entity drawn in a diagram. 
*/

use serde::{Serialize, Deserialize};
use strum_macros::Display;
use crate::model::diagram::graphic::position::Position;
use crate::model::diagram::graphic::size::Size;

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
  pub entity_type: EntityType,
  pub position: Position,
  pub rotation: f32,
  pub size: Size,
  pub from: String,
  pub to: String,
  pub label: String,
  pub linked_diagram_uuid: String
}