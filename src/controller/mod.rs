/// Contains function responsible of handling controller messages
/// the server receives.
pub mod controller_handler;

/// Definition of messages the controller can send and receive.
pub mod controller_message;

use std::collections::HashMap;

use crate::{
    server::consts::commands::{
        INVITE_COMMAND,
        JOIN_COMMAND,
        KICK_COMMAND,
        LIST_COMMAND,
        NAMES_COMMAND,
        NICK_COMMAND,
        PART_COMMAND,
        PASS_COMMAND,
        PRIVMSG_COMMAND,
        QUIT_COMMAND,
        USER_COMMAND,
    },
    views::{
        add_views::{
            add_channel_view::AddChannelView,
            invite_view::InviteView,
            notifications_view::NotificationsView,
            user_info_view::UserInfoView,
            warning_view::WarningView,
        },
        add_views::{
            add_client_view::AddClientView,
            channel_members_view::ChannelMembersView,
            safe_conversation_view::{ SafeConversationView, self },
        },
        ip_view::IpView,
        register_view::RegisterView,
    },
};
use gtk4 as gtk;

use crate::{ client::Client, views::main_view::MainView, ADDRESS };
use gtk::{ gdk::Display, glib, prelude::*, Application, CssProvider, StyleContext };

use controller_handler::to_controller_message;
use controller_message::ControllerMessage::*;

use self::controller_message::ControllerMessage;

const ADD_VIEW_ADD_CLIENT_ERROR_TEXT: &str = "ERROR: Add client view";
const ADD_VIEW_INVITE_ERROR_TEXT: &str = "ERROR: Add invite view";
const INVITE_ERROR_TEXT: &str = "ERROR: INVITE command";
const JOIN_ERROR_TEXT: &str = "ERROR: JOIN command";
const KICK_ERROR_TEXT: &str = "ERROR: KICK command";
const LIST_ERROR_TEXT: &str = "ERROR: LIST command";
const NAMES_ERROR_TEXT: &str = "ERROR: NAMES command";
const NICK_ERROR_TEXT: &str = "ERROR: NICK command";
const OPEN_WARNING_ERROR_TEXT: &str = "ERROR: Open warning";
const PART_ERROR_TEXT: &str = "ERROR: PART command";
const PASS_ERROR_TEXT: &str = "ERROR: PASS command";
const PRIVMSG_ERROR_TEXT: &str = "ERROR: PRIVMSG command";
const QUIT_ERROR_TEXT: &str = "ERROR: QUIT command";
const USER_ERROR_TEXT: &str = "ERROR: USER command";

const NO_CLIENTS_WARNING_TEXT: &str = "There are no clients to chat with.";
const NO_CHANNELS_WARNING_TEXT: &str = "You are not in any channel.";
const CLIENT_IS_ALREADY_IN_CHANNELS_WARNING_TEXT: &str =
    "The client you want to invite is already in all your channels.";
const ERR_NICK_COLLISION_WARNING_TEXT: &str = "The nickname is in use, please pick another one.";

const DISPLAY_CONNECT_ERROR_TEXT: &str = "Could not connect to a display.";
const SERVER_CONNECT_ERROR_TEXT: &str = "Error connecting to server";
const FAILED_TO_READ_MESSAGE_ERROR_TEXT: &str = "Failed to read message";

/// Has a reference to the application.
/// Communicates with the views and the server.
/// Handles server errors.
pub struct Controller {
    app: Application,
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    /// Creates new [`Controller`]
    pub fn new() -> Self {
        let app = Application::new(Some("com.lemon-pie.demo"), Default::default());

        Self { app }
    }

    /// Loads css for widgets
    fn load_css() {
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.scss"));

        StyleContext::add_provider_for_display(
            &Display::default().expect(DISPLAY_CONNECT_ERROR_TEXT),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );
    }

    /// Starts running the application in the [`Controller`]
    pub fn start(&mut self) {
        self.app.connect_startup(|_| Self::load_css());
        self.app.connect_activate(Self::build_ui);
        self.app.run();
    }

    /// Builds ui to show windows and handle messages
    fn build_ui(app: &Application) {
        let mut client = match Client::new(ADDRESS.to_string()) {
            Ok(stream) => stream,
            Err(error) => panic!("{SERVER_CONNECT_ERROR_TEXT}: {error:?}"),
        };

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let ip_window = IpView::new(sender.clone()).get_view(app.clone());
        ip_window.show();

        let register_window = RegisterView::new(sender.clone()).get_view(app.clone());

        let mut main_view = MainView::new(sender.clone());
        let mut main_window = MainView::new(sender.clone()).get_view(app.clone(), "".to_string());

        let mut add_channel_window = AddChannelView::new(sender.clone()).get_view(
            app.clone(),
            vec![]
        );

        let mut add_client_window = AddClientView::new(sender.clone()).get_view(
            app.clone(),
            vec![]
        );

        let mut safe_conversation_window = SafeConversationView::new(sender.clone()).get_view(
            app.clone()
        );

        let mut invite_window = InviteView::new(sender.clone()).get_view(app.clone(), vec![]);

        let mut current_conv = "".to_string();

        let app_clone = app.clone();
        let sender_clone = sender.clone();

        let mut current_nickname: String = String::from("");
        let mut current_realname: String = String::from("");
        let mut current_servername: String = String::from("");
        let mut current_username: String = String::from("");
        let mut trying_to_add_client: bool = false;
        let mut trying_to_invite_client: bool = false;

        receiver.attach(None, move |msg| {
            match msg {
                AddNewClient { new_client } => {
                    add_client_window.close();
                    main_view.add_client(new_client.to_string());
                }
                ChangeConversation { nickname } => {
                    let last_conv = current_conv.clone();
                    current_conv = nickname;
                    main_view.change_conversation(last_conv, current_conv.clone());
                }
                JoinChannel { channel } => {
                    add_channel_window.close();
                    let join_message = format!("{JOIN_COMMAND} {channel}");
                    client.send_raw(&join_message).expect(JOIN_ERROR_TEXT);
                    main_view.add_channel(channel);
                }
                KickMember { channel, member } => {
                    let kick = format!("{KICK_COMMAND} {channel} {member}");
                    client.send_raw(&kick).expect(KICK_ERROR_TEXT);
                }
                OpenAddClientView { channels_and_clients } => {
                    let clients_to_add: Vec<String> = Self::clients_not_mine(
                        Self::clients_to_add(channels_and_clients, current_nickname.clone()),
                        main_view.get_my_clients()
                    );

                    if !clients_to_add.is_empty() {
                        add_client_window = AddClientView::new(sender_clone.clone()).get_view(
                            app_clone.clone(),
                            clients_to_add
                        );
                        add_client_window.show();
                    } else {
                        sender_clone
                            .send(ControllerMessage::OpenWarningView {
                                message: NO_CLIENTS_WARNING_TEXT.to_string(),
                            })
                            .expect(OPEN_WARNING_ERROR_TEXT);
                    }
                }
                OpenInviteClientView { channels_and_clients } => {
                    let channels_to_invite = Self::channels_not_mine(
                        main_view.get_my_channels(),
                        Self::client_channels(channels_and_clients, current_conv.clone())
                    );
                    if !channels_to_invite.is_empty() {
                        invite_window = InviteView::new(sender_clone.clone()).get_view(
                            app_clone.clone(),
                            channels_to_invite
                        );
                        invite_window.show();
                    } else {
                        sender_clone
                            .send(ControllerMessage::OpenWarningView {
                                message: CLIENT_IS_ALREADY_IN_CHANNELS_WARNING_TEXT.to_string(),
                            })
                            .expect(OPEN_WARNING_ERROR_TEXT);
                    }
                }
                OpenMainView { realname, servername, nickname, username } => {
                    register_window.close();
                    current_realname = String::from(&realname[..]);
                    current_servername = String::from(&servername[..]);
                    current_nickname = String::from(&nickname[..]);
                    current_username = String::from(&username[..]);
                    main_window = main_view.get_view(app_clone.clone(), nickname);
                    main_window.show();
                }
                OpenNotificationsView {} => {
                    NotificationsView::new()
                        .get_view(app_clone.clone(), main_view.get_notifications())
                        .show();
                }
                OpenSafeConversationView {} => {
                    main_window.hide();
                    safe_conversation_window = SafeConversationView::new(
                        sender_clone.clone()
                    ).get_view(app_clone.clone());
                    safe_conversation_window.show();
                }
                OpenUserInfoView {} => {
                    UserInfoView::new()
                        .get_view(
                            app_clone.clone(),
                            current_realname.clone(),
                            current_servername.clone(),
                            current_nickname.clone(),
                            current_username.clone()
                        )
                        .show();
                }
                OpenWarningView { message } => {
                    WarningView::new().get_view(app_clone.clone(), message).show();
                }
                Quit {} => {
                    let quit_message = QUIT_COMMAND.to_string();
                    client.send_raw(&quit_message).expect(QUIT_ERROR_TEXT);
                }
                QuitChannel {} => {
                    let part_message = format!("{PART_COMMAND} {current_conv}");
                    client.send_raw(&part_message).expect(PART_ERROR_TEXT);
                }
                ReceiveInvite { nickname, channel } => {
                    let message = format!("{nickname} has invited you to join {channel}");
                    main_view.add_notification(message);
                }
                ReceiveKick { kicked, channel } => {
                    if kicked == current_nickname {
                        main_view.remove_conversation(channel.clone());
                        if channel == current_conv {
                            main_view.welcome_view();
                        }
                    }
                }
                ReceiveListChannels { channels } => {
                    add_channel_window = AddChannelView::new(sender_clone.clone()).get_view(
                        app_clone.clone(),
                        Self::channels_not_mine(channels, main_view.get_my_channels())
                    );
                    add_channel_window.show();
                }
                ReceiveNamesChannels { channels_and_clients } => {
                    if trying_to_add_client {
                        sender_clone
                            .send(ControllerMessage::OpenAddClientView {
                                channels_and_clients,
                            })
                            .expect(ADD_VIEW_ADD_CLIENT_ERROR_TEXT);
                    } else if trying_to_invite_client {
                        sender_clone
                            .send(ControllerMessage::OpenInviteClientView {
                                channels_and_clients,
                            })
                            .expect(ADD_VIEW_INVITE_ERROR_TEXT);
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
                ReceivePrivMessage { sender_nickname, message, channel } => {
                    if let Some(..) = channel {
                        main_view.receive_priv_channel_message(
                            message,
                            sender_nickname,
                            channel.unwrap(),
                            current_conv.clone()
                        );
                    } else {
                        main_view.receive_priv_client_message(
                            message,
                            sender_nickname,
                            current_conv.clone()
                        );
                    }
                }
                Register { pass, nickname, username, realname } => {
                    let pass_command = format!("{PASS_COMMAND} {pass}");
                    let nick_command = format!("{NICK_COMMAND} {nickname}");
                    let user_command = format!(
                        "{USER_COMMAND} {username} {username} {username} :{realname}"
                    );
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
                RegularMessage { message } => {
                    println!("{message}");
                }
                RemoveConversation {} => {
                    main_view.remove_conversation(current_conv.clone());
                    main_view.welcome_view();
                }
                SendInviteMessage { channel } => {
                    invite_window.close();
                    let invite = format!("{INVITE_COMMAND} {current_conv} {channel}");
                    client.send_raw(&invite).expect(INVITE_ERROR_TEXT);
                }
                SendListMessage {} => {
                    client.send_raw(LIST_COMMAND).expect(LIST_ERROR_TEXT);
                }
                SendNamesMessageToAddClient {} => {
                    trying_to_add_client = true;
                    trying_to_invite_client = false;
                    client.send_raw(NAMES_COMMAND).expect(NAMES_ERROR_TEXT);
                }
                SendNamesMessageToInviteClient {} => {
                    let my_channels = main_view.get_my_channels();
                    if !my_channels.is_empty() {
                        trying_to_add_client = false;
                        trying_to_invite_client = true;
                        client.send_raw(NAMES_COMMAND).expect(NAMES_ERROR_TEXT);
                    } else {
                        sender_clone
                            .send(ControllerMessage::OpenWarningView {
                                message: NO_CHANNELS_WARNING_TEXT.to_string(),
                            })
                            .expect(OPEN_WARNING_ERROR_TEXT);
                    }
                }
                SendNamesMessageToKnowMembers {} => {
                    trying_to_add_client = false;
                    trying_to_invite_client = false;
                    client
                        .send_raw(&format!("{NAMES_COMMAND} {current_conv}"))
                        .expect(NAMES_ERROR_TEXT);
                }
                SendPrivMessage { message } => {
                    let priv_message = format!("{PRIVMSG_COMMAND} {current_conv} :{message}");
                    client.send_raw(&priv_message).expect(PRIVMSG_ERROR_TEXT);
                    main_view.send_message(message.to_string(), current_conv.clone());
                }
                ToRegister { address } => {
                    client = match Client::new(address) {
                        Ok(stream) => stream,
                        Err(error) => panic!("{SERVER_CONNECT_ERROR_TEXT} {error:?}"),
                    };
                    ip_window.close();
                    register_window.show();
                }
            }
            // Returning false here would close the receiver
            // and have senders fail
            glib::Continue(true)
        });
    }

    /// Returns clients that are not from the current user.
    ///
    /// Receives a Vec<String> and a Vec<String>, returns a Vec<String>
    fn clients_not_mine(all: Vec<String>, mine: Vec<String>) -> Vec<String> {
        let mut not_mine: Vec<String> = vec![];
        for element in &all {
            let no_operator_indicator = if let Some(stripped) = element.strip_prefix('@') {
                stripped.to_string()
            } else {
                element.to_string()
            };
            if !mine.contains(&no_operator_indicator) {
                not_mine.push(no_operator_indicator.clone());
            }
        }
        not_mine
    }

    /// Returns channels that are not from the current user.
    ///
    /// Receives a Vec<String> and a Vec<String>, returns a Vec<String>
    fn channels_not_mine(all: Vec<String>, mine: Vec<String>) -> Vec<String> {
        let mut not_mine: Vec<String> = vec![];
        for element in &all {
            if !mine.contains(element) {
                not_mine.push(element.clone());
            }
        }
        not_mine
    }

    /// Returns all server clients.
    ///
    /// Receives a HashMap<String, Vec<String>>, returns a Vec<String>
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

    /// Returns all clients to add.
    ///
    /// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
    fn clients_to_add(
        channels_and_clients: HashMap<String, Vec<String>>,
        current_nickname: String
    ) -> Vec<String> {
        let mut all_clients = Self::server_clients(channels_and_clients);
        if all_clients.contains(&current_nickname) {
            all_clients.remove(
                all_clients
                    .iter()
                    .position(|x| *x == current_nickname)
                    .unwrap()
            );
        }
        if all_clients.contains(&format!("@{current_nickname}")) {
            all_clients.remove(
                all_clients
                    .iter()
                    .position(|x| *x == format!("@{current_nickname}"))
                    .unwrap()
            );
        }

        all_clients
    }

    /// Returns all channels from a client.
    ///
    /// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
    fn client_channels(
        channels_and_clients: HashMap<String, Vec<String>>,
        client: String
    ) -> Vec<String> {
        let mut client_channels_set: Vec<String> = vec![];
        for channel in channels_and_clients.keys() {
            let mut clients: Vec<String> = vec![];
            for element in channels_and_clients.get(channel).unwrap() {
                let no_operator_indicator: String = if
                    let Some(stripped) = element.strip_prefix('@')
                {
                    stripped.to_string()
                } else {
                    element.to_string()
                };
                clients.push(no_operator_indicator);
            }
            if clients.contains(&client) {
                client_channels_set.push((&channel).to_string());
            }
        }
        client_channels_set
    }
}