use std::{error::Error, net::{Shutdown}, thread};
use walkietalkie::walkietalkie::{Commander, CommanderConfig, Communications};

fn main() -> Result<(), Box<dyn Error>> {
  let config: CommanderConfig = Commander::config();

  let commands = config.commands; //commands_defined();
  let commander = Commander::new(commands);

  match Communications::listen(config.addr.clone()) {
    Ok(communications) => {
    println!("Listening on {}", config.addr.clone());
      for communication in communications.incoming() {
        if let Ok(communication_established) = communication {
          println!("Connection stabablished with: {}", communication_established.peer_addr().unwrap());
          let commander_actived = commander.clone();
          let (transmission_channel, recieve_channel) = Communications::wire();
          thread::spawn(move || {
            println!("A new thread was spawned!");
            let transmission_channel_clone = transmission_channel.clone();
            let responses = match commander_actived.order(&communication_established) {
                Ok(responses) => {responses}
                Err(_) => {panic!("Error while ordening commands to Soldier") }
          };
            match transmission_channel_clone.send(responses) {
                Ok(_) => {println!("Sending Responses from channel")}
                Err(_) => {eprintln!("Could not send Responses from channel")}
            }
          communication_established.shutdown(Shutdown::Both).unwrap();
          }).join().unwrap();
          println!("Recieve Responses form channel");
          let responses = match recieve_channel.recv() {
              Ok(responses) => {responses}
              Err(_) => {panic!("Could not recieve the Responses from channel")}
          };
          for response in responses {
            println!("Status: {}", response.status);
            println!("Stdout: {}", String::from_utf8(response.stdout).unwrap());
            println!("Stderr: {}", String::from_utf8(response.stderr).unwrap());
          }
        } else {
          eprintln!("A connection error!");
        }
      }
    },
    Err(_) => panic!("Could not listen"),
};

  Ok(())
}