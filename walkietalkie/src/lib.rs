pub mod soldier;
pub mod commander;

pub mod walkietalkie {
  use std::{error::Error, io::{Read, Write}, net::{Shutdown, TcpListener, TcpStream}, process, sync::mpsc::{Receiver, Sender, channel}};
  use log::{error, info};
  use crate::commander::commander::CommanderConfig;
  //use crate::soldier::soldier::SoldierConfig;

  use serde::{Serialize, Deserialize};
  /*#[derive(Deserialize, Clone, Debug)]
  pub struct Config {
    pub name: String,
    pub addr: String,
    pub commands: Vec<Command>,
  }*/
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct Report {
    pub status: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>
  }
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct Command {
    pub name: String,
    pub args: Vec<String>,
  }

  trait Communication {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<dyn Error>> where Self: Sized;
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> where Self: Sized;
    fn from_bytes_to_vec(bytes: Vec<u8>) -> Result<Vec<Self>, Box<dyn Error>> where Self: Sized;
    fn from_vec_to_bytes(communication: Vec<Self>) -> Result<Vec<u8>, Box<dyn Error>> where Self: Sized;
  }
  impl Report {
    /*fn new(status: i32, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
      Report {
        status,
        stdout,
        stderr
      }
    }*/
  }
  impl Communication for Report {
    fn from_bytes(bytes: Vec<u8>) -> Result<Report, Box<dyn Error>> {
      match bincode::deserialize(&bytes) {
        Ok(report) => { 
          let report: Report = report;
          Ok(report)
        },
        Err(error) => {
          error!("Could not convert from bytes to Report");
          Err(error)
        }
      }
    }
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
      match bincode::serialize(self) {
        Ok(bytes) => {
          Ok(bytes)
        },
        Err(error) => {
          error!("Could not convert from Report to bytes");
          Err(error)
        }
      }
    }
    fn from_bytes_to_vec(bytes: Vec<u8>) -> Result<Vec<Report>, Box<dyn Error>> {
      //println!("from_bytes_to_vec Report{:?}", bytes);
      match bincode::deserialize(&bytes) {
        Ok(reports) => { 
          let reports: Vec<Report> = reports;
          Ok(reports)
        },
        Err(error) => {
          error!("Could not convert from bytes to Vec<Report>");
          Err(error)
        }
      }
    }
    fn from_vec_to_bytes(reports: Vec<Report>) -> Result<Vec<u8>, Box<dyn Error>> {
      //println!("from_vec_to_bytes Report{:?}", reports);
      match bincode::serialize(&reports) {
        Ok(bytes) => { 
          let bytes: Vec<u8> = bytes;
          Ok(bytes)
        },
        Err(error) => {
          error!("Could not convert from Vec<Report> to bytes");
          Err(error)
        }
      }
    }
  }
  impl Command {
    /*fn new(name: String, args: Vec<String>) -> Self {
      Command {
        name,
        args
      }
    }*/
  }

  impl Communication for Command {
    fn from_bytes(bytes: Vec<u8>) -> Result<Command, Box<dyn Error>> {
      match bincode::deserialize(&bytes) {
        Ok(command) => { 
          let command: Command = command;
          Ok(command)
        },
        Err(error) => {
          error!("Could not convering from bytes to Command");
          Err(error)
        }
      }
    }
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
      match bincode::serialize(&self) {
        Ok(bytes) => {
          Ok(bytes)
        },
        Err(error) => {
          error!("Could not convering from Command to bytes");
          Err(error)
        }
      }
    }
    fn from_bytes_to_vec(bytes: Vec<u8>) -> Result<Vec<Command>, Box<dyn Error>> {
      //println!("from_bytes_to_vec Command{:?}", bytes);
      match bincode::deserialize(&bytes) {
        Ok(commands) => { 
          let commands: Vec<Command> = commands;
          Ok(commands)
        },
        Err(error) => {
          error!("Could not convert from bytes to Vec<Command>");
          Err(error)
        }
      }
    }

    fn from_vec_to_bytes(commands: Vec<Command>) -> Result<Vec<u8>, Box<dyn Error>> {
      //println!("from_vec_to_bytes Command{:?}", commands);
      match bincode::serialize(&commands) {
        Ok(bytes) => { 
          let bytes: Vec<u8> = bytes;
          Ok(bytes)
        },
        Err(error) => {
          error!("Could not convert from Vec<Command> to bytes");
          Err(error)
        }
      }
    }
  }

  pub struct Soldier;
  impl Soldier {
    pub fn connect(addr: String) -> TcpStream {
      info!("Trying to connect to the server {}", &addr);
      let tcp_stream = if let Ok(tcp_stream) = TcpStream::connect(addr) {
        tcp_stream
      } else {
        panic!("Could not connect with the commander server");
      };
  
      info!("Connected to the server");
      tcp_stream
    }

    pub fn send_report(mut tcp_stream: &TcpStream, reports: Vec<Report>) -> Result<usize, String> {
      info!("Trying to serialize the reports");
      match Report::from_vec_to_bytes(reports) {
        Ok(outputs_serialized) => {
          info!("Reports serialized");
          
          match tcp_stream.write(&outputs_serialized) {
            Ok(buf_wrote) => {
              info!("Bytes wrote to the stream");
              return Ok(buf_wrote);
            }
            Err(_) => {
              return Err("Could not write in the stream".to_string());
            }
          }
        },
        Err(e) => {
          return Err(format!("{}", e));
        } 
      }
    }

    pub fn receive_commands(mut tcp_stream: &TcpStream) -> Result<Vec<Command>, String> {
      let mut buf = [0 as u8; 1024];
      info!("Trying to read from the stream");
      match tcp_stream.read(&mut buf) {
        Ok(_buf_read) => {
            //info!("{}", buf_read);
            info!("Trying to deserialize the commands");
            match Command::from_bytes_to_vec(buf.to_vec()) {
              Ok(list_commands) => {
                info!("Deserialized!");
                let commands: Vec<Command> = list_commands;

                info!("Returning commands deserialized");
                return Ok(commands);
              }
              Err(_) => {
                return Err("Could not deserilize the commands from the stream".to_string());
              }
            }
          }
        Err(_) => {
          return Err("Could not read from stream".to_string());
        }
      }
    }

    pub fn desconnect(tcp_stream: &TcpStream) {
      info!("Desconnecting from the stream");
      tcp_stream.shutdown(Shutdown::Both).unwrap()
    }
  }
  #[derive(Clone, Debug)]
  pub struct Commander {
    config: CommanderConfig,
  }
  impl Commander {
    pub fn new(config: CommanderConfig) -> Self {
      Commander {
        config,
      }
    }
    pub fn listen(&self) -> Result<TcpListener, Box<dyn Error>> {
      match TcpListener::bind(self.config.addr.clone()) {
        Ok(listener) => {
          Ok(listener)
        },
        Err(error) => {
          Err(Box::new(error))
        }
      }
    }
    pub fn channel() -> (Sender<Vec<Report>>, Receiver<Vec<Report>>) {
      channel()
    }
    pub fn send_orders(tcp_stream: &mut TcpStream, orders: Vec<Command>) {
      let buf_order = if let Ok(buf) = Command::from_vec_to_bytes(orders) {
        buf
      } else {
        error!("Could not convert from Commands to bytes");
        Commander::desconnect(&tcp_stream);
        process::exit(1);
      };

      tcp_stream.write(&buf_order)
      .expect("Could not write on the stream");
    }
    
    pub fn receive_reports(mut tcp_stream: &TcpStream) -> Vec<Report> {
      let mut buf_reports = [0 as u8; 1024];
      tcp_stream.read(&mut buf_reports)
      .expect("Cound not read the orders from stream");

      let reports = if let Ok(reports) = Report::from_bytes_to_vec(buf_reports.to_vec()) {
        reports
      } else {
        error!("Could not convert from bytes to Commands");
        Commander::desconnect(tcp_stream);
        process::exit(1);
      };

      reports
    }
    pub fn desconnect(tcp_stream: &TcpStream) {
      tcp_stream.shutdown(Shutdown::Both).unwrap();
    }
  }
}