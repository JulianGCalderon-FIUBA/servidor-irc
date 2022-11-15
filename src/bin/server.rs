use std::io::{stdin, BufRead, BufReader};

use internet_relay_chat::server::Server;
use internet_relay_chat::ADDRESS;

fn main() {
    let mut server = Server::start("lemon pie");

    if let Err(error) = server.spawn_listener(ADDRESS.to_string()) {
        return eprintln!("Error: Binding to address: {:?}", error);
    }

    let reader = BufReader::new(stdin());
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return eprint!("Error reading from stdin: {}", error),
        };

        if let "QUIT" = &line[..] {
            server.quit();
            return;
        }
    }
}
