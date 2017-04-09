use bytes::{Buf, LittleEndian};

use std;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct Data {
}

impl Data {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Data, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfAck {
}

impl DataPlusCfAck {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataPlusCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfPoll {
}

impl DataPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfAckPlusCfPoll {
}

impl DataPlusCfAckPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataPlusCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataNull {
}

impl DataNull {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataNull, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfAck {
}

impl DataCfAck {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfPoll {
}

impl DataCfPoll {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfAckPlusCfPoll {
}

impl DataCfAckPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosData {
    duration: u16,
    destination_address: [u8; 6],
    source_address: [u8; 6],
    bss_id: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl DataQosData {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataQosData, std::io::Error> {
        let duration = cursor.get_u16::<LittleEndian>();

        //parse addresses
        let mut destination_address = [0; 6];
        try!(cursor.read_exact(&mut destination_address));
        let mut source_address = [0; 6];
        try!(cursor.read_exact(&mut source_address));
        let mut bss_id = [0; 6];
        try!(cursor.read_exact(&mut bss_id));

        //parse sequence 
        let sequence_control = cursor.get_u16::<LittleEndian>();
        let sequence_number = (sequence_control & 65520u16) >> 4;
        let frame_number = (sequence_control & 15u16) as u8;
        
        //TODO parse management fields

        Ok(
            DataQosData {
                duration: duration,
                destination_address: destination_address,
                source_address: source_address,
                bss_id: bss_id,
                sequence_number: sequence_number,
                frame_number: frame_number,
            }
        )
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfAck {
}

impl DataQosDataPlusCfAck {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataQosDataPlusCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfPoll {
}

impl DataQosDataPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataQosDataPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfAckPlusCfPoll {
}

impl DataQosDataPlusCfAckPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataQosDataPlusCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosNull {
}

impl DataQosNull {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataQosNull, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosPlusCfPollNoData {
}

impl DataQosPlusCfPollNoData {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataQosPlusCfPollNoData, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosPlusCfAckNoData {
}

impl DataQosPlusCfAckNoData {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<DataQosPlusCfAckNoData, std::io::Error> {
        unimplemented!();
    }
}