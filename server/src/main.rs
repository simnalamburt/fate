extern crate common;
#[macro_use]
extern crate log;

use common::simple_logger;
use std::net::UdpSocket;

mod manager;
mod user;
mod game;

use user::UserManager;
use game::GameManager;

#[allow(dead_code)]
fn main() {
    let _ = simple_logger::init();

    let addr = ("0.0.0.0", 4567);
    let socket = match UdpSocket::bind(&addr) {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };

    info!("");
    info!("Running \x1b[36m{}\x1b[0m server", common::PROJECT_NAME);
    info!("Start listening on \x1b[33m{}:{}\x1b[0m ...", addr.0, addr.1);
    info!("Test it with the command below:");
    info!("");
    info!("    $ \x1b[1;37mnc -u 127.0.0.1 {}\x1b[0m", addr.1);
    info!("");

    let mut buf = [0u8; 1024];
    let mut user_manager = UserManager::new();
    let mut game_manager = GameManager::new();

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                // Send a reply to the socket we received data from
                let buf = &buf[..amt];
                let _ = socket.send_to(buf, &src);

                let msg = String::from_utf8_lossy(buf);
                let msg = msg[..].trim_right();
                info!("Received: \x1b[33m\"{}\"\x1b[0m", msg);
            }
            Err(e) => error!("couldn't receive a datagram: {}", e)
        }
    }
}
