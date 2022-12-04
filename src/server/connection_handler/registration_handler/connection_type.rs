/// An incoming connection may be with a client, a server or undefined.
pub enum ConnectionType {
    Server,
    Client,
    Undefined,
}
