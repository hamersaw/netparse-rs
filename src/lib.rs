extern crate bytes;

pub mod error;
pub mod frame;
//mod input;

//pub use input::pcap::{PcapIterator, PcapBlock};
pub use error::NetparseError;

#[cfg(test)]
mod tests {
    use input::pcap::PcapIterator;

    use std::fs::File;

    #[test]
    fn test_wifi() {
        let file = match File::open("examples/802.11-02.cap") {
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

        loop {
            let pcap_block = match pcap_iter.next() {
                Ok(Some(pcap_block)) => pcap_block,
                Ok(None) => break,
                Err(e) => panic!("{}", e),
            };

            println!("{:?}", pcap_block);
        }
    }
}
