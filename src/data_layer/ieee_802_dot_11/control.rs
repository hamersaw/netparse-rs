use {get_u16};

use std;
use std::io::Read;

#[derive(Debug)]
pub struct CtrlBlockAckRequest {
}

impl CtrlBlockAckRequest {
    pub fn parse(input: Box<Read>) -> Result<CtrlBlockAckRequest, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct CtrlBlockAck {
}

impl CtrlBlockAck {
    pub fn parse(input: Box<Read>) -> Result<CtrlBlockAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct CtrlPowerSavePoll {
}

impl CtrlPowerSavePoll {
    pub fn parse(input: Box<Read>) -> Result<CtrlPowerSavePoll, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct CtrlRequestToSend {
}

impl CtrlRequestToSend  {
    pub fn parse(input: Box<Read>) -> Result<CtrlRequestToSend, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct CtrlClearToSend {
    duration: u16,
    receiver_address: [u8; 6],
}

impl CtrlClearToSend {
    pub fn parse(mut input: Box<Read>) -> Result<CtrlClearToSend, std::io::Error> {
        let duration = try!(get_u16(&mut input));
        let mut receiver_address = [0; 6];
        try!(input.read_exact(&mut receiver_address));

        Ok(
            CtrlClearToSend {
                duration: duration,
                receiver_address: receiver_address,
            }
        )
    }
}

#[derive(Debug)]
pub struct CtrlAck {
}

impl CtrlAck {
    pub fn parse(input: Box<Read>) -> Result<CtrlAck, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct CtrlCfEnd {
}

impl CtrlCfEnd {
    pub fn parse(input: Box<Read>) -> Result<CtrlCfEnd, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct CtrlCfEndPlusCfAck {
}

impl CtrlCfEndPlusCfAck {
    pub fn parse(input: Box<Read>) -> Result<CtrlCfEndPlusCfAck, std::io::Error> {
        unimplemented!();
    }
}
