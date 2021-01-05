pub mod soldier {
  use core::panic;
  use std::{io::{Read, Write}, net::{Shutdown, TcpStream}};
  use log::{info, warn};

  use walkietalkie::walkietalkie::{Command, Response};

pub struct Soldier {}

impl Soldier {
  pub fn connect(addr: String) -> TcpStream {
    info!("Trying to connect to the server {}", &addr);
    let tcp_stream = if let Ok(tcp_stream) = TcpStream::connect(addr) {
      tcp_stream
    } else {
      panic!("Could not connect with the commander server")
    };

    info!("Connected to the server");
    tcp_stream
  }

  pub fn receive_commands(tcp_stream: &mut TcpStream) -> Vec<Command> {
    let mut buf = vec![];
    info!("Trying to read from the stream");
    if let Ok(_) = tcp_stream.read(&mut buf) {
      info!("Trying to deserialize the commands");
      let commands_from_commander = if let Ok(list_of_commands) = bincode::deserialize(&buf) {
        info!("Deserialized!");
        let commands: Vec<Command> = list_of_commands;
        commands
      } else {
        panic!("Could not deserilize");
      };

      info!("Returning commands deserialized");
      commands_from_commander
    } else {
      panic!("Could not read from stream");
    }
  }
  pub fn run_commands(commands: Vec<Command>) -> Vec<Response> {
    let mut list_responses_from_commands: Vec<Response> = vec![];
    for command in commands{
      info!("Trying to executing the commands");
      let handle = if let Ok(output_from_a_command) = std::process::Command::new(command.name)
      .args(command.args)
      .output() {
        info!("Command executed!");
        output_from_a_command
      } else {
        warn!("A command could not be performed");
        break;
      };
      let response_from_command = Response {
          status: handle.status.to_string(),
          stdout: handle.stdout,
          stderr: handle.stderr
      };
      info!("{:#?}", response_from_command);
      list_responses_from_commands.push(response_from_command);
    }
    list_responses_from_commands
  }
  pub fn send_output(tcp_stream: &mut TcpStream, outputs: Vec<Response>) -> usize {
    info!("Trying to serialize the outputs");
    let outputs_serialized = if let Ok(outputs_serialized) = bincode::serialize(&outputs) {
      info!("Outputs serialized");
      outputs_serialized
    } else {
      panic!("Could not serialized the outputs");
    };
    
    if let Ok(buf_wrote) = tcp_stream.write(&outputs_serialized) {
      info!("Bytes wrote to the stream");
      buf_wrote
    } else {
      panic!("Could not write in the stream");
    }
  }
  pub fn desconnect(tcp_stream: &mut TcpStream) {
    info!("Desconnecting from the stream");
    tcp_stream.shutdown(Shutdown::Both).unwrap()
  }
}
}