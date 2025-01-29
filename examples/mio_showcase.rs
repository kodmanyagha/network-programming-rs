extern crate mio;
use std::{env, fmt::format, net::SocketAddr};

use mio::{net::TcpListener, Events, Interest, Poll, Token};

const SERVER: Token = Token(0);

struct TCPServer {
    address: SocketAddr,
}

impl TCPServer {
    pub fn new(host: &str, port: u32) -> Self {
        let address = format!("{host}:{port}").parse::<SocketAddr>().unwrap();
        TCPServer { address }
    }

    pub fn run(&mut self) {
        let mut server = TcpListener::bind(self.address).expect("Could not bind to port");
        let mut poll = Poll::new().unwrap();
        poll.registry()
            .register(&mut server, SERVER, Interest::READABLE)
            .unwrap();
        let mut events = Events::with_capacity(1024);

        loop {
            poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (_stream, remote) = server.accept().unwrap();
                        println!("Connection from {remote}");
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide only one port number as argument");
        std::process::exit(1);
    }

    let mut server = TCPServer::new("127.0.0.1", args[1].parse::<u32>().unwrap());
    server.run();
}
