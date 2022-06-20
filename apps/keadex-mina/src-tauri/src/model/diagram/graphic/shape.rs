/*!
Model representing a shape drawn in a diagram.
*/

use serde::{Serialize, Deserialize};
use strum_macros::Display;
use crate::model::diagram::graphic::position::Position;
use crate::model::diagram::graphic::size::Size;

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum ShapeType {
  #[strum(serialize = "LINE")]
  Line
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shape {
  pub uuid: String,
  pub shape_type: ShapeType,
  pub position: Position,
  pub rotation: f32,
  pub size: Size
}