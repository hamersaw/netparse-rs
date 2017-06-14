use bytes::{Buf, LittleEndian};

use error::NetparseError;

use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct CtrlBlockAckRequest {
    duration: u16,
    receiver_address: [u8; 6],
    transmitter_address: [u8; 6],
}

impl CtrlBlockAckRequest {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<CtrlBlockAckRequest, NetparseError> {
        let duration = cursor.get_u16::<LittleEndian>();
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));
        let mut transmitter_address = [0; 6];
        try!(cursor.read_exact(&mut transmitter_address));

        //TODO parse block ack request type (and subsequent fields)

        Ok(
            CtrlBlockAckRequest {
                duration: duration,
                receiver_address: receiver_address,
                transmitter_address: transmitter_address,
            }
        )
    }
}

#[derive(Debug)]
pub struct CtrlBlockAck {
    duration: u16,
    receiver_address: [u8; 6],
}

impl CtrlBlockAck {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<CtrlBlockAck, NetparseError> {
        let duration = cursor.get_u16::<LittleEndian>();
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));

        //TODO parse block ack type (and subsequent fields)

        Ok(
            CtrlBlockAck {
                duration: duration,
                receiver_address: receiver_address,
            }
        )
    }
}

#[derive(Debug)]
pub struct CtrlPowerSavePoll {
}

impl CtrlPowerSavePoll {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<CtrlPowerSavePoll, NetparseError> {
        Err(NetparseError::unimplemented("CtrlPowerSavePoll"))
    }
}

#[derive(Debug)]
pub struct CtrlRequestToSend {
    duration: u16,
    receiver_address: [u8; 6],
    transmitter_address: [u8; 6],
}

impl CtrlRequestToSend  {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<CtrlRequestToSend, NetparseError> {
        let duration = cursor.get_u16::<LittleEndian>();
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));
        let mut transmitter_address = [0; 6];
        try!(cursor.read_exact(&mut transmitter_address));

        Ok(
            CtrlRequestToSend {
                duration: duration,
                receiver_address: receiver_address,
                transmitter_address: transmitter_address,
            }
        )
    }
}

#[derive(Debug)]
pub struct CtrlClearToSend {
    id: u8,
    duration: u16,
    receiver_address: [u8; 6],
}

impl CtrlClearToSend {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<CtrlClearToSend, NetparseError> {
        let id_duration = cursor.get_u16::<LittleEndian>();
        let id = ((id_duration & 61440u16) >> 12) as u8;
        let duration = id_duration & 4095u16;
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));

        Ok(
            CtrlClearToSend {
                id: id,
                duration: duration,
                receiver_address: receiver_address,
            }
        )
    }
}

#[derive(Debug)]
pub struct CtrlAck {
    duration: u16,
    receiver_address: [u8; 6],
}

impl CtrlAck {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<CtrlAck, NetparseError> {
        let duration = cursor.get_u16::<LittleEndian>();
        let mut receiver_address = [0; 6];
        try!(cursor.read_exact(&mut receiver_address));

        Ok(
            CtrlAck {
                duration: duration,
                receiver_address: receiver_address,
            }
        )
    }
}

#[derive(Debug)]
pub struct CtrlCfEnd {
}

impl CtrlCfEnd {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<CtrlCfEnd, NetparseError> {
        Err(NetparseError::unimplemented("CtrlCfEnd"))
    }
}

#[derive(Debug)]
pub struct CtrlCfEndPlusCfAck {
}

impl CtrlCfEndPlusCfAck {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<CtrlCfEndPlusCfAck, NetparseError> {
        Err(NetparseError::unimplemented("CtrlCfEndPlusCfAck"))
    }
}
