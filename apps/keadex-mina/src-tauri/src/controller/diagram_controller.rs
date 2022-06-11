/*!
Diagram Controller
Exposes to the front-end the commands to manage diagrams.
*/

use crate::repository::diagram_repository::DiagramRepository;

pub struct DiagramController {
  pub diagram_repository: DiagramRepository, //TODO resolve by using the root resolver
  pub counter: u32
}

impl Default for DiagramController {
  fn default() -> Self {
    DiagramController {
      diagram_repository: DiagramRepository{}, //TODO resolve by using the root resolver
      counter: 0
    }
  }
}

impl DiagramController {
  pub fn give_me_name(&mut self, invoke_message: &str) -> String {
    log::debug!("Hello log");
    self.counter = self.counter + 1;
    "hello jack ".to_owned() + &self.counter.to_string() + " " + invoke_message
  }
}