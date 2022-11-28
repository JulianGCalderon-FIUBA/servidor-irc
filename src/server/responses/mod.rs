pub mod errors;
pub mod notifications;
pub mod replies;

pub use errors::ErrorReply;
pub use notifications::Notification;
pub use replies::CommandResponse;

fn to_trail(message: &Option<String>) -> String {
    message
        .clone()
        .map(|mut string| {
            string.insert(0, ':');
            string
        })
        .unwrap_or_default()
}
