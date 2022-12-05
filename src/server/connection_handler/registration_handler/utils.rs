use crate::server::connection::Connection;
use crate::server::connection_handler::ConnectionHandlerUtils;

use crate::server::data_structures::*;
use crate::server::responses::{CommandResponse, Notification};

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

    pub fn send_server_notification(
        &mut self,
        servername: &str,
        hopcount: usize,
        serverinfo: &str,
    ) {
        let notification = Notification::server(servername, hopcount, serverinfo);
        self.send_message_to_all_servers(&notification)
    }

    pub fn send_new_client_notification(&mut self, info: &ClientInfo) {
        let nickname = &info.nickname;
        let nick_notification = Notification::nick(nickname, 1);
        self.send_message_to_all_servers(&nick_notification);

        let user_notification = Notification::user(info);
        self.send_message_to_all_servers(&user_notification);
    }

    pub fn send_welcome_response(&mut self, client_info: ClientInfo) -> std::io::Result<()> {
        let response = CommandResponse::welcome(client_info);
        self.stream.send(&response)
    }
}
