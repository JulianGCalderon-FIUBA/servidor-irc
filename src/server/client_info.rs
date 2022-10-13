use std::net::TcpStream;

/// Holds a Clients' relevant information.
pub struct ClientInfo {
    pub stream: TcpStream,
}
