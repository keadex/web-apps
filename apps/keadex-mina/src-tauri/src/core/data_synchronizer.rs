/*!
Data Synchronizer.
Synchronizes data between file system (project's files) and local state (Mina's in-memory state).
*/

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::collections::HashMap;

pub struct DataSynchronizer {
  // is_running_path: Arc<Mutex<bool>>,
  is_running_path: Arc<Mutex<HashMap<String, bool>>>
}

impl Default for DataSynchronizer {
  fn default() -> Self {
    DataSynchronizer {
      // is_running: Arc::new(Mutex::new(false))
      is_running_path: Arc::new(Mutex::new(HashMap::new()))
    }
  }
}

impl DataSynchronizer {
  pub fn start_synchronization(&mut self, path: String) {
    log::debug!("Start synchronization");
    // &self.is_running_path.insert(String::from(path), true);
    let is_running_path_arc = Arc::clone(&self.is_running_path);
    let mut is_running_path = is_running_path_arc.lock().unwrap();
    (*is_running_path).insert(path.clone(), true);
    // *is_running = true;
    let is_running_path_arc_t = Arc::clone(&self.is_running_path);
    let path_arc_t = Arc::new(Mutex::new(path.clone()));
    thread::spawn(move || {
      let (tx, rx) = channel();
      let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
      watcher.watch(&path, RecursiveMode::Recursive).unwrap();
      loop {
        let is_running_path = is_running_path_arc_t.lock().unwrap();
        println!("{}", *is_running_path.get(&path).unwrap());
        if !*is_running_path.get(&path).unwrap() {
          break;
        }
        drop(is_running_path);
        let path_t = path_arc_t.lock().unwrap();
        match rx.recv() {
          Ok(event) => on_path_event((*path_t).clone(), event),
          Err(e) => println!("watch error: {:?}", e),
        }
      }
    });
  }

  pub fn stop_synchronization(&mut self, path: String) {
    log::debug!("Stop synchronization");
    let is_running_path_arc = Arc::clone(&self.is_running_path);
    let mut is_running_path = is_running_path_arc.lock().unwrap();
    (*is_running_path).insert(path.clone(), false);
  }
}

fn on_path_event(path: String, event: DebouncedEvent) {
  println!("{:?}", event)
}
