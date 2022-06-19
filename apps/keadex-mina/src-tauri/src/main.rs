/*!
Keadex Mina
This is the main entrypoint for the app.
*/

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use keadex_mina::core::app::ROOT_RESOLVER;
use keadex_mina::model::diagram::person::Person;
use keadex_mina::model::diagram::base_entity::EntityLocation;
use keadex_mina::model::diagram::base_entity::BaseEntity;

#[tauri::command]
fn my_custom_command(invoke_message: String) {
  log::info!("{}", ROOT_RESOLVER.get().read().unwrap().diagram_controller_resolver.resolve().get().write().unwrap().give_me_name(&invoke_message[..]));
  let person = Person {
    base_data: BaseEntity {
      uuid: "ciao".to_string(),
      alias: "ciao".to_string(),
      name: "ciao".to_string(),
      description: "ciao".to_string(),
      notes: "ciao".to_string()
    },
    location: EntityLocation::Internal
  };
  log::info!("{}", serde_json::to_string(&person).unwrap());
  log::info!("{}", person.location);
  //println!("{}", DIAGRAM_CONTROLLER.get().read().unwrap().diagram_repository.give_me_name(&invoke_message[..]));
}

fn main() {
  keadex_mina::logger::init();
  let _app = keadex_mina::core::app::App::default();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
