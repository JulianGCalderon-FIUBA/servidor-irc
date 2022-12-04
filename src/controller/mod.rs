pub mod controller_handler;
pub mod controller_message;

use std::collections::HashMap;

use crate::{
    server::consts::commands::{
        INVITE_COMMAND, JOIN_COMMAND, LIST_COMMAND, NAMES_COMMAND, NICK_COMMAND, PART_COMMAND,
        PASS_COMMAND, PRIVMSG_COMMAND, USER_COMMAND, KICK_COMMAND,
    },
    views::{
        view_register::RegisterView,
        views_add::{
            view_add_channel::AddChannelView, view_invite::InviteView, view_warning::WarningView,
        },
        views_add::{view_add_client::AddClientView, view_channel_members::ChannelMembersView},
    },
};
use gtk4 as gtk;

use crate::{client::Client, views::view_main::MainView, ADDRESS};
use gtk::{gdk::Display, glib, prelude::*, Application, CssProvider, StyleContext};

use controller_handler::to_controller_message;
use controller_message::ControllerMessage::*;

use self::controller_message::ControllerMessage;

const ERROR_TEXT: &str = "ERROR";
const NO_CHANNELS_WARNING_TEXT: &str = "There are no clients to chat with.";
pub struct Controller {
    app: Application,
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    pub fn new() -> Self {
        let app = Application::new(Some("com.lemon-pie.demo"), Default::default());

        Self { app }
    }

    fn load_css() {
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.scss"));

        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn start(&mut self) {
        self.app.connect_startup(|_| Self::load_css());
        self.app.connect_activate(Self::build_ui);
        self.app.run();
    }

    fn build_ui(app: &Application) {
        let mut client = match Client::new(ADDRESS.to_string()) {
            Ok(stream) => stream,
            Err(error) => panic!("Error connecting to server: {:?}", error),
        };

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let mut register_view = RegisterView::new(sender.clone());
        let register_window = register_view.get_view(app.clone());
        register_window.show();

        let mut main_view = MainView::new(sender.clone());

        let mut add_channel_view = AddChannelView::new(sender.clone());
        let mut add_channel_window = add_channel_view.get_view(app.clone(), vec![]);

        let mut add_client_view = AddClientView::new(sender.clone());
        let mut add_client_window = add_client_view.get_view(app.clone(), vec![]);

        let mut invite_window = InviteView::new(sender.clone()).get_view(app.clone(), vec![]);

        let mut current_conv = "".to_string();

        let app_clone = app.clone();
        let sender_clone = sender.clone();

        let mut current_nickname: String = String::from("");
        let mut trying_to_add_client: bool = false;

        receiver.attach(None, move |msg| {
            match msg {
                Register {
                    pass,
                    nickname,
                    username,
                    realname,
                    address,
                } => {
                    client = match Client::new(address) {
                        Ok(stream) => stream,
                        Err(error) => panic!("Error connecting to server: {:?}", error),
                    };

                    let pass_command = format!("{} {}", PASS_COMMAND, pass);
                    let nick_command = format!("{} {}", NICK_COMMAND, nickname);
                    let user_command = format!(
                        "{} {} {} {} :{}",
                        USER_COMMAND, username, username, username, realname
                    );
                    client.send_raw(&pass_command).expect(ERROR_TEXT);
                    client.send_raw(&nick_command).expect(ERROR_TEXT);
                    client.send_raw(&user_command).expect(ERROR_TEXT);

                    let sender_clone = sender.clone();
                    client.start_async_read(move |message| match message {
                        Ok(message) => {
                            let controller_message = to_controller_message(message);
                            sender_clone.send(controller_message).unwrap();
                        }
                        Err(error) => eprintln!("Failed to read message: {}", error),
                    });
                }
                ChangeViewToMain { nickname } => {
                    register_window.close();
                    current_nickname = String::from(&nickname.to_string()[..]);
                    main_view.get_view(app_clone.clone(), nickname).show();
                }
                SendPrivMessage { message } => {
                    let priv_message = format!("{} {} :{}", PRIVMSG_COMMAND, current_conv, message);
                    client.send_raw(&priv_message).expect(ERROR_TEXT);
                    main_view.send_message(message.to_string(), current_conv.clone());
                }
                SendNamesMessageToAddClient {} => {
                    trying_to_add_client = true;
                    client.send_raw(NAMES_COMMAND).expect(ERROR_TEXT);
                }
                JoinChannel { channel } => {
                    add_channel_window.close();
                    let join_message = format!("{} {}", JOIN_COMMAND, channel);
                    client.send_raw(&join_message).expect(ERROR_TEXT);
                    main_view.add_channel(channel.to_string());
                }
                AddNewClient { new_client } => {
                    add_client_window.close();
                    main_view.add_client(new_client.to_string());
                }
                AddViewToAddClient {
                    channels_and_clients,
                } => {
                    let clients_to_add: Vec<String> = Self::not_mine(
                        Self::clients_to_add(channels_and_clients, current_nickname.clone()),
                        main_view.get_my_clients(),
                    );

                    if !clients_to_add.is_empty() {
                        add_client_window = AddClientView::new(sender_clone.clone())
                            .get_view(app_clone.clone(), clients_to_add);
                        add_client_window.show();
                    } else {
                        WarningView::new()
                            .get_view(app_clone.clone(), NO_CHANNELS_WARNING_TEXT.to_string())
                            .show();
                    }
                }
                ReceivePrivMessage {
                    sender_nickname,
                    message,
                    channel,
                } => {
                    if let Some(..) = channel {
                        main_view.receive_priv_channel_message(
                            message,
                            sender_nickname,
                            channel.unwrap(),
                            current_conv.clone(),
                        );
                    } else {
                        main_view.receive_priv_client_message(
                            message,
                            sender_nickname,
                            current_conv.clone(),
                        );
                    }
                }
                ChangeConversation { nickname } => {
                    let last_conv = current_conv.clone();
                    current_conv = nickname;
                    main_view.change_conversation(last_conv, current_conv.clone());
                }
                QuitChannel {} => {
                    let part_message = format!("{} {}", PART_COMMAND, current_conv);
                    client.send_raw(&part_message).expect("ERROR: Part message");
                }
                RemoveConversation {} => {
                    main_view.remove_conversation(current_conv.clone());
                    main_view.welcome_view();
                }
                AddInviteView {} => {
                    let my_channels = main_view.get_my_channels();
                    if !my_channels.is_empty() {
                        invite_window = InviteView::new(sender_clone.clone())
                            .get_view(app_clone.clone(), my_channels);
                        invite_window.show();
                    }
                }
                SendInviteMessage { channel } => {
                    invite_window.close();
                    let invite = format!("{} {} {}", INVITE_COMMAND, current_conv, channel);
                    client.send_raw(&invite).expect(ERROR_TEXT);
                }
                RecieveInvite { nickname, channel } => {
                    let message = format!("{} has invited you to join {}", nickname, channel);
                    main_view.receive_priv_client_message(message, channel, current_conv.clone());
                }
                SendListMessage {} => {
                    client.send_raw(LIST_COMMAND).expect(ERROR_TEXT);
                }
                ReceiveListChannels { channels } => {
                    add_channel_window = AddChannelView::new(sender_clone.clone()).get_view(
                        app_clone.clone(),
                        Self::not_mine(channels, main_view.get_my_channels()),
                    );
                    add_channel_window.show();
                }
                SendNamesMessageToKnowMembers {} => {
                    trying_to_add_client = false;
                    client
                        .send_raw(&format!("{NAMES_COMMAND} {current_conv}"))
                        .expect(ERROR_TEXT);
                }
                ReceiveNamesChannels {
                    channels_and_clients,
                } => {
                    if trying_to_add_client {
                        sender_clone
                            .send(ControllerMessage::AddViewToAddClient {
                                channels_and_clients,
                            })
                            .expect(ERROR_TEXT);
                    } else {
                        ChannelMembersView::new()
                            .get_view(
                                app_clone.clone(),
                                channels_and_clients[&current_conv].clone(),
                                current_nickname.clone(),
                                current_conv.clone(),
                                sender.clone()
                            )
                            .show();
                    }
                }
                KickMember { channel, member } => {
                    let kick = format!("{} {} {}", KICK_COMMAND, channel, member);
                    client.send_raw(&kick).expect(ERROR_TEXT);
                }
                ReceiveKick { kicked, channel } => {
                    println!("kick {} from {}", kicked, channel);
                    if kicked == current_nickname {
                        main_view.remove_conversation(channel.clone());
                    }
                    if channel == current_conv {
                        main_view.welcome_view();
                    }
                }
                RegularMessage { message } => {
                    println!("{}", message);
                }
            }
            // Returning false here would close the receiver
            // and have senders fail
            glib::Continue(true)
        });
    }

    fn not_mine(all: Vec<String>, mine: Vec<String>) -> Vec<String> {
        let mut not_mine: Vec<String> = vec![];
        for element in &all {
            if !mine.contains(element) {
                not_mine.push(element.clone());
            }
        }
        not_mine
    }

    fn server_clients(channels_and_clients: HashMap<String, Vec<String>>) -> Vec<String> {
        let mut clients_set: Vec<String> = vec![];
        for clients_of_channel in channels_and_clients.values() {
            for client in clients_of_channel {
                if !clients_set.contains(client) {
                    clients_set.push(client.to_string());
                }
            }
        }
        clients_set
    }

    fn clients_to_add(
        channels_and_clients: HashMap<String, Vec<String>>,
        current_nickname: String,
    ) -> Vec<String> {
        let mut all_clients = Self::server_clients(channels_and_clients);
        if all_clients.contains(&current_nickname) {
            all_clients.remove(
                all_clients
                    .iter()
                    .position(|x| *x == current_nickname)
                    .unwrap(),
            );
        }
        all_clients
    }

    // fn listen_messages(mut client: Client, sender: Sender<ControllerMessage>) {
    //     client.start_async_read(move |message| match message {
    //         Ok(message) => {
    //             let controller_message = to_controller_message(message);
    //             sender.send(controller_message).unwrap();
    //         }
    //         Err(error) => eprintln!("Failed to read message: {}", error),
    //     });
    // }
}
