/// Contains structure for dcc resume sender. A DccResumeSender handles the request to send a dcc resume.
pub mod dcc_resume_sender;
/// Contains structure for dcc send receiver. A DccSendReceiver handles the receiving end of a dcc send request.
pub mod dcc_send_receiver;
/// Contains structure for dcc send sender. A DccSendSender handles the request to send a dcc send.
pub mod dcc_send_sender;
/// Contains structure for a file transferer. A FileTransferer handles everything related to files: uploading, downloading and resuming an operation.
pub mod file_transfer;
