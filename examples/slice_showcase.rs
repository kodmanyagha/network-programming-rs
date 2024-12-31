use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};

fn slice_showcase_1() {
    let mut buf = [0; 512];
    // let x = &buf[2..5];
    let mut x_mut_1: &mut [u8] = &mut buf[2..5];
    // drop(x);
    // let mut tmp = Vec::new();
    // tmp.extend_from_slice(x);
    // let mut x_mut_2: &mut [u8] = tmp.as_mut_slice();
    // x_mut_2[0] = 1;

    // x_mut_1[0] = 1;
    // x_mut_1[1] = 2;
    // x_mut_1[2] = 3;
}

fn main() {
    slice_showcase_1();
}
