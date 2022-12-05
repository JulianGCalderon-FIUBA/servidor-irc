use internet_relay_chat::server::Server;

pub fn create_server(servername: &str, serverinfo: &str) -> Server {
    let servername = servername.to_string();
    let serverinfo = serverinfo.to_string();

    Server::start(servername, serverinfo)
}

pub fn create_lemonpie_server(address: String) -> Server {
    let mut server = create_server("lemonpie", "serverinfo");

    server.listen_to(address).unwrap();

    server
}
