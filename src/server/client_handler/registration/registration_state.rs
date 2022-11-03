#[derive(PartialEq, Eq, Debug)]
pub enum RegistrationState {
    NotInitialized,
    NicknameSent,
    Registered,
}
