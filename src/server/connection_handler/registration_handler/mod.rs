use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

use crate::server::{connection::Connection, database::DatabaseHandle};

use super::connection_handler_trait::ConnectionHandler;

mod asserts;
mod commands;
mod getters;
mod logic;
mod structure;
mod utils;

pub struct RegistrationHandler<C: Connection> {
    stream: C,
    stream_for_database: Option<C>,
    database: DatabaseHandle<C>,
    servername: String,
    online: Arc<AtomicBool>,
    attributes: HashMap<&'static str, String>,
}

impl<C: Connection> ConnectionHandler<C> for RegistrationHandler<C> {
    fn from_connection(
        stream: C,
        servername: String,
        database: DatabaseHandle<C>,
        online: Arc<AtomicBool>,
    ) -> std::io::Result<Self> {
        let stream_for_database = Some(stream.try_clone()?);

        Ok(Self {
            stream,
            stream_for_database,
            database,
            servername,
            online,
            attributes: HashMap::new(),
        })
    }
}
