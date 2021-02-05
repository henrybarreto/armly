//use std::{fs::File, net::{Shutdown, TcpStream}, thread, time::Duration};
//use std::io::{Read, Write};
//use std::{fs::File, io::Read, thread, time};
//use daemonize::Daemonize;
//use soldier::soldier::Soldier;
//use walkietalkie::walkietalkie::{Command, Response, Soldier};

//use log::info;
use simple_logger::SimpleLogger;
use walkietalkie::{self, soldier};

fn main() {
SimpleLogger::new().init().unwrap();
/*let stdout = File::create("soldier.out").unwrap();
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
    loop {*/
      let config = soldier::Soldier::config();
      let tcp_stream = walkietalkie::walkietalkie::Soldier::connect(config.addr.clone());
      let commands_recieved = walkietalkie::walkietalkie::Soldier::receive_commands(&tcp_stream).unwrap();
      let commands_output = soldier::Soldier::run_commands(commands_recieved);
      let _bytes_sent = walkietalkie::walkietalkie::Soldier::send_report(&tcp_stream, commands_output).unwrap();
      walkietalkie::walkietalkie::Soldier::desconnect(&tcp_stream);
      /*
      thread::sleep(time::Duration::from_secs(config.interval));
    }
  },
  Err(e) => eprintln!("Error, {}", e),
  }*/
}