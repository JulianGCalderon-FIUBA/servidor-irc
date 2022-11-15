use std::{
    io,
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::thread_pool::ThreadPool;

use super::{client_handler::ClientHandler, database::DatabaseHandle, MAX_CLIENTS};

pub struct ConnectionListener {
    servername: String,
    database: DatabaseHandle<TcpStream>,
    listener: TcpListener,
    online: Arc<AtomicBool>,
}

impl ConnectionListener {
    pub fn new(
        servername: String,
        address: String,
        database: DatabaseHandle<TcpStream>,
        online: Arc<AtomicBool>,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        listener.set_nonblocking(true)?;

        let connection_listener = Self {
            servername,
            database,
            listener,
            online,
        };

        Ok(connection_listener)
    }

    pub fn listen(self) {
        let pool = ThreadPool::create(MAX_CLIENTS);

        while self.online.load(Ordering::Relaxed) {
            let client = match self.listener.accept() {
                Ok((client, _)) => client,
                Err(_) => continue,
            };

            match self.handler(client) {
                Ok(handler) => pool.execute(|| handler.handle()),
                Err(error) => eprintln!("Could not create handler {error:?}"),
            }
        }
    }

    fn handler(&self, client: TcpStream) -> io::Result<ClientHandler<TcpStream>> {
        let database = self.database.clone();
        let online_ref = Arc::clone(&self.online);
        ClientHandler::<TcpStream>::from_stream(
            database,
            client,
            self.servername.clone(),
            online_ref,
        )
    }
}
