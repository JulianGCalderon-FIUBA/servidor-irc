use std::env;
use std::io::{stdin, BufRead, BufReader};

use internet_relay_chat::server::Server;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let address = args.pop().unwrap();
    let servername = args.pop().unwrap();

    let mut server = Server::start(&servername);

    if let Err(error) = server.listen_to(address) {
        return eprintln!("Error: Binding to address: {:?}", error);
    }

    let reader = BufReader::new(stdin());
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return eprint!("Error reading from stdin: {}", error),
        };

        let split: Vec<&str> = line.split_whitespace().collect();

        match split[0] {
            "QUIT" => {
                server.quit();
                return;
            }
            "CONNECT" => {
                server.connect_to(split[1]);
            }
            _ => (),
        }
    }
}
