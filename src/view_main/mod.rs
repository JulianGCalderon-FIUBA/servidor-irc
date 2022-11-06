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
                create_sidebar_button("#channel1", 12),
                create_sidebar_button("#channel2", 12)
            ],
            add_channel: create_sidebar_button("+", 12),
            clients: vec![
                create_sidebar_button("juli", 12),
                create_sidebar_button("sol", 12),
                create_sidebar_button("santi", 12),
                create_sidebar_button("ana", 12)
            ],
            add_client: create_sidebar_button("+", 12),
            messages: vec![create_message("hola!")],
            user_info: create_button_chat("info"),
            input: create_entry("Message..."),
            send_message: create_button_chat("send"),
            quit_channel: create_button_conv_info("x"),
            channel_info: create_button_conv_info("info"),
            func_channel: create_button_conv_info("func"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder().application(&app).title("Lemon Pie IRC").build();

        let main_box = create_main_box(Orientation::Horizontal, 800, 1200);
        main_box.add_css_class("main_box");

        let sidebar = Box::builder()
            .orientation(Orientation::Vertical)
            .halign(gtk::Align::Center)
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
            .margin_start(20)
            .child(&message_box)
            .build();

        scrolled_window.add_css_class("message_box");
        self.send_message = create_send_button(message_box, &self.input, scrolled_window.clone());
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

    // fn connect_button(
    //     &self,
    //     window: ApplicationWindow,
    //     realname_entry: Entry,
    //     pass_entry: Entry,
    //     nick_entry: Entry,
    //     username_entry: Entry,
    //     sender: Sender<String>
    // ) {
    //     self.login_button.connect_clicked(move |_| {
    //         if
    //             realname_entry.text().len() != 0 &&
    //             !!pass_entry.text().len() != 0 &&
    //             !!nick_entry.text().len() != 0 &&
    //             !!username_entry.text().len() != 0
    //         {
    //             // let pass_command = format!("PASS {}", pass_entry.text());
    //             // let nick_command = format!("NICK {}", nick_entry.text());
    //             // let user_command = format!(
    //             //     "USER {} {} {} :{}",
    //             //     username_entry.text(),
    //             //     username_entry.text(),
    //             //     username_entry.text(),
    //             //     username_entry.text()
    //             // );
    //             // sender.send(pass_command).expect("Error: pass command");
    //             // sender.send(nick_command).expect("Error: nick command");
    //             // sender.send(user_command).expect("Error: user command");

    //             // window.close();

    //             sender.send("register".to_string()).expect("Error: pass command");
    //             sender.send("change".to_string()).expect("Error: pass command");
    //         }
    //     });
    // }

    // pub fn get_pass(&self) -> Entry {
    //     self.pass_entry.clone()
    // }
}

fn create_sidebar_button(label: &str, margin: i32) -> Button {
    let button = Button::builder()
        .label(label)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .valign(gtk::Align::Center)
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

fn create_button_chat(label: &str) -> Button {
    Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build()
}

fn create_send_button(message_box: Box, input: &Entry, scrolled_window: ScrolledWindow) -> Button {
    let send_button = create_button_chat("send");
    send_button.add_css_class("send_button");
    let input_text = input.text();

    send_button.connect_clicked(move |_| {
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

fn create_button_conv_info(label: &str) -> Button {
    let button = Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Center)
        .build();

    button.connect_clicked(|_| println!("Hi"));

    button
}

fn create_main_box(orientation: Orientation, height: i32, width: i32) -> Box {
    Box::builder()
        .orientation(orientation)
        .halign(gtk::Align::Center)
        .height_request(height)
        .width_request(width)
        .build()
}

fn build_main_app(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("Lemon Pie IRC").build();

    let main_box = create_main_box(Orientation::Horizontal, 800, 1200);
    main_box.add_css_class("main_box");

    let sidebar = Sidebar::new();
    sidebar.add_css_class("sidebar");
    main_box.append(&sidebar);

    let separator = create_separator();
    main_box.add_css_class("separator");
    main_box.append(&separator);

    let chat = Chat::new();
    chat.add_css_class("chat");
    main_box.append(&chat);

    // let separator = create_separator();
    // main_box.append(&separator);

    let conv_info = ConvInfo::new();
    main_box.append(&conv_info);

    window.set_child(Some(&main_box));
    window.show();
}

