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

    match client.get_all_container() {
        Err(e) => eprintln!("{}", e),
        Ok(s) => {
            s.into_iter().map(|c| c.id).for_each(|id| println!("id: {}", id));
        },
    };
}
