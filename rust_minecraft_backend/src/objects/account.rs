use super::states::LoginState;

pub struct Account {
    pub identifier: String,
    pub password: String,
    pub login_state: LoginState
}