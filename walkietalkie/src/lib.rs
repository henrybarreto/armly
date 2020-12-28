pub mod walkietalkie {
  use std::{error::Error, fs::File, io::{Read, Write}, path::Path, sync::mpsc::{Receiver, Sender, channel}};
  use std::net::{TcpListener, TcpStream};
  use serde::{Serialize, Deserialize};
  use bincode::{self, ErrorKind};

  type Arg = String;
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct Command {
    pub name: String,
    pub args: Vec<Arg>,
  }
  impl Command {
    pub fn new(name: String, args: Vec<Arg>) -> Self {
      Command {
        name,
        args
      }
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Command, Box<ErrorKind>> {
      match bincode::deserialize(&bytes) {
        Ok(data) => { 
          let command: Command = data;
          Ok(command)
        },
        Err(error) => {
          eprint!("Error in converting Command from_bytes");
          Err(error)
        }
      }
    }
    pub fn to_bytes(self) -> Result<Vec<u8>, Box<ErrorKind>> {
      match bincode::serialize(&self) {
        Ok(data) => {
          Ok(data)
        },
        Err(error) => {
          eprint!("Error in converting Command to_bytes");
          Err(error)
        }
      }

    }
  }
 
  #[derive(Debug, Serialize, Deserialize)]
  pub struct CommanderConfig {
    pub addr: String,
    pub commands: Vec<Command>,
  }
  pub struct Communications {
    pub listener: TcpListener,
    pub stream: TcpStream
  }
  impl Communications {
    pub fn listen(channel: String) -> Result<TcpListener, Box<dyn Error>> {
      match TcpListener::bind(channel) {
        Ok(listener) => {
          Ok(listener)
        },
        Err(error) => {
          Err(Box::new(error))
        }
      }
    }

    pub fn wire() -> (Sender<Vec<Response>>, Receiver<Vec<Response>>) {
      channel()
    }
  }
  
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct Response {
    pub status: String,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>
  }
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct Commander {
    commands: Vec<Command>,
    responses: Vec<Response> 
  }
  impl Commander {
    pub fn new(commands: Vec<Command>) -> Self {
      Commander {
        commands,
        responses: vec![]
      }
    }
    pub fn order(self, communication: &TcpStream) -> Result<Vec<Response>, Box<dyn Error>> {
      let mut communication = communication.try_clone().unwrap();
      match bincode::serialize(&self.commands) {
        Ok(buf_wrote) => {
            match communication.write(&buf_wrote) {
                Ok(_size_wrote) => {
                  let mut buf_readed: Vec<u8> = vec![];

                  let size_readed = match communication.read_to_end(&mut buf_readed) {
                    Ok(size) => {
                      size
                    }
                    Err(_) => {
                      panic!("Could not read in the Stream");
                    }
                  };
                  let responses: Vec<Response> = match bincode::deserialize(&mut buf_readed[0..size_readed]) {
                      Ok(responses) => {
                        responses
                      }
                      Err(_) => {
                        panic!("Could not convert recieved bytes to Vec<Response>");
                      }
                  };
                  Ok(responses)
                }
                Err(error) => {
                  eprint!("Could not write in the Stream");
                  Err(Box::new(error))
                }
            }
          }
        Err(error) => {
            eprint!("Could not serialize Commands");
            Err(error)
        }
      }

    }
  
    pub fn config() -> CommanderConfig {
      let config_file = if let Ok(config_file) = File::open(Path::new("config.ron")) {
        config_file
      } else {
        panic!("Could not read the config.ron file");
      };
      match ron::de::from_reader(config_file) {
          Ok(commander_config) => {
            commander_config
          }
          Err(error) => {
            panic!("Could not deserialize the config.ron file to CommanderConfig: {}", error)
          }
      }
    }
  }
  #[derive(Deserialize, Clone, Debug)]
  pub struct SoldierConfig {
    pub name: String,
    pub addr: String,
    pub interval: u64
  }

  pub struct Soldier;

  impl Soldier {
    pub fn config() -> SoldierConfig {
      let config_file = if let Ok(config_file) = File::open(Path::new("config.ron")) {
        config_file
      } else {
        panic!("Could not read the config.ron file");
      };
      match ron::de::from_reader(config_file) {
          Ok(commander_config) => {
            commander_config
          }
          Err(error) => {
            panic!("Could not deserialize the config.ron file to CommanderConfig: {}", error)
          }
      }
    }
  }
}