use std::env;
use std::io::{stdin, BufRead, BufReader};

use internet_relay_chat::server::Server;
use internet_relay_chat::{ADDRESS, SERVERINFO, SERVERNAME};

const QUIT_CONNECTION_COMMAND: &str = "QUIT";
const CONNECT_TO_SERVER_COMMAND: &str = "CONNECT";

fn main() {
    let args: Vec<String> = env::args().collect();
    let (address, servername) = unpack_args(args);

    let serverinfo = SERVERINFO.to_string();
    let mut server = Server::start(servername, serverinfo);

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
        if split.is_empty() {
            continue;
        }
        match split[0] {
            QUIT_CONNECTION_COMMAND => {
                server.quit();
                return;
            }
            CONNECT_TO_SERVER_COMMAND => {
                server.connect_to(split[1]);
            }
            _ => (),
        }
    }
}

fn unpack_args(mut args: Vec<String>) -> (String, String) {
    if args.len() == 3 {
        (args.remove(2), args.remove(1))
    } else {
        (ADDRESS.to_string(), SERVERNAME.to_string())
    }
}
