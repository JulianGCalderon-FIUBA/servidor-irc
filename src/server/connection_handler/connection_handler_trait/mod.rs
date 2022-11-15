use std::io::{self, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{atomic::AtomicBool, Arc};
use std::thread::{self, JoinHandle};

use crate::message::{CreationError, Message};
use crate::server::connection::Connection;
use crate::server::database::DatabaseHandle;

mod asserts;
mod commands;
mod getters;
mod logic;
mod structure;
mod utils;

pub use asserts::ConnectionHandlerAsserts;
pub use commands::ConnectionHandlerCommands;
pub use getters::ConnectionHandlerGetters;
pub use logic::ConnectionHandlerLogic;
pub use structure::ConnectionHandlerStructure;
pub use utils::ConnectionHandlerUtils;

pub trait ConnectionHandler<C: Connection>:
    Sized + ConnectionHandlerStructure<C> + ConnectionHandlerGetters<C> + ConnectionHandlerCommands<C>
{
    fn from_connection(
        connection: C,
        servername: String,
        database: DatabaseHandle<C>,
        online: Arc<AtomicBool>,
    ) -> io::Result<Self>;

    fn handle(mut self) {
        let (message_receiver, message_receiver_thread) =
            start_async_read_stream(self.connection().try_clone().unwrap());

        match self.try_handle(message_receiver) {
            Ok(()) => self.on_try_handle_error(),
            Err(_) => self.on_try_handle_success(),
        }

        self.connection().shutdown().unwrap();
        message_receiver_thread.join().unwrap();
    }
}

fn start_async_read_stream<C: Connection>(
    stream: C,
) -> (Receiver<Result<Message, CreationError>>, JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(|| async_read_stream(stream, sender));

    (receiver, handle)
}

fn async_read_stream<C: Connection>(stream: C, sender: Sender<Result<Message, CreationError>>) {
    let mut reader = BufReader::new(stream);

    loop {
        let message = Message::read_from_buffer(&mut reader);

        if let Err(CreationError::IoError(_)) = message {
            if sender.send(message).is_err() {
                return;
            };
            break;
        }

        if sender.send(message).is_err() {
            return;
        };
    }
}
