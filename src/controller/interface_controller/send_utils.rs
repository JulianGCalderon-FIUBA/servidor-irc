use std::{cell::RefCell, fs, io, net::SocketAddr, thread};

use gtk4::{
    glib::Sender,
    prelude::FileExt,
    traits::{DialogExt, FileChooserExt, GtkWindowExt},
    ApplicationWindow, ButtonsType, FileChooserDialog, HeaderBar, MessageDialog, MessageType,
    ResponseType,
};

use crate::{
    controller::controller_message::ControllerMessage::{
        self, DownloadFile, IgnoreFile, ReceiveResult, SendResult,
    },
    ctcp::dcc_send::{
        dcc_send_receiver::DccSendReceiver,
        file_transfer::{FileTransferer, TransferController},
    },
    macros::{ok_or_return, some_or_return},
};

use super::{download::Download, InterfaceController};

impl InterfaceController {
    /// Discards [`DccSendSender`]. Called if connection was refused.
    pub fn receive_dcc_send_decline(&mut self, sender: String) {
        self.dcc_send_senders.remove(&sender);
    }

    /// Sends file through stream. Called if connection was accepted.
    pub fn receive_dcc_send_accept(&mut self, sender: String) {
        let dcc_send_sender = some_or_return!(self.dcc_send_senders.remove(&sender));

        let (transferer, controller) = dcc_send_sender.accept().unwrap();

        self.start_file_upload(sender.clone(), transferer);

        self.cancel_transfer_dialog("Upload in progress", sender, controller);
    }

    /// Starts file upload.
    fn start_file_upload(&mut self, sender: String, transferer: FileTransferer) {
        let sender_channel = self.sender.clone();
        thread::spawn(move || {
            let result = transferer.upload_file();
            let message = SendResult { sender, result };
            sender_channel.send(message).unwrap();
        });
    }

    /// Creates a [`DccSendReceiver`] to handle the receiving end of a DCC SEND request.
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

        if let Some((path, filesize)) = self.get_failed_transfer_for(sender.clone(), filename) {
            self.receive_dcc_send_on_failed_transfer(dcc_send_receiver, filesize, path, sender);
        } else {
            self.receive_dcc_send_on_new_transfer(sender, dcc_send_receiver);
        }
    }

    /// Gets failed transfer
    fn get_failed_transfer_for(
        &mut self,
        sender: String,
        filename: String,
    ) -> Option<(std::path::PathBuf, u64)> {
        let transfer = self
            .downloads
            .iter()
            .position(|transfer| {
                transfer.client == sender && transfer.name == filename && transfer.failed
            })
            .map(|index| self.downloads.remove(index));

        let transfer = some_or_return!(transfer, None);
        let path = transfer.path;
        let metadata = ok_or_return!(fs::metadata(path.clone()), None);
        let filesize = metadata.len();

        Some((path, filesize))
    }

    /// Receives dcc send on failed transfer.
    fn receive_dcc_send_on_failed_transfer(
        &mut self,
        dcc_send_receiver: DccSendReceiver,
        filesize: u64,
        path: std::path::PathBuf,
        sender: String,
    ) {
        let dcc_resume_sender = dcc_send_receiver
            .resume_send_command(filesize, path)
            .unwrap();
        self.dcc_resume_senders.insert(sender, dcc_resume_sender);
    }

    /// Receives dcc send on new transfer.
    fn receive_dcc_send_on_new_transfer(
        &mut self,
        sender: String,
        dcc_send_receiver: DccSendReceiver,
    ) {
        let filename = dcc_send_receiver.original_name();
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

    /// Resumes the sending of a previously interrupted file.
    pub fn receive_dcc_resume(
        &mut self,
        sender: String,
        _filename: String,
        _port: u16,
        position: u64,
    ) {
        let dcc_send_sender = some_or_return!(self.dcc_send_senders.remove(&sender));

        let (transferer, controller) = dcc_send_sender.resume(position).unwrap();

        self.start_resume_upload_file(sender.clone(), transferer, position);

        let dialog_message = "Upload in progress";
        self.cancel_transfer_dialog(dialog_message, sender, controller);
    }

    /// Starts the sending of a previously interrupted file.
    fn start_resume_upload_file(
        &mut self,
        sender: String,
        transferer: FileTransferer,
        position: u64,
    ) {
        let sender_channel = self.sender.clone();
        thread::spawn(move || {
            let result = transferer.resume_upload_file(position);

            let message = SendResult { sender, result };
            sender_channel.send(message).unwrap();
        });
    }

    /// Resumes the download of a previously interrupted file.
    pub fn receive_dcc_accept(
        &mut self,
        sender: String,
        _filename: String,
        _port: u16,
        position: u64,
    ) {
        let dcc_resume_sender = some_or_return!(self.dcc_resume_senders.remove(&sender));

        let download = Download {
            client: sender.clone(),
            name: dcc_resume_sender.original_name(),
            path: dcc_resume_sender.path(),
            failed: false,
        };
        self.downloads.push(download);

        let name = dcc_resume_sender.original_name();
        let (transferer, controller) = dcc_resume_sender.accept().unwrap();

        self.start_resume_download_file(sender.clone(), transferer, position, name);

        let dialog_message = "Download in progress";
        self.cancel_transfer_dialog(dialog_message, sender, controller);
    }

    /// Starts the download of a previously interrupted file.
    fn start_resume_download_file(
        &mut self,
        sender: String,
        transferer: FileTransferer,
        position: u64,
        name: String,
    ) {
        let sender_channel = self.sender.clone();
        thread::spawn(move || {
            let result = transferer.resume_download_file(position);

            let message = ReceiveResult {
                sender,
                result,
                name,
            };
            sender_channel.send(message).unwrap();
        });
    }

    /// Creates a [`MessageDialog`] containing the dialog_message received. It contains a button to cancel the file transfer.
    pub fn cancel_transfer_dialog(
        &mut self,
        dialog_message: &str,
        sender_nickname: String,
        controller: TransferController,
    ) {
        let cancel_dialog = MessageDialog::builder()
            .title(dialog_message)
            .buttons(ButtonsType::Cancel)
            .modal(false)
            .build();

        let header = HeaderBar::new();

        cancel_dialog.set_titlebar(Some(&header));

        cancel_dialog.present();

        let controller_cell = RefCell::new(controller);
        cancel_dialog.connect_response(move |_, _| controller_cell.borrow_mut().cancel());

        self.cancel_dialogs.insert(sender_nickname, cancel_dialog);
    }

    /// Creates a [`MessageDialog`] to inform the client the transfer has been completed.
    pub fn completed_transfer_dialog(&self, title: &str) {
        let completed_dialog = MessageDialog::builder()
            .title(title)
            .transient_for(&self.main_window)
            .buttons(ButtonsType::Ok)
            .build();

        completed_dialog.present();
        completed_dialog.connect_response(move |completed_dialog, _| completed_dialog.destroy());
    }

    /// Connects download with a message dialog.
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
                let ignore_file_request = IgnoreFile { sender };

                channel_sender.send(ignore_file_request).unwrap();
            }

            message_dialog.destroy();
        });
    }

    /// Generates button to inform the client whether the transfer was completed successfully or not.
    pub fn transfer_result(&mut self, result: io::Result<()>, sender: String) {
        match result {
            Ok(()) => self.completed_transfer_dialog("File transfer completed successfully"),
            Err(error) => {
                if error.kind() != io::ErrorKind::Interrupted {
                    self.completed_transfer_dialog("File transfer was interrupted");
                }
            }
        };

        if let Some(dialog) = self.cancel_dialogs.remove(&sender) {
            dialog.destroy();
        }
    }

    /// Downloads file in a different thread and sends the result through a channel. This allows error handling.
    pub fn start_download_file(
        &mut self,
        sender: String,
        transferer: FileTransferer,
        name: String,
    ) {
        let sender_channel = self.sender.clone();
        thread::spawn(move || {
            let result = transferer.download_file();
            let message = ReceiveResult {
                sender,
                name,
                result,
            };
            sender_channel.send(message).unwrap();
        });
    }
}

/// Builds the file chooser dialog.
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

    file_chooser_dialog.connect_response(move |file_chooser_dialog, response| {
        if response != ResponseType::Accept {
            return;
        }

        let file = some_or_return!(file_chooser_dialog.file());
        let path = some_or_return!(file.path());

        let sender = sender.clone();
        let download_file_request = DownloadFile { path, sender };

        channel_sender.send(download_file_request).unwrap();

        file_chooser_dialog.destroy();
    });
}

/// Connects the file chooser dialog.
pub fn connect_receiver_file_chooser(
    file_chooser_dialog: FileChooserDialog,
    target: String,
    sender: Sender<ControllerMessage>,
) {
    file_chooser_dialog.connect_response(move |file_chooser_dialog, response| {
        if response != ResponseType::Accept {
            return;
        }

        let file = some_or_return!(file_chooser_dialog.file());

        let path = some_or_return!(file.path());

        let target = target.clone();

        sender
            .send(ControllerMessage::SendDccSend { path, target })
            .unwrap();
        file_chooser_dialog.destroy();
    });
}
