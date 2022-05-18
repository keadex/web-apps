/*!
Keadex Mina
This is the main entrypoint for the app.
*/

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]



use keadex_mina::controller::diagram_controller::DiagramController;

use state::Storage;
use std::sync::RwLock;

static DIAGRAM_CONTROLLER: Storage<RwLock<DiagramController>> = Storage::new();


#[tauri::command]
fn my_custom_command(invoke_message: String) {
  //let controller = DiagramController::new();
  println!("{}", DIAGRAM_CONTROLLER.get().read().unwrap().give_me_name(&invoke_message[..]));
  println!("{}", DIAGRAM_CONTROLLER.get().read().unwrap().diagram_repository.give_me_name(&invoke_message[..]));
  // println!("I was invoked from JS, with this message: {}", invoke_message);
}

fn init_services() {
  DIAGRAM_CONTROLLER.set(RwLock::new(DiagramController::new()));
}

fn main() {
  keadex_mina::logger::init();
  init_services();
  log::info!("a log from `MyLogger`");
  println!("hello there!");
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
