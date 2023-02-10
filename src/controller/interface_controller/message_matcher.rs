use std::collections::HashMap;

use gtk4 as gtk;

use crate::{
    client::Client,
    controller::{
        controller_handler::to_controller_message, controller_message::ControllerMessage,
        ADD_VIEW_ADD_CLIENT_ERROR_TEXT, ADD_VIEW_INVITE_ERROR_TEXT,
        CLIENT_IS_ALREADY_IN_CHANNELS_WARNING_TEXT, FAILED_TO_READ_MESSAGE_ERROR_TEXT,
        INVITE_ERROR_TEXT, JOIN_ERROR_TEXT, KICK_ERROR_TEXT, LIST_ERROR_TEXT, NAMES_ERROR_TEXT,
        NICK_ERROR_TEXT, NO_CHANNELS_WARNING_TEXT, NO_CLIENTS_WARNING_TEXT, PART_ERROR_TEXT,
        PASS_ERROR_TEXT, PRIVMSG_ERROR_TEXT, QUIT_ERROR_TEXT, SERVER_CONNECT_ERROR_TEXT,
        USER_ERROR_TEXT,
    },
    server::consts::commands::{
        INVITE_COMMAND, JOIN_COMMAND, KICK_COMMAND, LIST_COMMAND, NAMES_COMMAND, NICK_COMMAND,
        PART_COMMAND, PASS_COMMAND, PRIVMSG_COMMAND, QUIT_COMMAND, USER_COMMAND,
    },
    views::add_views::{
        add_channel_view::AddChannelView, add_client_view::AddClientView,
        channel_members_view::ChannelMembersView, invite_view::InviteView,
        notifications_view::NotificationsView, safe_conversation_view::SafeConversationView,
        user_info_view::UserInfoView, warning_view::WarningView,
    },
};
use gtk::{glib::GString, prelude::*};

use super::{
    utils::{
        channels_not_mine, client_channels, clients_not_mine, clients_to_add, is_not_empty,
        send_open_warning_view,
    },
    InterfaceController,
};

impl InterfaceController {
    pub fn add_new_client(&mut self, new_client: GString) {
        self.add_client_window.close();
        self.main_view.add_client(new_client.to_string());
    }

    pub fn change_conversation(&mut self, current_conversation: String) {
        let last_conv = self.current_conv.clone();
        self.current_conv = current_conversation;
        self.main_view
            .change_conversation(last_conv, self.current_conv.clone());
    }

    pub fn join_channel(&mut self, channel: String) {
        self.add_channel_window.close();
        let message = format!("{JOIN_COMMAND} {channel}");
        self.client.send_raw(&message).expect(JOIN_ERROR_TEXT);
        self.main_view.add_channel(channel);
    }

    pub fn kick_member(&mut self, channel: String, member: String) {
        let message = format!("{KICK_COMMAND} {channel} {member}");
        self.client.send_raw(&message).expect(KICK_ERROR_TEXT);
    }

    pub fn open_add_client_view(&mut self, channels_and_clients: HashMap<String, Vec<String>>) {
        let clients_to_add: Vec<String> = clients_not_mine(
            clients_to_add(channels_and_clients, self.current_nickname.clone()),
            self.main_view.get_my_clients(),
        );
        if is_not_empty(&clients_to_add) {
            self.add_client_window =
                AddClientView::new(self.sender.clone()).get_view(self.app.clone(), clients_to_add);
            self.add_client_window.show();
        } else {
            send_open_warning_view(self.sender.clone(), NO_CLIENTS_WARNING_TEXT);
        }
    }

    pub fn open_invite_client_view(&mut self, channels_and_clients: HashMap<String, Vec<String>>) {
        let channels_to_invite = channels_not_mine(
            self.main_view.get_my_channels(),
            client_channels(channels_and_clients, self.current_conv.clone()),
        );
        if is_not_empty(&channels_to_invite) {
            self.invite_window =
                InviteView::new(self.sender.clone()).get_view(self.app.clone(), channels_to_invite);
            self.invite_window.show();
        } else {
            send_open_warning_view(
                self.sender.clone(),
                CLIENT_IS_ALREADY_IN_CHANNELS_WARNING_TEXT,
            );
        }
    }

    pub fn open_main_view(
        &mut self,
        realname: String,
        servername: String,
        nickname: String,
        username: String,
    ) {
        self.register_window.close();
        self.current_realname = String::from(&realname[..]);
        self.current_servername = String::from(&servername[..]);
        self.current_nickname = String::from(&nickname[..]);
        self.current_username = String::from(&username[..]);
        self.main_window = self.main_view.get_view(self.app.clone(), nickname);
        self.main_window.show();
    }

    pub fn open_notifications_view(&mut self) {
        NotificationsView::new()
            .get_view(self.app.clone(), self.main_view.get_notifications())
            .show();
    }

    pub fn open_safe_conversation_view(&mut self) {
        self.main_window.hide();
        SafeConversationView::new(self.sender.clone())
            .get_view(self.app.clone())
            .show();
    }

    pub fn open_user_info_view(&mut self) {
        UserInfoView::new()
            .get_view(
                self.app.clone(),
                self.current_realname.clone(),
                self.current_servername.clone(),
                self.current_nickname.clone(),
                self.current_username.clone(),
            )
            .show();
    }

    pub fn open_warning_view(&mut self, message: String) {
        WarningView::new()
            .get_view(self.app.clone(), message)
            .show();
    }

    pub fn quit(&mut self) {
        let quit_message = QUIT_COMMAND.to_string();
        self.client.send_raw(&quit_message).expect(QUIT_ERROR_TEXT);
    }

    pub fn quit_channel(&mut self) {
        let part_message = format!("{PART_COMMAND} {}", self.current_conv);
        self.client.send_raw(&part_message).expect(PART_ERROR_TEXT);
    }

    pub fn receive_invite(&mut self, channel: String, nickname: String) {
        let message = format!("{nickname} has invited you to join {channel}");
        self.main_view.add_notification(message);
    }

    pub fn receive_kick(&mut self, channel: String, kicked: String) {
        if kicked == self.current_nickname {
            self.main_view.remove_conversation(channel.clone());
            if channel == self.current_conv {
                self.main_view.welcome_view();
            }
        }
    }

    pub fn receive_list_channels(&mut self, channels: Vec<String>) {
        self.add_channel_window = AddChannelView::new(self.sender.clone()).get_view(
            self.app.clone(),
            channels_not_mine(channels, self.main_view.get_my_channels()),
        );
        self.add_channel_window.show();
    }

    pub fn receive_names_channels(&mut self, channels_and_clients: HashMap<String, Vec<String>>) {
        if self.trying_to_add_client {
            self.sender
                .send(ControllerMessage::OpenAddClientView {
                    channels_and_clients,
                })
                .expect(ADD_VIEW_ADD_CLIENT_ERROR_TEXT);
        } else if self.trying_to_invite_client {
            self.sender
                .send(ControllerMessage::OpenInviteClientView {
                    channels_and_clients,
                })
                .expect(ADD_VIEW_INVITE_ERROR_TEXT);
        } else {
            ChannelMembersView::new()
                .get_view(
                    self.app.clone(),
                    channels_and_clients[&self.current_conv].clone(),
                    self.current_nickname.clone(),
                    self.current_conv.clone(),
                    self.sender.clone(),
                )
                .show();
        }
    }

    pub fn receive_priv_message(
        &mut self,
        channel: Option<String>,
        message: String,
        sender_nickname: String,
    ) {
        if let Some(..) = channel {
            self.main_view.receive_priv_channel_message(
                message,
                sender_nickname,
                channel.unwrap(),
                self.current_conv.clone(),
            );
        } else {
            self.main_view.receive_priv_client_message(
                message,
                sender_nickname,
                self.current_conv.clone(),
            );
        }
    }

    pub fn register(
        &mut self,
        pass: GString,
        nickname: GString,
        username: GString,
        realname: GString,
    ) {
        let pass_command = format!("{PASS_COMMAND} {pass}");
        let nick_command = format!("{NICK_COMMAND} {nickname}");
        let user_command = format!("{USER_COMMAND} {username} {username} {username} :{realname}");
        self.client.send_raw(&pass_command).expect(PASS_ERROR_TEXT);
        self.client.send_raw(&nick_command).expect(NICK_ERROR_TEXT);
        self.client.send_raw(&user_command).expect(USER_ERROR_TEXT);

        let sender_clone = self.sender.clone();
        self.client.start_async_read(move |message| match message {
            Ok(message) => {
                let controller_message = to_controller_message(message);
                sender_clone.send(controller_message).unwrap();
            }
            Err(error) => eprintln!("{FAILED_TO_READ_MESSAGE_ERROR_TEXT}: {error}"),
        });
    }

    pub fn regular_message(&mut self, message: String) {
        println!("{message}");
    }

    pub fn remove_conversation(&mut self) {
        self.main_view
            .remove_conversation(self.current_conv.clone());
        self.main_view.welcome_view();
    }

    pub fn send_invite_message(&mut self, channel: GString) {
        self.invite_window.close();
        let invite = format!("{INVITE_COMMAND} {} {channel}", self.current_conv);
        self.client.send_raw(&invite).expect(INVITE_ERROR_TEXT);
    }

    pub fn send_list_message(&mut self) {
        self.client.send_raw(LIST_COMMAND).expect(LIST_ERROR_TEXT);
    }

    pub fn send_names_message_to_add_client(&mut self) {
        self.trying_to_add_client = true;
        self.trying_to_invite_client = false;
        self.client.send_raw(NAMES_COMMAND).expect(NAMES_ERROR_TEXT);
    }

    pub fn send_names_message_to_invite_client(&mut self) {
        let my_channels = self.main_view.get_my_channels();
        if is_not_empty(&my_channels) {
            self.trying_to_add_client = false;
            self.trying_to_invite_client = true;
            self.client.send_raw(NAMES_COMMAND).expect(NAMES_ERROR_TEXT);
        } else {
            send_open_warning_view(self.sender.clone(), NO_CHANNELS_WARNING_TEXT);
        }
    }

    pub fn send_names_message_to_know_members(&mut self) {
        self.trying_to_add_client = false;
        self.trying_to_invite_client = false;
        self.client
            .send_raw(&format!("{NAMES_COMMAND} {}", self.current_conv))
            .expect(NAMES_ERROR_TEXT);
    }

    pub fn send_priv_message(&mut self, message: GString) {
        let priv_message = format!("{PRIVMSG_COMMAND} {} :{message}", self.current_conv);
        self.client
            .send_raw(&priv_message)
            .expect(PRIVMSG_ERROR_TEXT);
        self.main_view
            .send_message(message.to_string(), self.current_conv.clone());
    }

    pub fn to_register(&mut self, address: String) {
        self.client = match Client::new(address) {
            Ok(stream) => stream,
            Err(error) => panic!("{SERVER_CONNECT_ERROR_TEXT} {error:?}"),
        };
        self.ip_window.close();
        self.register_window.show();
    }
}
