/*!
Model representing settings of a Mina's project. 
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSettings {
  pub name: String,
  pub description: String,
  pub version: String
}