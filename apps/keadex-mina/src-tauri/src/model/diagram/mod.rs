pub mod graphic;
pub mod diagram_spec;

use serde::{Serialize, Deserialize};
use crate::model::diagram::diagram_spec::DiagramSpec;

/**!
  Mina's state
  Manages the internal state of Mina.
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Diagram {
  pub name: String,
  pub diagram_spec: DiagramSpec
}