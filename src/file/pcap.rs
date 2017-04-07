use {get_u16, get_u32, get_i32};

use std;
use std::io::Read;
use std::iter::Iterator;

pub struct PcapIterator {
    pub magic_number: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub this_zone: i32, //gmt to local correction
    pub sig_figs: u32, //accuracy of timestamp
    pub snap_len: u32, //max length of captured packets (in octets)
    pub network: u32, //data link type
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

    fn read_pcap_header(&mut self) -> Result<(u32, u32, u32, u32), std::io::Error> {
        let timestamp_secs = try!(get_u32(&mut self.input));
        let timestamp_usecs = try!(get_u32(&mut self.input));
        let included_length = try!(get_u32(&mut self.input));
        let original_length = try!(get_u32(&mut self.input));
        Ok((timestamp_secs, timestamp_usecs, included_length, original_length))
    }
}

impl Iterator for PcapIterator {
    type Item = PcapPacket;

    fn next(&mut self) -> Option<Self::Item> {
        let (timestamp_secs, timestamp_usecs, included_length, original_length) = match self.read_pcap_header() {
            Ok(tuple) => tuple,
            Err(e) => {
                println!("ERROR reading pcap header: {}", e);
                return None;
            },
        };

        let mut buffer = vec![0; included_length as usize];
        if let Err(e) = self.input.read_exact(&mut buffer) {
            println!("ERROR: {}", e);
            return None;
        }

        Some (
            PcapPacket {
                timestamp_secs: timestamp_secs,
                timestamp_usecs: timestamp_usecs,
                included_length: included_length,
                original_length: original_length,
            }
        )
    }
}

#[derive(Debug)]
pub struct PcapPacket {
    pub timestamp_secs: u32,
    pub timestamp_usecs: u32,
    pub included_length: u32,
    pub original_length: u32,
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

        println!("magic_number: {}", pcap_iter.magic_number);
        println!("version_major: {}", pcap_iter.version_major);
        println!("version_minor: {}", pcap_iter.version_minor);
        println!("this_zone: {}", pcap_iter.this_zone);
        println!("sig_figs: {}", pcap_iter.sig_figs);
        println!("snap_len: {}", pcap_iter.snap_len);
        println!("network: {}", pcap_iter.network);

        for packet in pcap_iter {
            println!("{:?}", packet);
        }
    }
}
