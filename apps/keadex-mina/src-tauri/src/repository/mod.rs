use std::sync::RwLock;
use state::Storage;

pub mod diagram_repository;

pub struct RepositoryProvider {
  pub DIAGRAM_REPOSITORY: Storage<RwLock<diagram_repository::DiagramRepository>>
}

impl RepositoryProvider {
  // pub fn give_me_name(&self, invoke_message: &str) -> String {
  //   "hello repo ".to_owned() + invoke_message
  // }
}