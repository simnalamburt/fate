use user::User;

use manager::*;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: Id,
    host: User,
    guest: Option<User>,
}

impl Item<User> for Game {
    fn new(id: &Id, host: &User) -> Self {
        Game {
            id: *id,
            host: host.clone(),
            guest: None,
        }
    }
}

pub type GameManager = Manager<Game, User>;
