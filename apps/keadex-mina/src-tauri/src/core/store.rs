/*!
  Mina's state
  Manages the internal state of Mina.
*/

use crate::model::project_settings::ProjectSettings;
use crate::model::diagram::Diagram;
use crate::model::c4_entity::person::Person;
use crate::model::c4_entity::software_system::SoftwareSystem;
use crate::model::c4_entity::container::Container;
use crate::model::c4_entity::component::Component;
use crate::model::c4_entity::boundary::Boundary;

pub struct State {
  pub project_settings: ProjectSettings,
  pub diagrams: Vec<Diagram>,
  pub persons: Vec<Person>,
  pub software_systems: Vec<SoftwareSystem>,
  pub containers: Vec<Container>,
  pub components: Vec<Component>,
  pub boundaries: Vec<Boundary>
}

impl State {}