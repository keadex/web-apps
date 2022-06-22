/*!
Model representing the Relationship between two C4 entities.
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
  pub uuid_from: String,
  pub uuid_to: String,
  pub label: String,
  pub technology: String
}