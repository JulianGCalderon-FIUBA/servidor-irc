use internet_relay_chat::server::Server;
use internet_relay_chat::ADDRESS;

fn main() {
    let server = Server::start();

    if let Err(error) = server.listen_to(ADDRESS.to_string()) {
        eprintln!("Error: Binding to address: {:?}", error);
    }
}
