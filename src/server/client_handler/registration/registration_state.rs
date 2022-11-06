#[derive(PartialEq, Eq, Debug)]
/// Possible states for Registration.
pub enum RegistrationState {
    NotInitialized,
    NicknameSent,
    Registered,
}
