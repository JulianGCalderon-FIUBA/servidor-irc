#[cfg(test)]
/// Utils for testing different functionalities.
mod testing;

/// Definition of the trait used in the project's structures.
mod connection;

/// Contains structure for database. A Database stores and updates information regarding clients, channels and related.
mod database;

mod connection_handler;
/// Contains structure for connection listener, this structure listens to an address and handles all clients connecting to that address
mod listener;

use database::Database;
use std::io;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crate::message::Message;

use self::connection_handler::{ConnectionHandler, ServerHandler};
use self::database::{DatabaseHandle, ExternalServer};
use self::listener::ConnectionListener;

const MAX_CLIENTS: usize = 26;

pub const OPER_USERNAME: &str = "admin";
pub const OPER_PASSWORD: &str = "admin";

/// Represents a Server clients can connect to it contains a Database that stores relevant information.
pub struct Server {
    servername: String,
    database: Option<DatabaseHandle<TcpStream>>,
    online: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

impl Server {
    /// Starts new [`Server`].
    pub fn start(servername: &str) -> Self {
        let servername = servername.to_string();
        let online = Arc::new(AtomicBool::new(true));

        let (database, database_thread) = Database::start();

        let threads = vec![database_thread];
        let database = Some(database);

        Self {
            servername,
            online,
            database,
            threads,
        }
    }

    pub fn quit(&self) {
        self.online.store(false, Ordering::Relaxed);
    }

    /// Listens for incoming clients and handles each request in a new thread.
    pub fn listen_to(&mut self, address: String) -> io::Result<()> {
        let online = Arc::clone(&self.online);
        let servername = self.servername.clone();
        let database = self.database.clone().unwrap();

        let connection_listener = ConnectionListener::new(servername, address, database, online)?;

        let thread = thread::spawn(|| connection_listener.listen());

        self.threads.push(thread);

        Ok(())
    }

    pub fn connect_to(&mut self, address: &str) {
        let mut stream = TcpStream::connect(address).unwrap();

        let database = self.database.as_ref().unwrap();

        self.register_to(&mut stream);

        let server_info = Message::read_from(&mut stream).unwrap();

        let (_prefix, _, mut parameters, trailing) = server_info.unpack();
        let hopcount = parameters.pop().unwrap();
        let servername = parameters.pop().unwrap();
        let serverinfo = trailing.unwrap();

        let server = ExternalServer::new(
            stream.try_clone().unwrap(),
            servername.clone(),
            serverinfo,
            hopcount.parse::<usize>().unwrap(),
        );

        database.add_server(server);

        for client in database.get_all_clients() {
            let nickname = client.nickname.clone();
            let hopcount = client.hopcount;
            let msg = Message::new(&format!("NICK {nickname} {hopcount}")).unwrap();
            msg.send_to(&mut stream).unwrap();

            let nickname = client.nickname.clone();
            let servername = client.servername.clone();
            let username = client.username.clone();
            let realname = client.realname.clone();
            let hostname = client.hostname.clone();

            let msg = Message::new(&format!(
                ":{nickname} USER {username} {hostname} {servername} :{realname}"
            ))
            .unwrap();
            msg.send_to(&mut stream).unwrap();
        }

        let server_handler = ServerHandler::from_connection(
            stream,
            servername,
            self.database.clone().unwrap(),
            Arc::clone(&self.online),
        )
        .unwrap();

        thread::spawn(|| server_handler.handle());
    }

    fn register_to(&self, stream: &mut TcpStream) {
        let message =
            Message::new(&format!("SERVER {} 1 :{}", &self.servername, "motivo")).unwrap();
        message.send_to(stream).unwrap();
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.quit();

        if let Some(database) = self.database.take() {
            drop(database);
        }

        for thread in self.threads.drain(..) {
            thread.join().ok();
        }
    }
}
