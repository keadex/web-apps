use crate::repository::diagram_repository::DiagramRepository;

pub struct DiagramController {
  pub diagram_repository: DiagramRepository
}

impl DiagramController {
  pub fn new() -> DiagramController {
    DiagramController {
      diagram_repository: DiagramRepository{}
    }
  }
  pub fn give_me_name(&self, invoke_message: &str) -> String {
    log::debug!("Hello log");
    "hello jack ".to_owned() + invoke_message
  }
}