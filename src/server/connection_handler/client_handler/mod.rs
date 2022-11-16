use std::sync::{atomic::AtomicBool, Arc};

use crate::server::{connection::Connection, database::DatabaseHandle};

use super::connection_handler_trait::{
    ConnectionHandler, ConnectionHandlerCommands, ConnectionHandlerGetters,
    ConnectionHandlerStructure,
};

mod asserts;
mod logic;
mod utils;

const REGISTRATION_TIMELIMIT_SECS: u64 = 60;
const INVALID_CHARACTER: char = '\'';
const MAX_CHANNELS: usize = 10;
const DISTRIBUTED_CHANNEL: u8 = b'#';
const LOCAL_CHANNEL: u8 = b'&';

const ADD_MODE: u8 = b'+';
const REMOVE_MODE: u8 = b'-';

pub struct ClientHandler<C: Connection> {
    connection: C,
    database: DatabaseHandle<C>,
    nickname: String,
    servername: String,
    online: Arc<AtomicBool>,
}
impl<C: Connection> ConnectionHandler<C> for ClientHandler<C> {}

impl<C: Connection> ClientHandler<C> {
    pub fn from_connection(
        connection: C,
        servername: String,
        nickname: String,
        database: DatabaseHandle<C>,
        online: Arc<AtomicBool>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            connection,
            database,
            servername,
            online,
            nickname,
        })
    }
}

impl<C: Connection> ConnectionHandlerGetters<C> for ClientHandler<C> {
    fn online(&self) -> &Arc<AtomicBool> {
        &self.online
    }

    fn connection(&mut self) -> &mut C {
        &mut self.connection
    }

    fn database(&self) -> &DatabaseHandle<C> {
        &self.database
    }
}

impl<C: Connection> ConnectionHandlerStructure<C> for ClientHandler<C> {
    fn on_try_handle_error(&mut self) {
        eprintln!("Connection with [{}] ended unexpectedly", self.nickname)
    }
    fn on_try_handle_success(&mut self) {
        eprintln!("Closing conection with [{}]", self.nickname)
    }
}

impl<C: Connection> ConnectionHandlerCommands<C> for ClientHandler<C> {}
