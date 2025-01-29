use network_programming_rs::MTU_LEN;
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpStream, UdpSocket},
    str,
    time::Duration,
};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:9090").unwrap();

    socket.connect("127.0.0.1:8080").unwrap();

    loop {
        let mut input = String::new();
        let mut buf = [0u8; MTU_LEN];

        io::stdin().read_line(&mut input).unwrap();
        socket.send(input.as_bytes()).unwrap();
        socket.recv_from(&mut buf).unwrap();

        let received_str = std::str::from_utf8(&buf).unwrap();

        println!("Received: {}", received_str);
    }
}
