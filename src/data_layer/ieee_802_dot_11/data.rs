use std;
use std::io::Read;

#[derive(Debug)]
pub struct Data {
}

impl Data {
    pub fn parse(input: Box<Read>) -> Result<Data, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfAck {
}

impl DataPlusCfAck {
    pub fn parse(input: Box<Read>) -> Result<DataPlusCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfPoll {
}

impl DataPlusCfPoll {
    pub fn parse(input: Box<Read>) -> Result<DataPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataPlusCfAckPlusCfPoll {
}

impl DataPlusCfAckPlusCfPoll {
    pub fn parse(input: Box<Read>) -> Result<DataPlusCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataNull {
}

impl DataNull {
    pub fn parse(input: Box<Read>) -> Result<DataNull, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfAck {
}

impl DataCfAck {
    pub fn parse(input: Box<Read>) -> Result<DataCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfPoll {
}

impl DataCfPoll {
    pub fn parse(input: Box<Read>) -> Result<DataCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataCfAckPlusCfPoll {
}

impl DataCfAckPlusCfPoll {
    pub fn parse(input: Box<Read>) -> Result<DataCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosData {
}

impl DataQosData {
    pub fn parse(input: Box<Read>) -> Result<DataQosData, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfAck {
}

impl DataQosDataPlusCfAck {
    pub fn parse(input: Box<Read>) -> Result<DataQosDataPlusCfAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfPoll {
}

impl DataQosDataPlusCfPoll {
    pub fn parse(input: Box<Read>) -> Result<DataQosDataPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosDataPlusCfAckPlusCfPoll {
}

impl DataQosDataPlusCfAckPlusCfPoll {
    pub fn parse(input: Box<Read>) -> Result<DataQosDataPlusCfAckPlusCfPoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosNull {
}

impl DataQosNull {
    pub fn parse(input: Box<Read>) -> Result<DataQosNull, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosPlusCfPollNoData {
}

impl DataQosPlusCfPollNoData {
    pub fn parse(input: Box<Read>) -> Result<DataQosPlusCfPollNoData, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DataQosPlusCfAckNoData {
}

impl DataQosPlusCfAckNoData {
    pub fn parse(input: Box<Read>) -> Result<DataQosPlusCfAckNoData, std::io::Error> {
        unimplemented!();
    }
}
