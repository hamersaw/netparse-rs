use bytes::{Buf, IntoBuf, LittleEndian};

use data_layer::DataLayerType;
use data_layer::ieee_802_dot_11::IEEE802Dot11Frame;

use std;
use std::io::{Cursor, Read};
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
        let mut global_header_bytes = vec![0; 24];
        try!(input.read_exact(&mut global_header_bytes));
        let mut global_header_buf = global_header_bytes.into_buf();

        Ok(
            PcapIterator {
                magic_number: global_header_buf.get_u32::<LittleEndian>(),
                version_major: global_header_buf.get_u16::<LittleEndian>(),
                version_minor: global_header_buf.get_u16::<LittleEndian>(),
                this_zone: global_header_buf.get_i32::<LittleEndian>(),
                sig_figs: global_header_buf.get_u32::<LittleEndian>(),
                snap_len: global_header_buf.get_u32::<LittleEndian>(),
                network: global_header_buf.get_u32::<LittleEndian>(),
                input: input,
            }
        )
    }
}

impl Iterator for PcapIterator {
    type Item = PcapPacket;

    fn next(&mut self) -> Option<Self::Item> {
        //parse pcap header
        let mut pcap_header_bytes = vec![0; 16];
        if let Err(e) = self.input.read_exact(&mut pcap_header_bytes) {
            println!("ERROR pcap header: {}", e);
            return None;
        }

        let mut pcap_header_buf = pcap_header_bytes.into_buf();
        let timestamp_secs = pcap_header_buf.get_u32::<LittleEndian>();
        let timestamp_usecs = pcap_header_buf.get_u32::<LittleEndian>();
        let included_length = pcap_header_buf.get_u32::<LittleEndian>();
        let original_length = pcap_header_buf.get_u32::<LittleEndian>();

        //read in data
        let mut data_bytes = vec![0; included_length as usize];
        if let Err(e) = self.input.read_exact(&mut data_bytes) {
            println!("ERROR pcap data: {}", e);
            return None;
        }

        let mut cursor = Cursor::new(data_bytes);

        //parse frame
        let data_layer_type = match self.network {
            105 => {
                match IEEE802Dot11Frame::parse(&mut cursor) {
                    Ok(frame) => DataLayerType::IEEE802Dot11(frame),
                    Err(e) => {
                        println!("ERROR parsing frame: {}", e);
                        return None;
                    },
                }
            },
            _ => unimplemented!(),
        };

        //return pcap packet
        Some (
            PcapPacket {
                timestamp_secs: timestamp_secs,
                timestamp_usecs: timestamp_usecs,
                included_length: included_length,
                original_length: original_length,
                data_layer_type: data_layer_type,
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
    pub data_layer_type: DataLayerType,
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
