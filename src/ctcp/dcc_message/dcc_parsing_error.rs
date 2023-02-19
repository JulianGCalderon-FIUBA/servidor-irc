#[derive(Debug)]
pub enum DccParsingError {
    EmptyMessage,
    UnknownCommand,
    NoCommand,
    NoFilename,
    NoFilesize,
    NoDcc,
    NoIp,
    NoPort,
    NoProtocol,
    NoPosition,
    InvalidAddress,
    InvalidFilesize,
    InvalidProtocol,
    InvalidPosition,
    InvalidPort,
}
