use common::manager::{Id, Item, Manager};
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    addr: SocketAddr,
}

impl Item<SocketAddr> for User {
    fn new(id: &Id, addr: &SocketAddr) -> Self {
        User {
            id: *id,
            addr: *addr,
        }
    }
}

pub type UserManager = Manager<User, SocketAddr>;
