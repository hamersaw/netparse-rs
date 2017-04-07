mod input;
mod frame;

pub use input::pcap::{PcapIterator, PcapPacket};

use std::io::Read;

fn get_u16(input: &mut Box<Read>) -> Result<u16, std::io::Error> {
    let mut buffer = vec![0; 2];
    try!(input.read_exact(&mut buffer));
    Ok((buffer[0] as u16) | ((buffer[1] as u16) << 8))
}

fn get_u32(input: &mut Box<Read>) -> Result<u32, std::io::Error> {
    let mut buffer = vec![0; 4];
    try!(input.read_exact(&mut buffer));
    Ok((buffer[0] as u32) | ((buffer[1] as u32) << 8) | ((buffer[2] as u32) << 16) | ((buffer[3] as u32) << 24))
}

fn get_i32(input: &mut Box<Read>) -> Result<i32, std::io::Error> {
    let mut buffer = vec![0; 4];
    try!(input.read_exact(&mut buffer));
    Ok((buffer[0] as i32) | ((buffer[1] as i32) << 8) | ((buffer[2] as i32) << 16) | ((buffer[3] as i32) << 24))
}
