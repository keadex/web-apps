/*!
Model representing specifications of a C4 diagram. 
*/

use serde::{Serialize, Deserialize};
use crate::model::diagram::graphic::diagram_entity::DiagramEntity;
use crate::model::diagram::graphic::shape::Shape;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagramSpec {
  pub uuid: String,
  pub name: String,
  pub description: String,
  pub entities: Vec<DiagramEntity>,
  pub shapes: Vec<Shape>
}