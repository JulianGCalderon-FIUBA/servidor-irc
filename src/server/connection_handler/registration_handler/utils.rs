use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerUtils;
use crate::server::database::{Client, ClientBuilder};

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for RegistrationHandler<C> {}

impl<C: Connection> RegistrationHandler<C> {
    pub fn build_client(&mut self) -> Option<Client<C>> {
        ClientBuilder::new()
            .nickname(self.attributes.get("nickname")?.clone())
            .password(self.attributes.remove("password"))
            .username(self.attributes.remove("username")?)
            .hostname(self.attributes.remove("hostname")?)
            .servername(self.attributes.remove("servername")?)
            .realname(self.attributes.remove("realname")?)
            .stream(self.stream_for_database.take()?)
            .build()
    }
}
