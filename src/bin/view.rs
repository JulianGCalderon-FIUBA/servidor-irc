use std::sync::mpsc::channel;

use internet_relay_chat::controller::Controller;

fn main() {
    
    let (sender, receiver) = channel();

    let mut controller = Controller::new();

    controller.start(sender.clone(), receiver);

}