use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        for current_byte in buf.iter() {
            let current_char =
                char::from_digit(current_byte.clone() as u32, 10).unwrap_or_default();
            print!("{current_char}");
        }
        println!("");

        stream.write(&buf[..bytes_read]);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind.");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("Failed: {}", e);
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|e| eprintln!("{:?}", e));
                });
            }
        }
    }
}
