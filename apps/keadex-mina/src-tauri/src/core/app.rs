/*!
App
This represents the shell of the application.
*/

use crate::core::root_resolver::RootResolver;
use state::Storage;
use std::sync::RwLock;

pub static ROOT_RESOLVER: Storage<RwLock<RootResolver>> = Storage::new();

pub struct App;

impl Default for App {
  fn default() -> Self {
      App::new()
  }
}

impl App {
  pub fn new() -> Self {
    ROOT_RESOLVER.set(RwLock::new(RootResolver::default()));
    App {}
  }
}