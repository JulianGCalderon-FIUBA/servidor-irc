#[cfg(test)]
/// Utils for testing different functionalities.
mod testing_utils;

/// Contains structure for client handler. It's main purpose is to handle the connection established between server and clients.
mod client_handler;

/// Definition of the trait used in the project's structures.
mod client_trait;

/// Contains structure for database. A Database stores and updates information regarding clients, channels and related.
mod database;

use crate::thread_pool::ThreadPool;
use client_handler::ClientHandler;
use database::Database;
use std::io::{self, stdin, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use self::database::DatabaseHandle;

const MAX_CLIENTS: usize = 26;

pub const OPER_USERNAME: &str = "admin";
pub const OPER_PASSWORD: &str = "admin";

/// Represents a Server clients can connect to it contains a Database that stores relevant information.
pub struct Server {
    servername: String,
    database: DatabaseHandle<TcpStream>,
    database_thread: JoinHandle<()>,
    online: Arc<AtomicBool>,
}

impl Server {
    /// Starts new [`Server`].
    pub fn start(servername: &str) -> Self {
        let servername = servername.to_string();
        let online = Arc::new(AtomicBool::new(true));

        let (database, database_thread) = Database::start();

        Self {
            servername,
            online,
            database,
            database_thread,
        }
    }

    fn start_input_read(&self) -> JoinHandle<()> {
        let online_ref = Arc::clone(&self.online);
        thread::spawn(|| input_read(online_ref))
    }

    /// Listens for incoming clients and handles each request in a new thread.
    pub fn listen_to(self, address: String) -> io::Result<()> {
        let handle = self.start_input_read();

        let listener = TcpListener::bind(address)?;

        let pool = ThreadPool::create(MAX_CLIENTS);

        listener.set_nonblocking(true).ok();

        while self.online.load(Ordering::Relaxed) {
            let client = match listener.accept() {
                Ok((client, _)) => client,
                Err(_) => continue,
            };

            match self.handler(client) {
                Ok(handler) => pool.execute(|| handler.handle()),
                Err(error) => eprintln!("Could not create handler {error:?}"),
            }
        }

        handle.join().ok();

        drop(self.database);
        self.database_thread.join().ok();

        Ok(())
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

fn input_read(online: Arc<AtomicBool>) {
    let reader = BufReader::new(stdin());
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return eprint!("Error reading from stdin: {}", error),
        };

        if let "QUIT" = &line[..] {
            online.store(false, Ordering::Relaxed);
            return;
        }
    }
}
