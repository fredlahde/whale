extern crate whale;

use std::process;
use whale::DockerClient;

fn main() {
    let mut client = match DockerClient::new("/var/run/docker.sock") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    match client.request_to_string("foo\n") {
        Err(e) => eprintln!("{}", e),
        Ok(s) => println!("{}", s),
    };
}
