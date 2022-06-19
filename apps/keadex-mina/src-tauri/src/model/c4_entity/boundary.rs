/*!
Model representing a Boundary.
*/

use serde::{Serialize, Deserialize};
use strum_macros::Display;
use crate::model::c4_entity::person::Person;
use crate::model::c4_entity::software_system::SoftwareSystem;
use crate::model::c4_entity::container::Container;
use crate::model::c4_entity::component::Component;

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum BoundaryType {
  #[strum(serialize = "SOFTWARE_SYSTEM")]
  SoftwareSystem,
  #[strum(serialize = "CONTAINER")]
  Container,
  #[strum(serialize = "COMPONENT")]
  Component,
  #[strum(serialize = "CUSTOM")]
  Custom
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Boundary {
  pub uuid: String,
  pub alias: String,
  pub name: String,
  pub boundary_type: BoundaryType,
  pub label: String,
  pub persons_uuids: Vec<Person>,
  pub software_systems_uuids: Vec<SoftwareSystem>,
  pub containers_uuids: Vec<Container>,
  pub components_uuids: Vec<Component>,
  pub boundaries_uuids: Vec<Boundary>
}