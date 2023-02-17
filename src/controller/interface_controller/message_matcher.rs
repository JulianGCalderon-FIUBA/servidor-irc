use std::{collections::HashMap, thread};

use gtk4 as gtk;

use crate::{
    client::{async_reader::AsyncReader, client::Client},
    controller::{
        controller_handler::to_controller_message,
        controller_message::ControllerMessage::{OpenAddClientView, OpenInviteClientView},
        CLIENT_IS_ALREADY_IN_CHANNELS_WARNING_TEXT, FAILED_TO_READ_MESSAGE_ERROR_TEXT,
        INVITE_ERROR_TEXT, JOIN_ERROR_TEXT, KICK_ERROR_TEXT, LIST_ERROR_TEXT, NICK_ERROR_TEXT,
        NO_CHANNELS_WARNING_TEXT, NO_CLIENTS_WARNING_TEXT, OPEN_ADD_CLIENT_VIEW_ERROR_TEXT,
        OPEN_INVITE_VIEW_ERROR_TEXT, PART_ERROR_TEXT, PASS_ERROR_TEXT, PRIVMSG_ERROR_TEXT,
        QUIT_ERROR_TEXT, SERVER_CONNECT_ERROR_TEXT, USER_ERROR_TEXT,
    },
    message::Message,
    server::consts::commands::{
        INVITE_COMMAND, JOIN_COMMAND, KICK_COMMAND, LIST_COMMAND, NICK_COMMAND, PART_COMMAND,
        PASS_COMMAND, PRIVMSG_COMMAND, QUIT_COMMAND, USER_COMMAND, CTCP_COMMAND
    }, ctcp::dcc_chat_sender::DccChatSender,
};
use gtk::{glib::GString, prelude::*};

use super::{
    utils::{channels_not_mine, is_not_empty},
    window_creation::{
        add_channel_window, add_client_window, channel_members_window, invite_window,
        notifications_window, safe_conversation_window, user_info_window, warning_window,
    },
    InterfaceController,
    NamesMessageIntention::*,
};

impl InterfaceController {
    pub fn add_new_client(&mut self, new_client: GString) {
        self.add_client_window.close();
        self.main_view.add_client(new_client.to_string());
    }

    pub fn change_conversation(&mut self, current_conversation: String) {
        let last_conv: String = self.current_conv.clone();
        self.current_conv = current_conversation;
        self.main_view
            .change_conversation(last_conv, self.current_conv.clone());
    }

    pub fn join_channel(&mut self, channel: String) {
        self.add_channel_window.close();
        let message: String = format!("{JOIN_COMMAND} {channel}");
        self.client.send(&message).expect(JOIN_ERROR_TEXT);
        self.main_view.add_channel(channel);
    }

    pub fn kick_member(&mut self, channel: String, member: String) {
        let message: String = format!("{KICK_COMMAND} {channel} {member}");
        self.client.send(&message).expect(KICK_ERROR_TEXT);
    }

    pub fn open_add_client_view(&mut self, channels_and_clients: HashMap<String, Vec<String>>) {
        let clients_not_mine: Vec<String> = self.clients_not_mine(channels_and_clients);
        if is_not_empty(&clients_not_mine) {
            self.add_client_window = add_client_window(&self.app, clients_not_mine, &self.sender);
            self.add_client_window.show();
        } else {
            self.send_open_warning_view(NO_CLIENTS_WARNING_TEXT);
        }
    }

    pub fn open_invite_client_view(&mut self, channels_and_clients: HashMap<String, Vec<String>>) {
        let my_channels: Vec<String> = self.main_view.get_my_channels();
        let current_conv_channels: Vec<String> = self.current_conv_channels(channels_and_clients);
        let channels_to_invite: Vec<String> = channels_not_mine(my_channels, current_conv_channels);
        if is_not_empty(&channels_to_invite) {
            self.invite_window = invite_window(&self.app, channels_to_invite, &self.sender);
            self.invite_window.show();
        } else {
            self.send_open_warning_view(CLIENT_IS_ALREADY_IN_CHANNELS_WARNING_TEXT);
        }
    }

    pub fn open_main_view(&mut self, message: Message) {
        let (nickname, realname, servername, username) = self.decode_registration(message);

        self.register_window.close();
        self.realname = realname;
        self.servername = servername;
        self.nickname = nickname.clone();
        self.username = username;
        self.main_window = self.main_view.get_view(self.app.clone(), nickname);
        self.main_window.show();
    }

    pub fn open_notifications_view(&mut self) {
        notifications_window(&self.app, self.main_view.get_notifications()).show();
    }

    pub fn send_safe_conversation_request(&mut self) {
        // self.main_window.hide();
        // safe_conversation_window(&self.app, &self.sender).show();
        println!("Sending safe conv request");
        let stream = self.client.get_stream().unwrap();
        let chat_sender = DccChatSender::send(stream, self.current_conv.clone()).unwrap();
        self.safe_conversations.insert(self.current_conv.clone(), chat_sender);
    }

    pub fn open_user_info_view(&mut self) {
        let nickname: String = self.nickname.clone();
        let realname: String = self.realname.clone();
        let servername: String = self.servername.clone();
        let username: String = self.username.clone();
        user_info_window(&self.app, nickname, realname, servername, username).show();
    }

    pub fn open_warning_view(&mut self, message: String) {
        warning_window(&self.app, message).show();
    }

    pub fn quit(&mut self) {
        self.client.send(QUIT_COMMAND).expect(QUIT_ERROR_TEXT);
    }

    pub fn quit_channel(&mut self) {
        let part_message: String = format!("{PART_COMMAND} {}", self.current_conv);
        self.client.send(&part_message).expect(PART_ERROR_TEXT);
    }

    pub fn receive_invite(&mut self, message: Message) {
        let (channel, nickname) = self.decode_invite_message(message);
        let message: String = format!("{nickname} has invited you to join {channel}");
        self.main_view.add_notification(message);
    }

    pub fn receive_kick(&mut self, message: Message) {
        let (channel, kicked) = self.decode_kick_message(message);
        if kicked == self.nickname {
            self.main_view.remove_conversation(channel.clone());
            if channel == self.current_conv {
                self.main_view.welcome_view();
            }
        }
    }

    pub fn receive_list_end(&mut self) {
        let channels: Vec<String> = self.process_list_end_message();
        let my_channels: Vec<String> = self.main_view.get_my_channels();
        let channels_not_mine: Vec<String> = channels_not_mine(channels, my_channels);

        self.add_channel_window = add_channel_window(&self.app, channels_not_mine, &self.sender);
        self.add_channel_window.show();
    }

    pub fn receive_list_line(&mut self, message: Message) {
        let channel: String = self.decode_list_line_message(message);
        self.accumulated_channels_from_list.push(channel);
    }

    pub fn receive_names_line(&mut self, message: Message) {
        let (channels, clients) = self.decode_names_line_message(message);
        self.accumulated_channels_from_names.push(channels);
        self.accumulated_clients_from_names.push(clients);
    }

    pub fn receive_names_end(&mut self) {
        let channels_and_clients: HashMap<String, Vec<String>> = self.process_names_end_message();

        match self.names_message_intention {
            AddClient => {
                self.sender
                    .send(OpenAddClientView {
                        channels_and_clients,
                    })
                    .expect(OPEN_ADD_CLIENT_VIEW_ERROR_TEXT);
            }
            InviteClient => {
                self.sender
                    .send(OpenInviteClientView {
                        channels_and_clients,
                    })
                    .expect(OPEN_INVITE_VIEW_ERROR_TEXT);
            }
            KnowMembers => {
                let channel: String = self.current_conv.clone();
                let clients: Vec<String> = channels_and_clients[&self.current_conv].clone();
                let nickname: String = self.nickname.clone();

                channel_members_window(&self.app, channel, clients, nickname, &self.sender).show();
            }
            _ => {}
        }
    }

    pub fn receive_priv_message(&mut self, message: Message) {
        let (channel, message, sender_nickname) = self.decode_priv_message(message);
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
        let pass_command: String = format!("{PASS_COMMAND} {pass}");
        let nick_command: String = format!("{NICK_COMMAND} {nickname}");
        let user_command: String =
            format!("{USER_COMMAND} {username} {username} {username} :{realname}");
        self.client.send(&pass_command).expect(PASS_ERROR_TEXT);
        self.client.send(&nick_command).expect(NICK_ERROR_TEXT);
        self.client.send(&user_command).expect(USER_ERROR_TEXT);

        let sender_clone = self.sender.clone();
        let (_async_reader, message_receiver) =
            AsyncReader::spawn(self.client.get_stream().expect("error"));
        thread::spawn(move || {
            while let Ok(message_received) = message_receiver.recv() {
                match message_received {
                    Ok(message) => {
                        let controller_message = to_controller_message(message);
                        sender_clone.send(controller_message).unwrap();
                    }
                    Err(error) => eprintln!("{FAILED_TO_READ_MESSAGE_ERROR_TEXT}: {error}"),
                }
            }
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
        let invite: String = format!("{INVITE_COMMAND} {} {channel}", self.current_conv);
        self.client.send(&invite).expect(INVITE_ERROR_TEXT);
    }

    pub fn send_list_message(&mut self) {
        self.client.send(LIST_COMMAND).expect(LIST_ERROR_TEXT);
    }

    pub fn send_names_message_to_add_client(&mut self) {
        self.send_names_message(AddClient, None);
    }

    pub fn send_names_message_to_invite_client(&mut self) {
        let my_channels: Vec<String> = self.main_view.get_my_channels();
        if is_not_empty(&my_channels) {
            self.send_names_message(InviteClient, None);
        } else {
            self.send_open_warning_view(NO_CHANNELS_WARNING_TEXT);
        }
    }

    pub fn send_names_message_to_know_members(&mut self) {
        self.send_names_message(KnowMembers, Some(self.current_conv.clone()));
    }

    pub fn send_priv_message(&mut self, message: GString) {
        let priv_message = format!("{PRIVMSG_COMMAND} {} :{message}", self.current_conv);
        self.client.send(&priv_message).expect(PRIVMSG_ERROR_TEXT);
        self.main_view
            .send_message(message.to_string(), self.current_conv.clone());
    }

    pub fn to_register(&mut self, address: String) {
        self.client = match Client::connect(address) {
            Ok(stream) => stream,
            Err(error) => panic!("{SERVER_CONNECT_ERROR_TEXT} {error:?}"),
        };
        self.ip_window.close();
        self.register_window.show();
    }
}
