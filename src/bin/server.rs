use internet_relay_chat::server::Server;
use internet_relay_chat::ADDRESS;

fn main() {
    if let Err(error) = Server::listening_to(ADDRESS.to_string()) {
        eprintln!("Error: Binding to address: {:?}", error)
    }
}
