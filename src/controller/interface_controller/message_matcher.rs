use std::{collections::HashMap, io, net::SocketAddr, path::PathBuf, thread};

use gtk4 as gtk;

use crate::{
    client::Client,
    controller::{
        controller_message::ControllerMessage::{
            OpenAddClientView, OpenInviteClientView, ReceiveResult, SendDccSend,
        },
        utils::{channels_not_mine, get_sender_and_receiver, vec_is_not_empty},
        CLIENT_IS_ALREADY_IN_CHANNELS_WARNING_TEXT, INVITE_ERROR_TEXT, JOIN_ERROR_TEXT,
        KICK_ERROR_TEXT, LIST_ERROR_TEXT, NICK_ERROR_TEXT, NO_CHANNELS_WARNING_TEXT,
        NO_CLIENTS_WARNING_TEXT, OPEN_ADD_CLIENT_VIEW_ERROR_TEXT, OPEN_INVITE_VIEW_ERROR_TEXT,
        PART_ERROR_TEXT, PASS_ERROR_TEXT, PRIVMSG_ERROR_TEXT, QUIT_ERROR_TEXT,
        SERVER_CONNECT_ERROR_TEXT, USER_ERROR_TEXT,
    },
    ctcp::{
        dcc_chat::{dcc_chat_receiver::DccChatReceiver, dcc_chat_sender::DccChatSender},
        dcc_send::dcc_send_sender::DccSendSender,
        parse_ctcp,
    },
    macros::some_or_return,
    message::Message,
    server::consts::commands::{
        INVITE_COMMAND, JOIN_COMMAND, KICK_COMMAND, LIST_COMMAND, NICK_COMMAND, PART_COMMAND,
        PASS_COMMAND, PRIVMSG_COMMAND, QUIT_COMMAND, USER_COMMAND,
    },
};
use gtk::{
    prelude::FileExt,
    traits::{DialogExt, FileChooserExt, GtkWindowExt, WidgetExt},
    FileChooserDialog, ResponseType,
};

use super::{
    download::Download,
    window_creation::{
        add_channel_view, add_client_window, channel_members_window, dcc_invitation_window,
        invite_window, main_view, notifications_window, safe_conversation_view, user_info_window,
        warning_window,
    },
    InterfaceController,
    NamesMessageIntention::*,
};

impl InterfaceController {
    pub fn accept_dcc_chat(&mut self, client: String, address: SocketAddr) {
        self.dcc_invitation_window.close();
        let dcc = self.dcc_receivers.remove(&client).unwrap();
        let mut dcc_chat = dcc.accept_chat_command(address).unwrap();
        let dcc_std_receiver = dcc_chat.async_read_message();
        self.dcc_chats.insert(client.clone(), dcc_chat);

        let (dcc_sender, dcc_receiver) = get_sender_and_receiver();
        thread::spawn(move || {
            while let Ok(message_received) = dcc_std_receiver.recv() {
                dcc_sender.send(message_received).expect("error");
            }
        });

        self.receiver_attach(client.clone(), dcc_receiver, self.sender.clone());
        
        self.main_view.disable_safe_conversation_button();

        self.safe_conversation_view = safe_conversation_view(self.nickname.clone(), &self.sender);
        self.safe_conversation_view
            .get_view(&client, self.app.clone())
            .show();
    }

    pub fn add_new_client(&mut self, new_client: String) {
        self.add_client_window.close();
        self.main_view.add_client(&new_client);
        self.change_conversation(new_client.to_string());
    }

    pub fn change_conversation(&mut self, current_conversation: String) {
        let last_conv: String = self.current_conv.clone();
        self.current_conv = current_conversation;
        self.main_view
            .change_conversation(last_conv, self.current_conv.clone(), &self.dcc_chats);
    }

    pub fn decline_dcc_chat(&mut self, client: String) {
        self.dcc_invitation_window.close();
        self.dcc_receivers
            .remove(&client)
            .unwrap()
            .decline_chat_command()
            .unwrap();
    }

    pub fn download_file(&mut self, sender: String, path: PathBuf) {
        let dcc_send_receiver = some_or_return!(self.dcc_send_receivers.remove(&sender));

        let download = Download {
            client: sender.clone(),
            name: dcc_send_receiver.original_name(),
            path: path.clone(),
            failed: false,
        };
        self.downloads.push(download);

        let name = dcc_send_receiver.original_name();

        let (transferer, controller) = dcc_send_receiver.accept_send_command(path).unwrap();

        let sender_channel = self.sender.clone();
        let sender_client = sender.clone();
        thread::spawn(move || {
            let result = transferer.download_file();
            let message = ReceiveResult {
                sender: sender_client,
                name,
                result,
            };
            sender_channel.send(message).unwrap();
        });

        self.cancel_transfer_dialog("Download in progress", sender, controller);
    }

    pub fn error_when_adding_channel(&mut self, message: String) {
        self.add_channel_view.set_error_text(message);
    }

    pub fn ignore_file(&mut self, sender: String) {
        let dcc_send_receiver = some_or_return!(self.dcc_send_receivers.remove(&sender));

        dcc_send_receiver.decline_send_command().unwrap();
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

    pub fn open_add_channel_view(&mut self) {
        let channels: Vec<String> = self.process_list_end_message();
        let my_channels: Vec<String> = self.main_view.get_my_channels();
        let channels_not_mine: Vec<String> = channels_not_mine(channels, my_channels);

        self.add_channel_view = add_channel_view(&self.sender);
        self.add_channel_window = self
            .add_channel_view
            .get_view(self.app.clone(), channels_not_mine);
        self.add_channel_window.show();
    }

    pub fn open_add_client_view(&mut self, channels_and_clients: HashMap<String, Vec<String>>) {
        let clients_not_mine: Vec<String> = self.clients_not_mine(channels_and_clients);
        if vec_is_not_empty(&clients_not_mine) {
            self.add_client_window = add_client_window(&self.app, clients_not_mine, &self.sender);
            self.add_client_window.show();
        } else {
            self.send_open_warning_view(NO_CLIENTS_WARNING_TEXT);
        }
    }

    pub fn open_dcc_invitation_view(&mut self, client: String, message: SocketAddr) {
        let stream = self.get_stream();
        let dcc_receiver = DccChatReceiver::new(stream, client.clone());
        self.dcc_receivers.insert(client.clone(), dcc_receiver);

        self.dcc_invitation_window =
            dcc_invitation_window(&self.app, client, message, &self.sender);
        self.dcc_invitation_window.show();
    }

    pub fn open_file_chooser_dialog_view(&mut self) {
        let target = self.current_conv.clone();
        if target.is_empty() {
            return;
        }

        let title = "Please, select which file you wish to send".to_string();
        let file_chooser_dialog = FileChooserDialog::builder()
            .transient_for(&self.main_window)
            .action(gtk::FileChooserAction::Open)
            .title(&title)
            .build();

        file_chooser_dialog.add_button("Send", ResponseType::Accept);

        file_chooser_dialog.present();

        let sender = self.sender.clone();

        file_chooser_dialog.connect_response(move |file_chooser_dialog, response| {
            if response != ResponseType::Accept {
                return;
            }

            let file = some_or_return!(file_chooser_dialog.file());

            let path = some_or_return!(file.path());

            let target = target.clone();

            sender.send(SendDccSend { path, target }).unwrap();
            file_chooser_dialog.destroy();
        });
    }

    pub fn open_invite_client_view(&mut self, channels_and_clients: HashMap<String, Vec<String>>) {
        let my_channels: Vec<String> = self.main_view.get_my_channels();
        let current_conv_channels: Vec<String> = self.current_conv_channels(channels_and_clients);
        let channels_to_invite: Vec<String> = channels_not_mine(my_channels, current_conv_channels);
        if vec_is_not_empty(&channels_to_invite) {
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

        self.main_view = main_view(&self.sender);
        self.main_window = self.main_view.get_view(self.app.clone(), nickname);
        self.main_window.show();
    }

    pub fn open_notifications_view(&mut self) {
        notifications_window(&self.app, self.main_view.get_notifications()).show();
    }

    pub fn open_register_view(&mut self, address: String) {
        self.client = match Client::connect(address) {
            Ok(stream) => stream,
            Err(error) => panic!("{SERVER_CONNECT_ERROR_TEXT} {error:?}"),
        };

        self.start_listening();

        self.ip_window.close();
        self.register_window.show();
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

    pub fn receive_invite(&mut self, message: Message) {
        let (channel, nickname) = self.decode_invite_message(message);
        let message: String = format!("{nickname} has invited you to join {channel}");
        self.main_view.add_notification(message);
    }

    pub fn receive_join(&mut self, message: Message) {
        let channel = self.decode_join_message(message);

        self.add_channel_window.close();
        self.main_view.add_channel(channel.clone());
        self.change_conversation(channel);
    }

    pub fn receive_join_notification(&mut self, message: Message) {
        let (channel, client) = self.decode_join_notification_message(message);

        let message: String = format!("{client} has joined {channel}");
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

    pub fn receive_list_line(&mut self, message: Message) {
        let channel: String = self.decode_list_line_message(message);
        self.accumulated_channels_from_list.push(channel);
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

    pub fn receive_names_line(&mut self, message: Message) {
        let (channels, clients) = self.decode_names_line_message(message);
        self.accumulated_channels_from_names.push(channels);
        self.accumulated_clients_from_names.push(clients);
    }

    pub fn receive_priv_message(&mut self, message: Message) {
        match parse_ctcp(&message) {
            Some(message_text) => {
                self.receive_dcc_message(message, message_text);
            }
            None => {
                self.receive_regular_privmsg(message);
            }
        }
    }

    pub fn receive_result(&mut self, sender: String, name: String, result: io::Result<()>) {
        let position = self
            .downloads
            .iter()
            .position(|transfer| transfer.client == sender && transfer.name == name)
            .unwrap();

        if result.is_err() {
            self.downloads.get_mut(position).unwrap().failed = true;
        } else {
            self.downloads.remove(position);
        }

        self.transfer_result(result, sender);
    }

    pub fn receive_safe_message(&mut self, _client: String, message: String) {
        self.safe_conversation_view.receive_message(message);
    }

    pub fn register(&mut self, pass: String, nickname: String, username: String, realname: String) {
        let pass_command: String = format!("{PASS_COMMAND} {pass}");
        let nick_command: String = format!("{NICK_COMMAND} {nickname}");
        let user_command: String =
            format!("{USER_COMMAND} {username} {username} {username} :{realname}");
        self.client.send(&pass_command).expect(PASS_ERROR_TEXT);
        self.client.send(&nick_command).expect(NICK_ERROR_TEXT);
        self.client.send(&user_command).expect(USER_ERROR_TEXT);
    }

    pub fn regular_message(&mut self, message: String) {
        println!("{message}");
    }

    pub fn remove_conversation(&mut self) {
        self.main_view
            .remove_conversation(self.current_conv.clone());
        self.main_view.welcome_view();
    }

    pub fn send_dcc_send(&mut self, target: String, path: PathBuf) {
        let server_stream = self.get_stream();
        let dcc_send_sender = DccSendSender::send(server_stream, target.clone(), path).unwrap();

        self.dcc_send_senders.insert(target, dcc_send_sender);
    }

    pub fn send_invite_message(&mut self, channel: String) {
        self.invite_window.close();
        let invite: String = format!("{INVITE_COMMAND} {} {channel}", self.current_conv);
        self.client.send(&invite).expect(INVITE_ERROR_TEXT);
    }

    pub fn send_join_message(&mut self, channel: String) {
        let message: String = format!("{JOIN_COMMAND} {channel}");
        self.client.send(&message).expect(JOIN_ERROR_TEXT);
    }

    pub fn send_kick_message(&mut self, channel: String, member: String) {
        let message: String = format!("{KICK_COMMAND} {channel} {member}");
        self.client.send(&message).expect(KICK_ERROR_TEXT);
    }

    pub fn send_list_message(&mut self) {
        self.client.send(LIST_COMMAND).expect(LIST_ERROR_TEXT);
    }

    pub fn send_names_message_to_add_client(&mut self) {
        self.send_names_message(AddClient, None);
    }

    pub fn send_names_message_to_invite_client(&mut self) {
        let my_channels: Vec<String> = self.main_view.get_my_channels();
        if vec_is_not_empty(&my_channels) {
            self.send_names_message(InviteClient, None);
        } else {
            self.send_open_warning_view(NO_CHANNELS_WARNING_TEXT);
        }
    }

    pub fn send_names_message_to_know_members(&mut self) {
        self.send_names_message(KnowMembers, Some(self.current_conv.clone()));
    }

    pub fn send_part_message(&mut self) {
        let part_message: String = format!("{PART_COMMAND} {}", self.current_conv);
        self.client.send(&part_message).expect(PART_ERROR_TEXT);
    }

    pub fn send_priv_message(&mut self, message: String) {
        let priv_message = format!("{PRIVMSG_COMMAND} {} :{message}", self.current_conv);
        self.client.send(&priv_message).expect(PRIVMSG_ERROR_TEXT);
        self.main_view.send_message(message);
    }

    pub fn send_quit_message(&mut self) {
        self.client.send(QUIT_COMMAND).expect(QUIT_ERROR_TEXT);
    }

    pub fn send_result(&mut self, sender: String, result: io::Result<()>) {
        self.transfer_result(result, sender);
    }

    pub fn send_safe_conversation_request(&mut self) {
        let stream = self.get_stream();
        let chat_sender = DccChatSender::send(stream, self.current_conv.clone()).unwrap();
        self.dcc_senders
            .insert(self.current_conv.clone(), chat_sender);
    }

    pub fn send_safe_message(&mut self, receiver_client: String, message: String) {
        let dcc = self.dcc_chats.remove(&receiver_client);
        if let Some(mut dcc_chat) = dcc {
            dcc_chat.send_raw(&message).unwrap();
            self.dcc_chats.insert(receiver_client.clone(), dcc_chat);
            self.safe_conversation_view
                .send_message(message, receiver_client);
        }
    }
}
