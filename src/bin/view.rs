use std::env;

use internet_relay_chat::{controller::Controller, ADDRESS};

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = unpack_args(args);
    
    let mut controller = Controller::new();

    controller.start(address);
}

fn unpack_args(mut args: Vec<String>) -> String {
    args.remove(0);

    match args.pop() {
        Some(address) => address,
        None => ADDRESS.to_string(),
    }
}