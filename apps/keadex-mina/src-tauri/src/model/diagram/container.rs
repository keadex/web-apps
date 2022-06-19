/*!
Model representing the Container diagram.
*/

use serde::{Serialize, Deserialize};
use crate::model::diagram::base_entity::BaseEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
  pub base_data: BaseEntity,
  pub technology: String,
  pub container_type: String
}