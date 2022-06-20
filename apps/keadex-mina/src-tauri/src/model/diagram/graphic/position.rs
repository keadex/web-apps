/*!
Model representing the position of a diagram's entity. 
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
  pub x: f32,
  pub y: f32,
  pub z: f32
}