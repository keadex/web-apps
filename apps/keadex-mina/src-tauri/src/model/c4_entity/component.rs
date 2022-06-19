/*!
Model representing the Component diagram.
*/

use serde::{Serialize, Deserialize};
use crate::model::c4_entity::base_entity::BaseEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
  pub base_data: BaseEntity,
  pub technology: String
}