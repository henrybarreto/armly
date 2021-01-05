//use std::{fs::File, net::{Shutdown, TcpStream}, thread, time::Duration};
//use std::io::{Read, Write};
use std::fs::File;
use daemonize::Daemonize;
use soldier::soldier::Soldier;
//use walkietalkie::walkietalkie::{Command, Response, Soldier};

mod soldier;

fn main() {
let stdout = File::create("soldier.out").unwrap();
let stderr = File::create("soldier.err").unwrap();

let daemonize = Daemonize::new()
    .pid_file("soldier.pid") // Every method except `new` and `start`
    //.chown_pid_file(true)      // is optional, see `Daemonize` documentation
    .working_directory("./") // for default behaviour.
    .user("root")
    .group("root") // Group name
    .group(2)        // or group id.
    .umask(0o777)    // Set umask, `0o027` by default.
    .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
    .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
    .privileged_action(|| "Executed before drop privileges");

match daemonize.start() {
  Ok(_) => {
    let mut tcp_stream = Soldier::connect("127.0.0.1:14114".to_string());
    let commands_recieved = Soldier::receive_commands(&mut tcp_stream);
    let commands_output = Soldier::run_commands(commands_recieved);
    let _bytes_sent = Soldier::send_output(&mut tcp_stream, commands_output);
    Soldier::desconnect(&mut tcp_stream);
  },
  Err(e) => eprintln!("Error, {}", e),
  }
}