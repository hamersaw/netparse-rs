use {get_u16};

use std;
use std::io::Read;

#[derive(Debug)]
pub struct IEEE802Dot11Frame {
    pub protocol_version: u8,
    pub packet_type: u8,
    pub packet_subtype: u8,
    pub to_ds: bool,
    pub from_ds: bool,
    pub more_fragements: bool,
    pub retry: bool,
    pub power_management: bool,
    pub more_data: bool,
    pub wep: bool,
    pub order: bool,
    pub bssid: Option<[u8; 6]>,
    pub source_address: Option<[u8; 6]>,
    pub destination_address: Option<[u8; 6]>,
    pub transmitter_address: Option<[u8; 6]>,
    pub receiver_address: Option<[u8; 6]>,
    pub sequence_number: Option<u16>,
    pub fragement_number: Option<u8>,
}

impl IEEE802Dot11Frame {
    pub fn parse(mut input: Box<Read>) -> Result<IEEE802Dot11Frame, std::io::Error> {
        let mut frame_control_buffer = vec![0; 2];
        try!(input.read_exact(&mut frame_control_buffer));
        let protocol_version = frame_control_buffer[0] & 3u8;
        let packet_type = (frame_control_buffer[0] & 12u8) >> 2;
        let packet_subtype = (frame_control_buffer[0] & 240u8) >> 4;
        let to_ds = (frame_control_buffer[1] & 1) == 1;
        let from_ds = (frame_control_buffer[1] & 2) == 2;
        let more_fragements = (frame_control_buffer[1] & 4) == 4;
        let retry = (frame_control_buffer[1] & 8) == 8;
        let power_management = (frame_control_buffer[1] & 16) == 16;
        let more_data = (frame_control_buffer[1] & 32) == 32;
        let wep = (frame_control_buffer[1] & 64) == 64;
        let order = (frame_control_buffer[1] & 128) == 128;

        /*match (packet_type, packet_subtype) {
            (0, 0) => { //management association request
                unimplemented!();
            },
            (0, 1) => { //management association response
                unimplemented!();
            },
            (0, 2) => { //management reassociation request
                unimplemented!();
            },
            (0, 3) => { //management reassociation response
                unimplemented!();
            },
            (0, 4) => { //management probe request
                unimplemented!();
            },
            (0, 5) => { //management probe response
                unimplemented!();
            },
            (0, 8) => { //management beacon
                unimplemented!();
            },
            (0, 9) => { //management atim
                unimplemented!();
            },
            (0, 10) => { //management disassociation
                unimplemented!();
            },
            (0, 11) => { //management authentication
                unimplemented!();
            },
            (0, 12) => { //management deauthentication
                unimplemented!();
            },
            (0, 13) => { //management action
                unimplemented!();
            },
            (1, 8) => { //control block ack request
                unimplemented!();
            },
            (1, 9) => { //control block ack
                unimplemented!();
            },
            (1, 10) => { //control ps poll
                unimplemented!();
            },
            (1, 11) => { //control rts
                unimplemented!();
            },
            (1, 12) => { //control cts
                unimplemented!();
            },
            (1, 13) => { //control ack
                unimplemented!();
            },
            (1, 14) => { //control cf end
                unimplemented!();
            },
            (1, 15) => { //control cf end + cf ack
                unimplemented!();
            },
            (2, 0) => { //data
                unimplemented!();
            },
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "malformed frame")),
        }*/

        Ok(
            IEEE802Dot11Frame {
                protocol_version: protocol_version,
                packet_type: packet_type,
                packet_subtype: packet_subtype,
                to_ds: to_ds,
                from_ds: from_ds,
                more_fragements: more_fragements,
                retry: retry,
                power_management: power_management,
                more_data: more_data,
                wep: wep,
                order: order,
                bssid: None,
                source_address: None,
                destination_address: None,
                transmitter_address: None,
                receiver_address: None,
                sequence_number: None,
                fragement_number: None,
            }
        )
    }
}
