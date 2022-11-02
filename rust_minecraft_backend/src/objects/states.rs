pub struct BlockState {}

pub enum LoginState {
    WrongIdentifier(String),
    WrongPassword,
    Success,
    Error(String),
    Requesting,
    Requested,
    Prepare
}