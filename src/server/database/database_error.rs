#[derive(Debug)]
/// Errors that may happen when handling requests.
pub enum DatabaseError {
    NoSuchClient,
    NoSuchChannel,
    NoSuchServer,
    ClientIsOffline,
    CannotCloneStream,
}
