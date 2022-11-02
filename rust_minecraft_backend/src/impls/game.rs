use crate::objects::{game::Game, account::Account, states::LoginState};

impl Game {
    pub fn new() -> Game {
        Game {
            account: Account {
                identifier: String::new(),
                password: String::new(),
                login_state: LoginState::Prepare
            }
        }
    }

    pub fn from(identifier: String) -> Game {
        let get_password = || String::new();

        Game {
            account: Account {
                identifier,
                password: get_password(),
                login_state: LoginState::Prepare
            }
        }
    }
}