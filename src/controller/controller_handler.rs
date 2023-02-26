use crate::{
    message::Message,
    server::consts::commands::{INVITE_COMMAND, KICK_COMMAND, PRIVMSG_COMMAND},
};

use super::{
    controller_message::ControllerMessage::{
        self, ErrorWhenAddingChannel, OpenAddChannelView, OpenMainView, OpenWarningView,
        ReceiveInvite, ReceiveJoin, ReceiveJoinNotification, ReceiveKick, ReceiveListLine,
        ReceiveNamesEnd, ReceiveNamesLine, ReceivePrivMessage, RegularMessage,
    },
    ERR_IS_ALREADY_ON_CHANNEL_WARNING_TEXT, ERR_NICK_COLLISION_WARNING_TEXT,
};

pub const ERR_IS_ALREADY_ON_CHANNEL: &str = "443";
/// 436 -> Nick collision
pub const ERR_NICK_COLLISION: &str = "436";
pub const JOIN_COMMAND: &str = "331";
pub const JOIN_NOTIFICATION_COMMAND: &str = "JOIN";
/// 323 -> End of list
pub const LIST_END_COMMAND: &str = "323";
/// 322 -> List command
pub const LIST_LINE_COMMAND: &str = "322";
/// 001 -> Succesful registration
pub const LOGIN_OK: &str = "001";
/// 366 -> End of names
pub const NAMES_END_COMMAND: &str = "366";
/// 353 -> Names command
pub const NAMES_LINE_COMMAND: &str = "353";

/// Parses a received message and puts it in Controller Message format.
///
/// Receives a Message and returns a Controller Message.
pub fn to_controller_message(message: Message) -> ControllerMessage {
    match &message.get_command()[..] {
        ERR_NICK_COLLISION => OpenWarningView {
            message: ERR_NICK_COLLISION_WARNING_TEXT.to_string(),
        },
        ERR_IS_ALREADY_ON_CHANNEL => ErrorWhenAddingChannel {
            message: ERR_IS_ALREADY_ON_CHANNEL_WARNING_TEXT.to_string(),
        },
        INVITE_COMMAND => ReceiveInvite { message },
        JOIN_COMMAND => ReceiveJoin { message },
        KICK_COMMAND => ReceiveKick { message },
        LIST_END_COMMAND => OpenAddChannelView {},
        LIST_LINE_COMMAND => ReceiveListLine { message },
        LOGIN_OK => OpenMainView { message },
        NAMES_END_COMMAND => ReceiveNamesEnd {},
        NAMES_LINE_COMMAND => ReceiveNamesLine { message },
        PRIVMSG_COMMAND => ReceivePrivMessage { message },
        JOIN_NOTIFICATION_COMMAND => ReceiveJoinNotification { message },
        _ => RegularMessage {
            message: message.to_string(),
        },
    }
}
