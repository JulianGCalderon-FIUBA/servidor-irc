mod chat;
mod conv_info;
mod login;
mod sidebar;

use gtk::{
    Align,
    Application,
    ApplicationWindow,
    Box,
    Button,
    Entry,
    Label,
    Orientation,
    prelude::*,
    ScrolledWindow,
    Separator,
    glib::Sender
};
use gtk4 as gtk;

use chat::Chat;
use conv_info::ConvInfo;
use sidebar::Sidebar;

pub struct MainView {
    pub channels: Vec<Button>,
    pub add_channel: Button,
    pub clients: Vec<Button>,
    pub add_client: Button,
    pub messages: Vec<Label>,
    pub user_info: Button,
    pub send_message: Button,
    pub input: Entry,
    pub channel_info: Button,
    pub quit_channel: Button,
    pub func_channel: Button,
    sender: Sender<String>,
}
impl MainView {
    pub fn new(sender: Sender<String>) -> Self {
        Self {
            channels: vec![
                create_button("#channel1"),
                create_button("#channel2")
            ],
            add_channel: create_button("+"),
            clients: vec![
                create_button("juli"),
                create_button("sol"),
                create_button("santi"),
                create_button("ana")
            ],
            add_client: create_button("+"),
            messages: vec![create_message("hola!")],
            user_info: create_button("info"),
            input: create_entry("Message..."),
            send_message: create_button("send"),
            quit_channel: create_button("x"),
            channel_info: create_button("info"),
            func_channel: create_button("func"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder().application(&app).title("Lemon Pie IRC").build();

        let main_box = create_main_box(Orientation::Horizontal, 800, 1200);
        main_box.add_css_class("main_box");

        let sidebar = Box::builder()
            .orientation(Orientation::Vertical)
            .build();

        sidebar.append(&self.channels[0]);
        sidebar.append(&self.channels[1]);

        self.add_channel.add_css_class("add");
        sidebar.append(&self.add_channel);

        let separator = create_separator_sidebar();
        sidebar.append(&separator);

        sidebar.append(&self.clients[0]);
        sidebar.append(&self.clients[1]);
        sidebar.append(&self.clients[2]);
        sidebar.append(&self.clients[3]);

        self.add_client.add_css_class("add");
        sidebar.append(&self.add_client);

        main_box.append(&sidebar);

        //-------------------------------------

        let separator = create_separator();
        main_box.append(&separator);

        //-------------------------------------

        let chat = Box::builder()
            .orientation(Orientation::Vertical)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::End)
            .hexpand(true)
            .build();
        chat.add_css_class("chat");

        let message_box = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_bottom(10)
            .halign(gtk::Align::Start)
            .build();

        let message_sender_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();

        self.user_info.connect_clicked(|_| println!("Hi"));
        message_sender_box.append(&self.user_info);

        self.input.set_hexpand(true);
        self.input.set_width_request(600);
        message_sender_box.append(&self.input);

        let scrolled_window: ScrolledWindow = ScrolledWindow::builder()
            .min_content_height(800)
            .min_content_width(600)
            .margin_top(20)
            .margin_bottom(20)
            .child(&message_box)
            .build();

        scrolled_window.add_css_class("message_box");
        self.send_message = create_send_button(message_box, self.input.clone(), scrolled_window.clone());
        message_sender_box.append(&self.send_message);

        chat.append(&scrolled_window);
        chat.append(&message_sender_box);

        main_box.append(&chat);

        //-------------------------------------

        let conv_info = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_start(12)
        .margin_bottom(12)
        .margin_top(12)
        .margin_end(12)
        .halign(Align::Start)
        .build();

        self.quit_channel.add_css_class("exit_channel");
        conv_info.append(&self.quit_channel);

        conv_info.append(&self.channel_info);

        conv_info.append(&self.func_channel);
        
        main_box.append(&conv_info);

        //-------------------------------------
    
        window.set_child(Some(&main_box));

        window
    }
}

fn create_button(label: &str) -> Button {
    let button = Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|_| println!("Hi"));

    button
}

fn create_separator_sidebar() -> Separator {
    Separator::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(20)
        .margin_bottom(20)
        .hexpand(true)
        .build()
}

fn create_separator() -> Separator {
    Separator::builder().orientation(Orientation::Vertical).build()
}

fn create_send_button(message_box: Box, input: Entry, scrolled_window: ScrolledWindow) -> Button {
    let send_button = create_button("send");
    send_button.add_css_class("send_button");
    
    send_button.connect_clicked(move |_| {
        let input_text = input.text();
        if !entry_is_valid(&input_text) {
            return;
        }

        let message = create_message(&input_text);
        message.add_css_class("message");
        message_box.append(&message);

        let adj = scrolled_window.vadjustment();
        adj.set_upper(adj.upper() + adj.page_size());
        adj.set_value(adj.upper());
        scrolled_window.set_vadjustment(Some(&adj));
    });

    send_button
}

fn create_entry(placeholder: &str) -> Entry {
    Entry::builder().placeholder_text(placeholder).build()
}

fn entry_is_valid(entry_text: &str) -> bool {
    !entry_text.is_empty()
}

fn create_message(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Start)
        .halign(Align::Start)
        .build()
}

fn create_main_box(orientation: Orientation, height: i32, width: i32) -> Box {
    Box::builder()
        .orientation(orientation)
        .halign(gtk::Align::Center)
        .height_request(height)
        .width_request(width)
        .build()
}
