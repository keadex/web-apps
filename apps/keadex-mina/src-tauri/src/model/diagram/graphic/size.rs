/*!
Model representing the size of a shape/entity drawn in a diagram. 
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
  pub width: f32,
  pub height: f32
}