pub mod commander {
  use std::{fs::File, path::Path};
  use crate::walkietalkie::Command;
  use serde::Deserialize;

  #[derive(Deserialize, Clone, Debug)]
  pub struct CommanderConfig {
    pub name: String,
    pub addr: String,
    pub commands: Vec<Command>,
  }

  pub struct Commander;
  impl Commander {
    pub fn config() -> CommanderConfig {
      let config_file = File::open(Path::new("config.ron")).expect("Could not read the config.ron file");
      match ron::de::from_reader(config_file) {
          Ok(commander_config) => {
            commander_config
          }
          Err(error) => {
            panic!("Could not deserialize the config.ron file to Config: {}", error)
          }
      }
    }
  }
}