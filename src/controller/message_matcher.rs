use std::collections::HashMap;

use gtk4 as gtk;

use crate::{
    client::Client,
    server::consts::commands::{
        JOIN_COMMAND,
        KICK_COMMAND,
        NICK_COMMAND,
        PART_COMMAND,
        PASS_COMMAND,
        QUIT_COMMAND,
        USER_COMMAND,
        INVITE_COMMAND,
        LIST_COMMAND,
        PRIVMSG_COMMAND,
    },
    views::add_views::{
        notifications_view::NotificationsView,
        user_info_view::UserInfoView,
        warning_view::WarningView,
        channel_members_view::ChannelMembersView,
    },
};
use gtk::{ glib::{ GString, Sender }, prelude::*, Application, ApplicationWindow };

use crate::views::main_view::MainView;

use super::{
    controller_message::ControllerMessage,
    JOIN_ERROR_TEXT,
    KICK_ERROR_TEXT,
    PART_ERROR_TEXT,
    QUIT_ERROR_TEXT,
    ADD_VIEW_ADD_CLIENT_ERROR_TEXT,
    ADD_VIEW_INVITE_ERROR_TEXT,
    NICK_ERROR_TEXT,
    USER_ERROR_TEXT,
    PASS_ERROR_TEXT,
    controller_handler::to_controller_message,
    FAILED_TO_READ_MESSAGE_ERROR_TEXT,
    INVITE_ERROR_TEXT,
    LIST_ERROR_TEXT,
    PRIVMSG_ERROR_TEXT,
};

pub fn add_new_client(
    add_client_window: &mut ApplicationWindow,
    main_view: &mut MainView,
    new_client: GString
) {
    add_client_window.close();
    main_view.add_client(new_client.to_string());
}

pub fn change_conversation(
    main_view: &mut MainView,
    last_conversation: String,
    current_conversation: String
) {
    main_view.change_conversation(last_conversation, current_conversation);
}

pub fn join_channel(
    add_channel_window: &mut ApplicationWindow,
    main_view: &mut MainView,
    channel: String,
    client: &mut Client
) {
    add_channel_window.close();
    let message = format!("{JOIN_COMMAND} {channel}");
    client.send_raw(&message).expect(JOIN_ERROR_TEXT);
    main_view.add_channel(channel);
}

pub fn kick_member(channel: String, member: String, client: &mut Client) {
    let message = format!("{KICK_COMMAND} {channel} {member}");
    client.send_raw(&message).expect(KICK_ERROR_TEXT);
}

pub fn open_notifications_view(app: Application, main_view: &mut MainView) {
    NotificationsView::new().get_view(app.clone(), main_view.get_notifications()).show();
}

pub fn open_user_info_view(
    app: Application,
    realname: String,
    servername: String,
    nickname: String,
    username: String
) {
    UserInfoView::new().get_view(app, realname, servername, nickname, username).show();
}

pub fn open_warning_view(app: Application, message: String) {
    WarningView::new().get_view(app.clone(), message).show();
}

pub fn quit(client: &mut Client) {
    let quit_message = QUIT_COMMAND.to_string();
    client.send_raw(&quit_message).expect(QUIT_ERROR_TEXT);
}

pub fn quit_channel(client: &mut Client, current_conv: String) {
    let part_message = format!("{PART_COMMAND} {current_conv}");
    client.send_raw(&part_message).expect(PART_ERROR_TEXT);
}

pub fn receive_invite(channel: String, main_view: &mut MainView, nickname: String) {
    let message = format!("{nickname} has invited you to join {channel}");
    main_view.add_notification(message);
}

pub fn receive_kick(
    channel: String,
    current_conv: String,
    current_nickname: String,
    kicked: String,
    main_view: &mut MainView
) {
    if kicked == current_nickname {
        main_view.remove_conversation(channel.clone());
        if channel == current_conv {
            main_view.welcome_view();
        }
    }
}

pub fn receive_names_channels(
    app: Application,
    trying_to_add_client: bool,
    trying_to_invite_client: bool,
    sender: Sender<ControllerMessage>,
    channels_and_clients: HashMap<String, Vec<String>>,
    current_conv: String,
    current_nickname: String
) {
    if trying_to_add_client {
        sender
            .send(ControllerMessage::OpenAddClientView {
                channels_and_clients,
            })
            .expect(ADD_VIEW_ADD_CLIENT_ERROR_TEXT);
    } else if trying_to_invite_client {
        sender
            .send(ControllerMessage::OpenInviteClientView {
                channels_and_clients,
            })
            .expect(ADD_VIEW_INVITE_ERROR_TEXT);
    } else {
        ChannelMembersView::new()
            .get_view(
                app.clone(),
                channels_and_clients[&current_conv].clone(),
                current_nickname.clone(),
                current_conv.clone(),
                sender.clone()
            )
            .show();
    }
}

pub fn receive_priv_message(
    channel: Option<String>,
    main_view: &mut MainView,
    message: String,
    sender_nickname: String,
    current_conv: String
) {
    if let Some(..) = channel {
        main_view.receive_priv_channel_message(
            message,
            sender_nickname,
            channel.unwrap(),
            current_conv.clone()
        );
    } else {
        main_view.receive_priv_client_message(message, sender_nickname, current_conv.clone());
    }
}

pub fn register(
    pass: GString,
    nickname: GString,
    username: GString,
    realname: GString,
    client: &mut Client,
    sender: Sender<ControllerMessage>
) {
    let pass_command = format!("{PASS_COMMAND} {pass}");
    let nick_command = format!("{NICK_COMMAND} {nickname}");
    let user_command = format!("{USER_COMMAND} {username} {username} {username} :{realname}");
    client.send_raw(&pass_command).expect(PASS_ERROR_TEXT);
    client.send_raw(&nick_command).expect(NICK_ERROR_TEXT);
    client.send_raw(&user_command).expect(USER_ERROR_TEXT);

    let sender_clone = sender.clone();
    client.start_async_read(move |message| {
        match message {
            Ok(message) => {
                let controller_message = to_controller_message(message);
                sender_clone.send(controller_message).unwrap();
            }
            Err(error) => eprintln!("{FAILED_TO_READ_MESSAGE_ERROR_TEXT}: {error}"),
        }
    });
}

pub fn regular_message(message: String) {
    println!("{message}");
}

pub fn remove_conversation(current_conv: String, main_view: &mut MainView) {
    main_view.remove_conversation(current_conv.clone());
    main_view.welcome_view();
}

pub fn send_invite_message(
    channel: GString,
    client: &mut Client,
    current_conv: String,
    invite_window: &mut ApplicationWindow
) {
    invite_window.close();
    let invite = format!("{INVITE_COMMAND} {current_conv} {channel}");
    client.send_raw(&invite).expect(INVITE_ERROR_TEXT);
}

pub fn send_list_message(client: &mut Client) {
    client.send_raw(LIST_COMMAND).expect(LIST_ERROR_TEXT);
}

pub fn send_priv_message(
    client: &mut Client,
    current_conv: String,
    main_view: &mut MainView,
    message: GString,
) {
    let priv_message = format!("{PRIVMSG_COMMAND} {current_conv} :{message}");
    client.send_raw(&priv_message).expect(PRIVMSG_ERROR_TEXT);
    main_view.send_message(message.to_string(), current_conv.clone());
}