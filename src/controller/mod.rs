/// Contains function responsible of handling controller messages
/// the server receives.
pub mod controller_handler;

/// Definition of messages the controller can send and receive.
pub mod controller_message;
pub mod interface_controller;

use gtk4 as gtk;

use crate::{client::Client, ADDRESS};
use gtk::{gdk::Display, glib, prelude::*, Application, CssProvider, StyleContext};

use self::{controller_message::ControllerMessage, interface_controller::InterfaceController};

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
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
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
        let client = match Client::new(ADDRESS.to_string()) {
            Ok(stream) => stream,
            Err(error) => panic!("{SERVER_CONNECT_ERROR_TEXT}: {error:?}"),
        };

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        InterfaceController::new(app.clone(), client, sender).build(receiver);
    }
}
