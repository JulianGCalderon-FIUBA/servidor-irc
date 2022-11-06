mod controller_handler;
pub mod controller_message;
use crate::views::view_register::RegisterView;
use gtk4 as gtk;

use crate::{ client::Client, views::view_main::MainView, ADDRESS };
use gtk::{ gdk::Display, glib::{ self }, prelude::*, Application, CssProvider, StyleContext };

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

        let mut view = RegisterView::new(sender.clone());
        let window = view.get_view(app.clone());
        window.show();

        let app_clone = app.clone();

        let sender_clone = sender.clone();
        client.async_read(move |message| {
            match message {
                Ok(message) => {
                    let controller_message = to_controller_message(message);
                    sender_clone.send(controller_message).unwrap();
                }
                Err(error) => eprintln!("Failed to read message: {}", error),
            }
        });

        receiver.attach(None, move |msg| {
            match msg {
                Register { pass, nickname, username, realname } => {
                    let pass_command = format!("PASS {}", pass);
                    let nick_command = format!("NICK {}", nickname);
                    let user_command = format!(
                        "USER {} {} {} :{}",
                        username,
                        username,
                        username,
                        realname
                    );
                    client.send_raw(&pass_command).expect("ERROR");
                    client.send_raw(&nick_command).expect("ERROR");
                    client.send_raw(&user_command).expect("ERROR");
                }
                ChangeViewToMain {} => {
                    window.close();
                    let mut main_view = MainView::new(sender.clone());
                    main_view.get_view(app_clone.clone()).show();
                }
                SendPrivMessage { nickname, message } => {
                    let priv_message = format!("PRIVMSG {} :{}", nickname, message);
                    client.send_raw(&priv_message).expect("ERROR");
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

    // fn handle_message(msg: String, view: RegisterView) {

    // }

    // fn print_message(message: Result<Message, CreationError>) {
    //     match message {
    //         Ok(message) => println!("{}", message),
    //         Err(error) => eprintln!("Failed to read message: {}", error),
    //     }
    // }
}

// pub fn send_msg(message: Result<Message, CreationError>, sender: Sender<String>) {

//     match message {
//                 Ok(message) => sender.send(message.to_string()).unwrap(),
//                 Err(error) => (eprintln!("Failed to read message: {}", error)),
//             }
// }
