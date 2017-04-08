use std;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct Data {
}

impl Data {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<Data, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfAck {
}

impl DataPlusCfAck {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataPlusCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfPoll {
}

impl DataPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfAckPlusCfPoll {
}

impl DataPlusCfAckPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataPlusCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataNull {
}

impl DataNull {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataNull, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfAck {
}

impl DataCfAck {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfPoll {
}

impl DataCfPoll {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfAckPlusCfPoll {
}

impl DataCfAckPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosData {
}

impl DataQosData {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataQosData, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfAck {
}

impl DataQosDataPlusCfAck {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataQosDataPlusCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfPoll {
}

impl DataQosDataPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataQosDataPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfAckPlusCfPoll {
}

impl DataQosDataPlusCfAckPlusCfPoll {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataQosDataPlusCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosNull {
}

impl DataQosNull {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataQosNull, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosPlusCfPollNoData {
}

impl DataQosPlusCfPollNoData {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataQosPlusCfPollNoData, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosPlusCfAckNoData {
}

impl DataQosPlusCfAckNoData {
    pub fn parse(cursor: &mut Cursor<&Vec<u8>>) -> Result<DataQosPlusCfAckNoData, std::io::Error> {
        unimplemented!();
    }
}
