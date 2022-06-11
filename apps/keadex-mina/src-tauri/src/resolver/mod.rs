/*!
Resolver
Generic resolver which allows to safely share objects across the application.
*/

use state::Storage;
use std::sync::RwLock;

pub struct Resolver<T> where T: Default + Send + Sync {
  obj: Storage<RwLock<T>>
}

impl<T> Default for Resolver<T> where T: Default + Send + Sync {
  fn default() -> Self {
    let resolver = Resolver {
      obj: Storage::new()
    };
    resolver.obj.set(RwLock::new(T::default()));
    return resolver;
  }
}

impl<T> Resolver<T> where T: Default + Send + Sync {
  pub fn resolve(&self) -> &Storage<RwLock<T>> {
    return &self.obj;
  }
}