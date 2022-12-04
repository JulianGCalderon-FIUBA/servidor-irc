use std::{
    io,
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::thread_pool::ThreadPool;

use super::{
    connection_handler::{ConnectionHandler, RegistrationHandler},
    database::DatabaseHandle,
    MAX_CLIENTS,
};

/// In charge of creating handlers for each new client or server
///  connecting to the specified address.
pub struct ConnectionListener {
    database: DatabaseHandle<TcpStream>,
    listener: TcpListener,
    online: Arc<AtomicBool>,
}

impl ConnectionListener {
    /// Creates new [`ConnectionListener`] from an address to listen from.
    pub fn new(
        address: String,
        database: DatabaseHandle<TcpStream>,
        online: Arc<AtomicBool>,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        listener.set_nonblocking(true)?;

        let connection_listener = Self {
            database,
            listener,
            online,
        };

        Ok(connection_listener)
    }

    /// Starts listening from configured address.
    pub fn listen(self) {
        let pool = ThreadPool::create(MAX_CLIENTS);

        while self.online.load(Ordering::Relaxed) {
            let client = match self.listener.accept() {
                Ok((client, _)) => client,
                Err(error) => match error.kind() {
                    io::ErrorKind::WouldBlock => continue,
                    _ => return eprintln!("Could not listen from address {error:?}"),
                },
            };

            match self.handler(client) {
                Ok(handler) => pool.execute(|| handler.handle()),
                Err(error) => eprintln!("Could not create handler {error:?}"),
            }
        }
    }

    /// Creates RegistrationHandler for given stream.
    fn handler(&self, client: TcpStream) -> io::Result<RegistrationHandler<TcpStream>> {
        let database = self.database.clone();
        let online = Arc::clone(&self.online);
        RegistrationHandler::<TcpStream>::from_connection(client, database, online)
    }
}
