/*!
Model representing the Person diagram's entity.
*/

use serde::{Serialize, Deserialize};
use crate::model::c4_entity::base_entity::BaseEntity;
use crate::model::c4_entity::base_entity::EntityLocation;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
  pub base_data: BaseEntity,
  pub location: EntityLocation
}