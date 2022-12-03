/// This module contains error responses the command's may generate.
pub mod errors;
/// This module contains notifications for specific commands.
pub mod notifications;
/// This module contains replies valid command's may generate.
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
