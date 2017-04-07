use std::io::Read;
use std::iter::Iterator;

pub struct PcapIterator {
    magic_number: u32,
    version_major: u16,
    version_minor: u16,
    this_zone: i32, //gmt to local correction
    sig_figs: u32, //accuracy of timestamp
    snap_len: u32, //max length of captured packets (in octets)
    network: u32, //data link type
    input: Box<Read>,
}

impl PcapIterator {
    pub fn new(mut input: Box<Read>) -> Result<PcapIterator, std::io::Error> {
        Ok(
            PcapIterator {
                magic_number: try!(get_u32(&mut input)),
                version_major: try!(get_u16(&mut input)),
                version_minor: try!(get_u16(&mut input)),
                this_zone: try!(get_i32(&mut input)),
                sig_figs: try!(get_u32(&mut input)),
                snap_len: try!(get_u32(&mut input)),
                network: try!(get_u32(&mut input)),
                input: input,
            }
        )
    }
}

impl Iterator for PcapIterator {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug)]
pub struct Packet {

}

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

#[cfg(test)]
mod tests {
    use super::PcapIterator;

    use std::fs::File;

    #[test]
    fn test_wifi() {
        let file = match File::open("examples/802.11-01.cap") {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };

        let mut pcap_iter = match PcapIterator::new(Box::new(file)) {
            Ok(pcap_iter) => pcap_iter,
            Err(e) => panic!("{}", e),
        };

        for packet in pcap_iter {
            println!("found packet: {:?}", packet);
        }
    }
}
