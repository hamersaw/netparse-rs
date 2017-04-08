extern crate bytes;

use bytes::{BigEndian, Buf, LittleEndian};

mod data_layer;
mod input;

pub use input::pcap::{PcapIterator, PcapPacket};
