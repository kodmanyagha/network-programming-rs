use rand::{seq::SliceRandom, thread_rng, Rng};

use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];

    loop {
        let bytes_read_cnt = stream.read(&mut buf)?;
        if bytes_read_cnt == 0 {
            return Ok(());
        }

        let sleep_duration =
            Duration::from_secs([2, 3, 4, 5, 6].choose(&mut thread_rng()).unwrap().clone() as u64);

        println!("Sleeping for {:?} seconds before replying", sleep_duration);
        thread::sleep(sleep_duration);
        // println!("bytes_read_cnt: {bytes_read_cnt}");

        for current_byte in buf.iter().take(bytes_read_cnt) {
            let current_char =
                char::from_digit(current_byte.clone() as u32, 10).unwrap_or_default();
            // print!("Incoming char: {current_char}");

            let current_char = current_byte.clone() as char;
            // print!("Incoming current_byte: {current_byte}");
            // print!("Incoming char: {current_char}");
        }
        // println!("");

        stream.write(&buf[..bytes_read_cnt]);
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
