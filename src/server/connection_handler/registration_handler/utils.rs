use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerUtils;

use crate::server::data_structures::*;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for RegistrationHandler<C> {}

impl<C: Connection> RegistrationHandler<C> {
    pub fn build_client(&mut self) -> Option<LocalClient<C>> {
        ClientBuilder::<C>::new()
            .nickname(self.attributes.get("nickname")?)
            .username(self.attributes.get("username")?)
            .hostname(self.attributes.get("hostname")?)
            .servername(self.attributes.get("servername")?)
            .realname(self.attributes.get("realname")?)
            .password(self.attributes.get("password"))
            .stream(self.stream_for_database.take()?)
            .build_local_client()
    }
}
