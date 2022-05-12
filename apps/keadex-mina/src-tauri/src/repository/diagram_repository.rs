pub struct DiagramRepository;

impl DiagramRepository {
  pub fn give_me_name(&self, invoke_message: &str) -> String {
    "hello repo ".to_owned() + invoke_message
  }
}