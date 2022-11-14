mod controller_handler;
pub mod controller_message;
use crate::views::{
    view_register::RegisterView, views_add::{view_add_channel::AddChannelView, view_invite::InviteView},
    views_add::view_add_client::AddClientView,
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
        let mut add_channel_window = add_channel_view.get_view(app.clone());

        let mut add_client_view = AddClientView::new(sender.clone());
        let mut add_client_window = add_client_view.get_view(app.clone());

        let mut invite_window = InviteView::new(sender.clone()).get_view(app.clone());

        let mut current_conv = "".to_string();

        

        let app_clone = app.clone();
        let sender_clone = sender.clone();

        client.async_read(move |message| match message {
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
                    let pass_command = format!("PASS {}", pass);
                    let nick_command = format!("NICK {}", nickname);
                    let user_command =
                        format!("USER {} {} {} :{}", username, username, username, realname);
                    client.send_raw(&pass_command).expect("ERROR");
                    client.send_raw(&nick_command).expect("ERROR");
                    client.send_raw(&user_command).expect("ERROR");
                }
                ChangeViewToMain { nickname } => {
                    register_window.close();
                    main_view.get_view(app_clone.clone(), nickname).show();
                }
                SendPrivMessage { message } => {
                    let priv_message = format!("PRIVMSG {} :{}", current_conv, message);
                    client.send_raw(&priv_message).expect("ERROR");
                    main_view.send_message(message.to_string());
                }
                AddViewToAddChannel {} => {
                    add_channel_window =
                        AddChannelView::new(sender_clone.clone()).get_view(app_clone.clone());
                    add_channel_window.show();
                }
                AddViewToAddClient {} => {
                    add_client_window =
                        AddClientView::new(sender_clone.clone()).get_view(app_clone.clone());
                    add_client_window.show();
                }
                AddNewChannel { channel } => {
                    add_channel_window.close();
                    main_view.add_channel(channel);
                }
                AddNewClient { client } => {
                    add_client_window.close();
                    main_view.add_client(client);
                }
                JoinChannel { channel } => {
                    let join_message = format!("JOIN {}", channel);
                    client.send_raw(&join_message).expect("ERROR");
                }
                ReceivePrivMessage { nickname, message } => {
                    main_view.receive_priv_message(message, nickname);
                }
                ChangeConversation { nickname } => {
                    current_conv = nickname;
                    main_view.change_conversation(current_conv.clone());
                }
                QuitChannel {} => {
                    let part_message = format!("PART {}", current_conv);
                    client.send_raw(&part_message).expect("ERROR: Part message");
                    main_view.remove_channel(current_conv.clone());
                }
                AddInviteView {} => {
                    invite_window = InviteView::new(sender_clone.clone()).get_view(app_clone.clone());
                    invite_window.show()
                }
                SendInviteMessage { channel } => {
                    invite_window.close();
                    let invite = format!("INVITE {} {}", current_conv, channel);
                    client.send_raw(&invite).expect("ERROR");
                }
                RecieveInvite { nickname, channel } => {
                    let message = format!("{} has invited you to join {}", nickname, channel);
                    main_view.receive_priv_message(message, channel);
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
}
