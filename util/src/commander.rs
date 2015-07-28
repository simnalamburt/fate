extern crate common;

use common::message::*;
use std::env;
use std::io::Result as IoResult;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::net::UdpSocket;

fn main() {
    let mut args = env::args();
    let _program_name = args.next();
    let ip = args.next().unwrap();
    let port = args.next().unwrap()
        .parse::<u16>().unwrap();

    let addr = ("0.0.0.0", 7654);
    let target = (&ip[..], port);

    let socket = UdpSocket::bind(&addr)
        .unwrap_or_else(|e| {
            panic!("couldn't bind socket: {}", e);
        });

    let mut buf = [0u8; 1024];
    loop {
        let command = command_to_send().unwrap();
        let _ = socket.send_to(command.stringify().unwrap().as_bytes(), &target);
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let buf = &buf[..amt];
                let msg = String::from_utf8_lossy(buf);
                let msg = msg[..].trim_right();
                println!("Received: \x1b[33m\"{}\"\x1b[0m", msg);

                let result = Message::parse(&msg.to_string()).map_err(|err| {
                    format!("{:?} when parsing \"{}\"", err, msg)
                }).map(|command: ServerToClient| {
                    format!("{:?}", command)
                });

                match result {
                    Ok(result) => {
                        println!("{}", result);
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            Err(e) => {
                println!("couldn't receive a datagram: {}", e);
            }
        }
    }
}

fn command_to_send() -> IoResult<ClientToServer> {
    println!("=============");
    println!("1: ConnectRequest");
    println!("2: CreateGameRequest");
    loop {
        let mut line = String::new();
        let _len = try!(stdin().read_line(&mut line));
        let line = line.trim();
        match line {
            "1" => {
                return Ok(ClientToServer::ConnectRequest)
            }
            "2" => {
                print!("Enter user id: ");
                let _ = stdout().flush();
                loop {
                    let mut line = String::new();
                    let _len = try!(stdin().read_line(&mut line));
                    let line = line.trim();
                    match line.parse::<usize>() {
                        Ok(user_id) => {
                            return Ok(ClientToServer::CreateGameRequest { user_id: user_id })
                        }
                        Err(_) => {
                            println!("Invalid user id.");
                        }
                    }
                }
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}
