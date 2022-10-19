#[derive(PartialEq, Eq)]
pub enum RegistrationState {
    NotInitialized,
    NicknameSent,
    Registered,
}

impl RegistrationState {
    pub fn next(&self) -> Self {
        match self {
            Self::NotInitialized => Self::NicknameSent,
            Self::NicknameSent => Self::Registered,
            Self::Registered => panic!("Already registered"),
        }
    }
}
