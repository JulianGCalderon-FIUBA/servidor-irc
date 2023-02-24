use std::{cell::RefCell, collections::HashMap, fs, io, net::SocketAddr, thread};

use gtk4::{
    glib::Sender,
    prelude::*,
    traits::{DialogExt, FileChooserExt, GtkWindowExt},
    ApplicationWindow, ButtonsType, FileChooserDialog, MessageDialog, MessageType, ResponseType,
};

use crate::{
    controller::{
        controller_message::ControllerMessage, failed_transfer::Transfer, NAMES_ERROR_TEXT,
        OPEN_WARNING_ERROR_TEXT,
    },
    ctcp::{dcc_message::DccMessage, dcc_send::dcc_send_receiver::DccSendReceiver},
    message::Message,
    server::consts::commands::NAMES_COMMAND,
};

use super::{names_message_intention::NamesMessageIntention, InterfaceController};

use crate::controller::controller_message::ControllerMessage::OpenWarningView;

const CHANNEL_FIRST_CHARACTER: &str = "#";

impl InterfaceController {
    /// Returns all clients to add.
    ///
    /// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
    pub fn all_clients_except_me(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let mut all_clients = all_clients(channels_and_clients);
        self.remove_myself(&mut all_clients);
        all_clients
    }

    /// Returns clients that are not from the current user.
    ///
    /// Receives a Vec<String> and a Vec<String>, returns a Vec<String>
    pub fn clients_not_mine(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let all_clients = self.all_clients_except_me(channels_and_clients);
        let my_clients = self.main_view.get_my_clients();
        let mut not_mine: Vec<String> = vec![];

        for element in &all_clients {
            let element_without_operator_indicator: String = remove_operator_indicator(element);
            push_if_absent(
                &my_clients,
                &mut not_mine,
                element_without_operator_indicator,
            );
        }

        not_mine
    }

    pub fn current_conv_channels(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        client_channels(channels_and_clients, self.current_conv.clone())
    }

    pub fn remove_myself(&mut self, all_clients: &mut Vec<String>) {
        let nickname = self.nickname.clone();
        remove_element(all_clients, &nickname);
        remove_element(all_clients, &format!("@{nickname}"));
    }

    pub fn send_names_message(
        &mut self,
        intention: NamesMessageIntention,
        parameter: Option<String>,
    ) {
        self.names_message_intention = intention;
        let parameter_value = parameter.unwrap_or_default();
        let message = format!("{NAMES_COMMAND} {}", parameter_value);
        self.client.send(&message).expect(NAMES_ERROR_TEXT);
    }

    pub fn send_open_warning_view(&mut self, warning_text: &str) {
        let to_send = OpenWarningView {
            message: warning_text.to_string(),
        };
        self.sender.send(to_send).expect(OPEN_WARNING_ERROR_TEXT);
    }

    pub fn receive_regular_privmsg(&mut self, message: Message) {
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

    pub fn receive_dcc_message(&mut self, sender: String, dcc_message: DccMessage) {
        match dcc_message {
            DccMessage::Send {
                filename,
                address,
                filesize,
            } => {
                self.receive_dcc_send(sender, filename, address, filesize);
            }
            DccMessage::SendAccept => {
                self.receive_dcc_send_accept(sender);
            }
            DccMessage::SendDecline => {
                self.receive_dcc_send_decline(sender);
            }
            DccMessage::Chat { address: _address } => todo!(),
            DccMessage::ChatAccept => todo!(),
            DccMessage::ChatDecline => todo!(),
            DccMessage::Close => todo!(),
            DccMessage::Resume {
                filename,
                port,
                position,
            } => self.receive_dcc_resume(sender, filename, port, position),
            DccMessage::Accept {
                filename,
                port,
                position,
            } => self.receive_dcc_accept(sender, filename, port, position),
        }
    }

    pub fn receive_dcc_send_decline(&mut self, sender: String) {
        self.dcc_send_senders.remove(&sender);
    }

    pub fn receive_dcc_send_accept(&mut self, sender: String) {
        let dcc_send_sender = if let Some(dcc_send_sender) = self.dcc_send_senders.remove(&sender) {
            dcc_send_sender
        } else {
            return;
        };

        let (transferer, controller) = dcc_send_sender.accept().unwrap();

        let sender_channel = self.sender.clone();
        let sender_client = sender.clone();
        thread::spawn(move || {
            let result = transferer.upload_file();
            let message = ControllerMessage::SendResult {
                sender: sender_client,
                result,
            };
            sender_channel.send(message).unwrap();
        });

        let cancel_dialog = MessageDialog::builder()
            .title("Upload in progress")
            .buttons(ButtonsType::Cancel)
            .transient_for(&self.main_window)
            .build();

        cancel_dialog.present();

        let controller_cell = RefCell::new(controller);
        cancel_dialog.connect_response(move |_, _| controller_cell.borrow_mut().cancel());

        self.cancel_dialogs.insert(sender, cancel_dialog);
    }

    pub fn receive_dcc_send(
        &mut self,
        sender: String,
        filename: String,
        address: SocketAddr,
        filesize: u64,
    ) {
        let server_stream = self.client.get_stream().unwrap();
        let dcc_send_receiver = DccSendReceiver::new(
            server_stream,
            sender.clone(),
            filename.clone(),
            filesize,
            address,
        );

        let transfer = self
            .downloads
            .iter()
            .position(|transfer| {
                transfer.client == sender && transfer.name == filename && transfer.failed
            })
            .map(|index| self.downloads.remove(index));

        if let Some(transfer) = transfer {
            let path = transfer.path;
            let filesize = fs::metadata(path.clone()).map(|metadata| metadata.len());
            if let Ok(filesize) = filesize {
                let dcc_resume_sender = dcc_send_receiver
                    .resume_send_command(filesize, path)
                    .unwrap();
                self.dcc_resume_senders.insert(sender, dcc_resume_sender);
                return;
            }
        }

        let message_dialog = MessageDialog::builder()
            .message_type(MessageType::Question)
            .transient_for(&self.main_window)
            .text(&format!("{sender} wishes to send you a file: {filename}"))
            .secondary_text("Do you want to download it?")
            .buttons(ButtonsType::YesNo)
            .build();

        message_dialog.present();
        self.connect_download_request_dialog(message_dialog, sender.clone());

        self.dcc_send_receivers.insert(sender, dcc_send_receiver);
    }

    pub fn receive_dcc_resume(
        &mut self,
        sender: String,
        _filename: String,
        _port: u16,
        position: u64,
    ) {
        let dcc_send_sender = if let Some(dcc_send_sender) = self.dcc_send_senders.remove(&sender) {
            dcc_send_sender
        } else {
            return;
        };

        let (transferer, controller) = dcc_send_sender.resume(position).unwrap();

        let sender_channel = self.sender.clone();
        let sender_client = sender.clone();
        thread::spawn(move || {
            let result = transferer.resume_upload_file(position);

            let message = ControllerMessage::SendResult {
                sender: sender_client,
                result,
            };
            sender_channel.send(message).unwrap();
        });

        let cancel_dialog = MessageDialog::builder()
            .title("Upload in progress")
            .buttons(ButtonsType::Cancel)
            .transient_for(&self.main_window)
            .build();

        cancel_dialog.present();

        let controller_cell = RefCell::new(controller);
        cancel_dialog.connect_response(move |_, _| controller_cell.borrow_mut().cancel());

        self.cancel_dialogs.insert(sender, cancel_dialog);
    }

    fn receive_dcc_accept(&mut self, sender: String, _filename: String, _port: u16, position: u64) {
        let dcc_resume_sender =
            if let Some(dcc_resume_sender) = self.dcc_resume_senders.remove(&sender) {
                dcc_resume_sender
            } else {
                return;
            };

        let download = Transfer {
            client: sender.clone(),
            name: dcc_resume_sender.original_name(),
            path: dcc_resume_sender.path(),
            failed: false,
        };
        self.downloads.push(download);

        let name = dcc_resume_sender.original_name();
        let (transferer, controller) = dcc_resume_sender.accept().unwrap();

        let sender_channel = self.sender.clone();
        let sender_client = sender.clone();
        thread::spawn(move || {
            let result = transferer.resume_download_file(position);

            let message = ControllerMessage::ReceiveResult {
                sender: sender_client,
                result,
                name,
            };
            sender_channel.send(message).unwrap();
        });

        let cancel_dialog = MessageDialog::builder()
            .title("Download in progress")
            .buttons(ButtonsType::Cancel)
            .transient_for(&self.main_window)
            .build();

        cancel_dialog.present();

        let controller_cell = RefCell::new(controller);
        cancel_dialog.connect_response(move |_, _| controller_cell.borrow_mut().cancel());

        self.cancel_dialogs.insert(sender, cancel_dialog);
    }

    fn connect_download_request_dialog(&mut self, message_dialog: MessageDialog, sender: String) {
        let channel_sender = self.sender.clone();
        let main_window = self.main_window.clone();
        message_dialog.connect_response(move |message_dialog, response_type| {
            if let ResponseType::Yes = response_type {
                let sender = sender.clone();
                let channel_sender = channel_sender.clone();
                let main_window = main_window.clone();
                build_file_download_chooser_dialog(main_window, sender, channel_sender);
            } else {
                let sender = sender.clone();
                let ignore_file_request = ControllerMessage::IgnoreFile { sender };

                channel_sender.send(ignore_file_request).unwrap();
            }

            message_dialog.destroy();
        });
    }

    pub fn completed_transfer_dialog(&self) {
        self.build_notification_popup("File transfer completed successfully");
    }

    pub fn failed_transfer_dialog(&self) {
        self.build_notification_popup("File transfer was interrupted");
    }

    pub fn transfer_result(&mut self, result: Result<(), io::Error>, sender: String) {
        match result {
            Ok(()) => self.completed_transfer_dialog(),
            Err(error) => {
                if error.kind() != io::ErrorKind::Interrupted {
                    self.failed_transfer_dialog();
                }
            }
        };

        if let Some(dialog) = self.cancel_dialogs.remove(&sender) {
            dialog.destroy();
        }
    }

    fn build_notification_popup(&self, title: &str) {
        let completed_dialog = MessageDialog::builder()
            .title(title)
            .transient_for(&self.main_window)
            .buttons(ButtonsType::Ok)
            .build();

        completed_dialog.present();
        completed_dialog.connect_response(move |completed_dialog, _| completed_dialog.destroy());
    }
}

fn build_file_download_chooser_dialog(
    main_window: ApplicationWindow,
    sender: String,
    channel_sender: Sender<ControllerMessage>,
) {
    let file_chooser_dialog = FileChooserDialog::builder()
        .transient_for(&main_window)
        .action(gtk4::FileChooserAction::Save)
        .build();

    file_chooser_dialog.add_button("Download", ResponseType::Accept);
    file_chooser_dialog.present();

    file_chooser_dialog.connect_response(move |file_chooser_dialog, _| {
        let file = if let Some(file) = file_chooser_dialog.file() {
            file
        } else {
            return;
        };

        let path = if let Some(path) = file.path() {
            path
        } else {
            return;
        };

        let sender = sender.clone();
        let download_file_request = ControllerMessage::DownloadFile { path, sender };

        channel_sender.send(download_file_request).unwrap();

        file_chooser_dialog.destroy();
    });
}

/// Returns all clients.
///
/// Receives a HashMap<String, Vec<String>>, returns a Vec<String>
pub fn all_clients(channels_and_clients: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut clients_set: Vec<String> = vec![];
    for clients_of_channel in channels_and_clients.values() {
        for client in clients_of_channel {
            push_if_absent(&clients_set.clone(), &mut clients_set, client.to_string());
        }
    }
    clients_set
}

/// Returns channels that are not from the current user.
///
/// Receives a Vec<String> and a Vec<String>, returns a Vec<String>
pub fn channels_not_mine(all: Vec<String>, mine: Vec<String>) -> Vec<String> {
    let mut not_mine: Vec<String> = vec![];
    for element in &all {
        push_if_absent(&mine, &mut not_mine, element.to_string());
    }
    not_mine
}

/// Returns all channels from a client.
///
/// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
pub fn client_channels(
    channels_and_clients: HashMap<String, Vec<String>>,
    client: String,
) -> Vec<String> {
    let mut client_channels_set: Vec<String> = vec![];
    for channel in channels_and_clients.keys() {
        let mut clients: Vec<String> = vec![];
        for element in channels_and_clients.get(channel).unwrap() {
            let element_without_operator_indicator: String = remove_operator_indicator(element);
            clients.push(element_without_operator_indicator);
        }
        if clients.contains(&client) {
            client_channels_set.push((&channel).to_string());
        }
    }
    client_channels_set
}

/// Returns a bool indicating if the conversation is a channel or not.
///
/// Receives a String, returns a bool
pub fn is_channel(parameter: String) -> bool {
    parameter.starts_with(CHANNEL_FIRST_CHARACTER)
}

pub fn is_not_empty(vector: &Vec<String>) -> bool {
    !vector.is_empty()
}

pub fn push_if_absent(original_vector: &[String], new_vector: &mut Vec<String>, element: String) {
    if !original_vector.contains(&element) {
        new_vector.push(element);
    }
}

pub fn remove_element(vector: &mut Vec<String>, element: &String) {
    if vector.contains(element) {
        vector.remove(vector.iter().position(|x| x == element).unwrap());
    }
}

pub fn remove_operator_indicator(element: &str) -> String {
    if let Some(stripped) = element.strip_prefix('@') {
        stripped.to_string()
    } else {
        element.to_string()
    }
}
