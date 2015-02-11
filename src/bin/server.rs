#![feature(io)]

use std::old_io::net::udp::UdpSocket;
use std::old_io::net::ip::{Ipv4Addr, SocketAddr};

fn main() {
    let addr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 4567 };
    let mut socket = match UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };
    println!("Listening on \x1b[33m{}\x1b[0m", addr);

    let mut buf = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                // Send a reply to the socket we received data from
                let buf = &buf[..amt];
                let _ = socket.send_to(buf, src);

                let msg = String::from_utf8_lossy(buf);
                let msg = msg[].trim_right();
                println!("Received: \x1b[33m\"{}\"\x1b[0m", msg);
            }
            Err(e) => println!("couldn't receive a datagram: {}", e)
        }
    }
}
