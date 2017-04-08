extern crate bytes;

use bytes::{BigEndian, Buf, LittleEndian};

mod data_layer;
mod input;

pub use input::pcap::{PcapIterator, PcapPacket};

use std::iter::Iterator;

#[cfg(test)]
mod tests {
    use data_layer::DataLayerType;
    use data_layer::ieee_802_dot_11::IEEE802Dot11FrameType;
    use input::pcap::PcapIterator;

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

        for mut packet in pcap_iter {
            //println!("PCAP: {:?}", packet);
            match packet.parse() {
                Ok(DataLayerType::IEEE802Dot11(mut frame)) => {
                    //println!("802.11: {:?}", frame);

                    match frame.parse() {
                        Ok(IEEE802Dot11FrameType::CtrlBlockAck(message)) => println!("CTRL_BLOCK_ACK: {:?}", message),
                        Ok(IEEE802Dot11FrameType::CtrlClearToSend(message)) => println!("CTRL_CLEAR_TO_SEND: {:?}", message),
                        Ok(IEEE802Dot11FrameType::MgmtBeacon(message)) => println!("MGMT_BEACON: {:?}", message),
                        Ok(IEEE802Dot11FrameType::MgmtDeauthentication(message)) => println!("MGMT_DEAUTHENTICATION: {:?}", message),
                        _ => {},
                    }
                },
                Err(e) => panic!("{}", e),
                _ => {},
            }
        }
    }
}
