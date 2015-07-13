use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{ AtomicUsize, Ordering };

pub type Id = usize;

#[derive(Debug, Clone)]
pub struct User {
    id: Id,
    addr: SocketAddr,
}

impl User {
    fn new(id: &Id, addr: &SocketAddr) -> Self {
        User {
            id: *id,
            addr: addr.clone(),
        }
    }
}

pub struct UserManager {
    next_user_id: AtomicUsize,
    users: HashMap<Id, User>,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager {
            next_user_id: AtomicUsize::new(0),
            users: HashMap::new(),
        }
    }

    pub fn create(&mut self, addr: &SocketAddr) -> User {
        let id = self.next_user_id.fetch_add(1, Ordering::Relaxed);
        let user = User::new(&id, addr);
        debug_assert!(!self.users.contains_key(&id));
        self.users.insert(id, user.clone());
        user
    }
}
