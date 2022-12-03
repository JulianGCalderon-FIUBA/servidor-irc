mod controller_handler;
pub mod controller_message;

use crate::{
    server::consts::commands::{
        INVITE_COMMAND, JOIN_COMMAND, LIST_COMMAND, NAMES_COMMAND, NICK_COMMAND, PART_COMMAND,
        PASS_COMMAND, PRIVMSG_COMMAND, USER_COMMAND,
    },
    views::{
        view_register::RegisterView,
        views_add::{view_add_channel::AddChannelView, view_invite::InviteView},
        views_add::{view_add_client::AddClientView, view_channel_members::ChannelMembersView},
    },
};
use gtk4 as gtk;

use crate::{client::Client, views::view_main::MainView, ADDRESS};
use gtk::{
    gdk::Display,
    glib::{self},
    prelude::*,
    Application, CssProvider, StyleContext,
};

use controller_handler::to_controller_message;
use controller_message::ControllerMessage::*;

const ERROR_TEXT: &str = "ERROR";

pub struct Controller {
    app: Application,
    address: String
}

// impl Default for Controller {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Controller {
    pub fn new(address: String) -> Self {
        let app = Application::new(Some("com.lemon-pie.demo"), Default::default());

        Self { app , address}
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
        let mut add_client_window = add_client_view.get_view(app.clone());

        let mut invite_window = InviteView::new(sender.clone()).get_view(app.clone(), vec![]);

        // let mut channel_members_window = ChannelMembersView::new().get_view(app.clone(), vec![]);

        let mut current_conv = "".to_string();

        let app_clone = app.clone();
        let sender_clone = sender.clone();

        let mut _current_nickname: String = String::from("");

        client.start_async_read(move |message| match message {
            Ok(message) => {
                let controller_message = to_controller_message(message);
                sender.send(controller_message).unwrap();
            }
            Err(error) => eprintln!("Failed to read message: {}", error),
        });

        receiver.attach(None, move |msg| {
            match msg {
                Register {
                    pass,
                    nickname,
                    username,
                    realname,
                } => {
                    let pass_command = format!("{} {}", PASS_COMMAND, pass);
                    let nick_command = format!("{} {}", NICK_COMMAND, nickname);
                    let user_command = format!(
                        "{} {} {} {} :{}",
                        USER_COMMAND, username, username, username, realname
                    );
                    client.send_raw(&pass_command).expect(ERROR_TEXT);
                    client.send_raw(&nick_command).expect(ERROR_TEXT);
                    client.send_raw(&user_command).expect(ERROR_TEXT);
                }
                ChangeViewToMain { nickname } => {
                    register_window.close();
                    _current_nickname = String::from(&nickname.to_string()[..]);
                    main_view.get_view(app_clone.clone(), nickname).show();
                }
                SendPrivMessage { message } => {
                    let priv_message = format!("{} {} :{}", PRIVMSG_COMMAND, current_conv, message);
                    client.send_raw(&priv_message).expect(ERROR_TEXT);
                    main_view.send_message(message.to_string(), current_conv.clone());
                }
                AddViewToAddClient {} => {
                    add_client_window =
                        AddClientView::new(sender_clone.clone()).get_view(app_clone.clone());
                    add_client_window.show();
                }
                JoinChannel { channel } => {
                    add_channel_window.close();
                    let join_message = format!("{} {}", JOIN_COMMAND, channel);
                    client.send_raw(&join_message).expect(ERROR_TEXT);
                    main_view.add_channel(channel);
                }
                AddNewClient { client } => {
                    add_client_window.close();
                    main_view.add_client(client);
                }
                ReceivePrivMessage { nickname, message } => {
                    main_view.receive_priv_message(message, nickname, current_conv.clone());
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
                    println!("{}", message);
                    main_view.receive_priv_message(message, nickname, current_conv.clone());
                }
                SendListMessage {} => {
                    client.send_raw(LIST_COMMAND).expect(ERROR_TEXT);
                }
                ReceiveListChannels { channels } => {
                    add_channel_window = AddChannelView::new(sender_clone.clone()).get_view(
                        app_clone.clone(),
                        Self::not_my_channels(channels, main_view.get_my_channels()),
                    );
                    add_channel_window.show();
                }
                SendNamesMessage {} => {
                    client.send_raw(NAMES_COMMAND).expect(ERROR_TEXT);
                }
                ReceiveNamesChannels {
                    channels_and_clients,
                } => {
                    ChannelMembersView::new().get_view(
                        app_clone.clone(),
                        channels_and_clients[&current_conv].clone(),
                    ).show();
                    // channel_members_window.show();
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

    fn not_my_channels(all_channels: Vec<String>, my_channels: Vec<String>) -> Vec<String> {
        let mut not_my_channels: Vec<String> = vec![];
        for channel in &all_channels {
            if !my_channels.contains(channel) {
                not_my_channels.push(channel.clone());
            }
        }
        not_my_channels
    }
}
