use bytes::{Buf, LittleEndian};

use error::NetparseError;

use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct Data {
    duration: u16,
    receiver_address: [u8; 6],
    destination_address: [u8; 6],
    transmitter_address: [u8; 6],
    source_address: [u8; 6],
    bssid: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl Data {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>, to_ds: bool, from_ds: bool) -> Result<Data, NetparseError> {
        let duration = cursor.get_u16::<LittleEndian>();

        //parse addresses
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));
        let mut transmitter_address = [0; 6];
        try!(cursor.read_exact(&mut transmitter_address));

        let (source_address, destination_address, bssid) = try!(super::parse_ds_addresses(cursor, 
                to_ds, from_ds, &receiver_address, &transmitter_address));

        //parse sequence 
        let sequence_control = cursor.get_u16::<LittleEndian>();
        let sequence_number = (sequence_control & 65520u16) >> 4;
        let frame_number = (sequence_control & 15u16) as u8;
        
        //TODO parse ccmp parameters, and data

        Ok(
            Data {
                duration: duration,
                receiver_address: receiver_address,
                destination_address: destination_address,
                transmitter_address: transmitter_address,
                source_address: source_address,
                bssid: bssid,
                sequence_number: sequence_number,
                frame_number: frame_number,
            }
        )
    }
}

#[derive(Debug)]
pub struct DataPlusCfAck {
}

impl DataPlusCfAck {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataPlusCfAck, NetparseError> {
        Err(NetparseError::unimplemented("DataPlusCfAck"))
    }
}

#[derive(Debug)]
pub struct DataPlusCfPoll {
}

impl DataPlusCfPoll {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataPlusCfPoll, NetparseError> {
        Err(NetparseError::unimplemented("DataPlusCfPoll"))
    }
}

#[derive(Debug)]
pub struct DataPlusCfAckPlusCfPoll {
}

impl DataPlusCfAckPlusCfPoll {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataPlusCfAckPlusCfPoll, NetparseError> {
        Err(NetparseError::unimplemented("DataPlusCfAckPlusCfPoll"))
    }
}

#[derive(Debug)]
pub struct DataNull {
    duration: u16,
    receiver_address: [u8; 6],
    destination_address: [u8; 6],
    transmitter_address: [u8; 6],
    source_address: [u8; 6],
    bssid: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl DataNull {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>, to_ds: bool, from_ds: bool) -> Result<DataNull, NetparseError> {
        let duration = cursor.get_u16::<LittleEndian>();

        //parse addresses
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));
        let mut transmitter_address = [0; 6];
        try!(cursor.read_exact(&mut transmitter_address));

        let (source_address, destination_address, bssid) = try!(super::parse_ds_addresses(cursor, 
                to_ds, from_ds, &receiver_address, &transmitter_address));

        //parse sequence 
        let sequence_control = cursor.get_u16::<LittleEndian>();
        let sequence_number = (sequence_control & 65520u16) >> 4;
        let frame_number = (sequence_control & 15u16) as u8;
        
        //TODO parse STA

        Ok(
            DataNull {
                duration: duration,
                receiver_address: receiver_address,
                destination_address: destination_address,
                transmitter_address: transmitter_address,
                source_address: source_address,
                bssid: bssid,
                sequence_number: sequence_number,
                frame_number: frame_number,
            }
        )
    }
}

#[derive(Debug)]
pub struct DataCfAck {
}

impl DataCfAck {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataCfAck, NetparseError> {
        Err(NetparseError::unimplemented("DataCfAck"))
    }
}

#[derive(Debug)]
pub struct DataCfPoll {
}

impl DataCfPoll {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataCfPoll, NetparseError> {
        Err(NetparseError::unimplemented("DataCfPoll"))
    }
}

#[derive(Debug)]
pub struct DataCfAckPlusCfPoll {
}

impl DataCfAckPlusCfPoll {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataCfAckPlusCfPoll, NetparseError> {
        Err(NetparseError::unimplemented("DataCtAckPlusCfPoll"))
    }
}

#[derive(Debug)]
pub struct DataQosData {
    duration: u16,
    receiver_address: [u8; 6],
    destination_address: [u8; 6],
    transmitter_address: [u8; 6],
    source_address: [u8; 6],
    bssid: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl DataQosData {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>, to_ds: bool, from_ds: bool) -> Result<DataQosData, NetparseError> {
        let duration = cursor.get_u16::<LittleEndian>();

        //parse addresses
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));
        let mut transmitter_address = [0; 6];
        try!(cursor.read_exact(&mut transmitter_address));

        let (source_address, destination_address, bssid) = try!(super::parse_ds_addresses(cursor, 
                to_ds, from_ds, &receiver_address, &transmitter_address));

        //parse sequence 
        let sequence_control = cursor.get_u16::<LittleEndian>();
        let sequence_number = (sequence_control & 65520u16) >> 4;
        let frame_number = (sequence_control & 15u16) as u8;
        
        //TODO parse qos control, ccmp parameters, and data

        Ok(
            DataQosData {
                duration: duration,
                receiver_address: receiver_address,
                destination_address: destination_address,
                transmitter_address: transmitter_address,
                source_address: source_address,
                bssid: bssid,
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
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataQosDataPlusCfAck, NetparseError> {
        Err(NetparseError::unimplemented("DataQosDataPlusCfAck"))
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfPoll {
}

impl DataQosDataPlusCfPoll {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataQosDataPlusCfPoll, NetparseError> {
        Err(NetparseError::unimplemented("DataQosDataPlusCfPoll"))
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfAckPlusCfPoll {
}

impl DataQosDataPlusCfAckPlusCfPoll {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataQosDataPlusCfAckPlusCfPoll, NetparseError> {
        Err(NetparseError::unimplemented("DataQosDataPlusCfAckPlusCfPoll"))
    }
}

#[derive(Debug)]
pub struct DataQosNull {
    duration: u16,
    receiver_address: [u8; 6],
    destination_address: [u8; 6],
    transmitter_address: [u8; 6],
    source_address: [u8; 6],
    bssid: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl DataQosNull {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>, to_ds: bool, from_ds: bool) -> Result<DataQosNull, NetparseError> {
        let duration = cursor.get_u16::<LittleEndian>();

        //parse addresses
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));
        let mut transmitter_address = [0; 6];
        try!(cursor.read_exact(&mut transmitter_address));

        let (source_address, destination_address, bssid) = try!(super::parse_ds_addresses(cursor, 
                to_ds, from_ds, &receiver_address, &transmitter_address));

        //parse sequence 
        let sequence_control = cursor.get_u16::<LittleEndian>();
        let sequence_number = (sequence_control & 65520u16) >> 4;
        let frame_number = (sequence_control & 15u16) as u8;
        
        //TODO parse qos control

        Ok(
            DataQosNull {
                duration: duration,
                receiver_address: receiver_address,
                destination_address: destination_address,
                transmitter_address: transmitter_address,
                source_address: source_address,
                bssid: bssid,
                sequence_number: sequence_number,
                frame_number: frame_number,
            }
        )
    }
}

#[derive(Debug)]
pub struct DataQosPlusCfPollNoData {
}

impl DataQosPlusCfPollNoData {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataQosPlusCfPollNoData, NetparseError> {
        Err(NetparseError::unimplemented("DataQosPlusCfPollNoData"))
    }
}

#[derive(Debug)]
pub struct DataQosPlusCfAckNoData {
}

impl DataQosPlusCfAckNoData {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<DataQosPlusCfAckNoData, NetparseError> {
        Err(NetparseError::unimplemented("DataQosPlusCfAckNoData"))
    }
}
