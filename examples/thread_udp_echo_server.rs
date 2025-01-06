use std::{net::UdpSocket, thread};

const MTU_LEN: u32 = 1500;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("Could not bind socket");

    loop {
        let mut buf = [0u8, MTU_LEN as u8];
        let sock = socket.try_clone().expect("Filed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src);
                    sock.send_to(&buf, &src).expect("Failed to send a response");
                });
            }
            Err(e) => {
                eprintln!("Could not receive a datagra: {}", e);
            }
        }
    }
}
