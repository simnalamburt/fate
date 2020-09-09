extern crate common;
#[macro_use]
extern crate log;

use common::message::*;
use common::simple_logger;
use std::net::SocketAddr;
use std::net::UdpSocket;

mod game;
mod user;

use game::GameManager;
use user::UserManager;

type CommandResult = Result<ServerToClient, String>;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let _ = simple_logger::init();

    let addr = ("0.0.0.0", 4567);
    let socket = match UdpSocket::bind(&addr) {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };

    info!("");
    info!("Running \x1b[36m{}\x1b[0m server", common::PROJECT_NAME);
    info!(
        "Start listening on \x1b[33m{}:{}\x1b[0m ...",
        addr.0, addr.1
    );
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
                let buf = &buf[..amt];
                let msg = String::from_utf8_lossy(buf);
                let msg = msg[..].trim_end();
                info!("Received: \x1b[33m\"{}\"\x1b[0m", msg);

                let result = Message::parse(&msg.to_string())
                    .map_err(|err| format!("{:?} when parsing \"{}\"", err, msg))
                    .and_then(|command| {
                        handle_command(&command, &src, &mut user_manager, &mut game_manager)
                    });

                match result {
                    Ok(response) => {
                        let _ = socket.send_to(response.stringify().unwrap().as_bytes(), &src);
                    }
                    Err(err) => {
                        error!("{}", err);
                    }
                }
            }
            Err(e) => error!("couldn't receive a datagram: {}", e),
        }
    }
}

fn handle_command(
    command: &ClientToServer,
    src: &SocketAddr,
    user_manager: &mut UserManager,
    game_manager: &mut GameManager,
) -> CommandResult {
    match command {
        &ClientToServer::ConnectRequest => {
            let user = user_manager.create(src);
            info!("{:?} created", user);
            Ok(ServerToClient::ConnectResponse { user_id: user.id })
        }
        &ClientToServer::CreateGameRequest { user_id } => user_manager
            .get(user_id)
            .ok_or(format!("user id {} is not exists", user_id))
            .map(|user| {
                let game = game_manager.create(&user);
                info!("{:?} created", game);
                ServerToClient::CreateGameResponse { game_id: game.id }
            }),
    }
}
