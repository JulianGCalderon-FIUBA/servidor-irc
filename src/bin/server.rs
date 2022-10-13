use internet_relay_chat::server::Server;
use internet_relay_chat::ADDRESS;

fn main() {
    let server = match Server::new(ADDRESS.to_string()) {
        Ok(server) => server,
        Err(error) => return eprintln!("Error: Binding to address: {:?}", error),
    };

    if let Err(error) = server.listen() {
        eprintln!("Error: Listening from addresss: {:?}", error);
    }
}
