use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    ConnectionHandlerLogic, ConnectionHandlerUtils,
};
use crate::server::connection_handler::responses::Notification;

use super::ClientHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for ClientHandler<C> {
    fn quit_logic(&mut self, trail: Option<String>) -> std::io::Result<()> {
        let message = trail.unwrap_or_else(|| self.nickname.clone());

        let notification = Notification::Quit { message };

        self.send_response(&notification.to_string())
    }
}
