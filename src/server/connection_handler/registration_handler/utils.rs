use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerUtils;

use crate::server::data_structures::*;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for RegistrationHandler<C> {}

impl<C: Connection> RegistrationHandler<C> {
    pub fn build_client(&mut self) -> Option<LocalClient<C>> {
        let info = ClientInfo::new(
            self.attributes.get("nickname")?,
            self.attributes.get("username")?,
            self.attributes.get("hostname")?,
            self.attributes.get("servername")?,
            self.attributes.get("realname")?,
            1,
        );

        let client = LocalClient::new(
            self.stream_for_database.take()?,
            &self.attributes.get("password").map(|x| x.to_string()),
            info,
        );

        Some(client)
    }
}
