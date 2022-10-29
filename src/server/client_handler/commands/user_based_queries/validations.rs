use crate::server::{
    client_handler::{responses::errors::ErrorReply, ClientHandler},
    client_trait::ClientTrait,
};

impl<T: ClientTrait> ClientHandler<T> {
    pub fn assert_whois_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        None
    }
}
