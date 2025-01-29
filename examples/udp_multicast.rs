use std::{
    env,
    net::{Ipv4Addr, UdpSocket},
};

use network_programming_rs::MTU_LEN;

fn main() {
    // Why 239? Because multicast address first four bits has to be 1110.
    let mcast_group: Ipv4Addr = "239.0.0.1".parse().unwrap();
    let port: u16 = 6000;
    let any: Ipv4Addr = "0.0.0.0".parse().unwrap();
    let mut buf = [0u8; MTU_LEN];

    if env::args().count() > 1 {
        let socket = UdpSocket::bind((any, port)).unwrap();
        socket.join_multicast_v4(&mcast_group, &any).unwrap();
        socket.recv_from(&mut buf).unwrap();
        print!("{}", std::str::from_utf8(&buf).unwrap());
    } else {
        let socket = UdpSocket::bind((any, 0)).unwrap();
        socket
            .send_to("hello world!!!".as_bytes(), &(mcast_group, port))
            .unwrap();
    }
}
