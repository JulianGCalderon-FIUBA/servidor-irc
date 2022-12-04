#[derive(Debug)]
pub enum DatabaseError {
    NoSuchClient,
    NoSuchChannel,
    NoSuchServer,
    ClientIsOffline,
    CannotCloneStream,
}
