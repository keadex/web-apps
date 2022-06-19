/*!
Model representing the Container diagram.
*/

use serde::{Serialize, Deserialize};
use strum_macros::Display;
use crate::model::c4_entity::base_entity::BaseEntity;

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum ContainerType {
  #[strum(serialize = "STANDARD")]
  Standard,
  #[strum(serialize = "DATABASE")]
  Database,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
  pub base_data: BaseEntity,
  pub technology: String,
  pub container_type: ContainerType
}