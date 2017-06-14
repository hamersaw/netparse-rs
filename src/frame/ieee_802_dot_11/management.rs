use bytes::{Buf, LittleEndian};

use error::NetparseError;

use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct MgmtAssociationRequest {
}

impl MgmtAssociationRequest {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<MgmtAssociationRequest, NetparseError> {
        Err(NetparseError::unimplemented("MgmtAssociationRequest"))
    }
}

#[derive(Debug)]
pub struct MgmtAssociationResponse {
}

impl MgmtAssociationResponse{
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<MgmtAssociationResponse, NetparseError> {
        Err(NetparseError::unimplemented("MgmtAssociationResponse"))
    }
}

#[derive(Debug)]
pub struct MgmtReassociationRequest {
}

impl MgmtReassociationRequest {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<MgmtReassociationRequest, NetparseError> {
        Err(NetparseError::unimplemented("MgmtReassociationRequest"))
    }
}

#[derive(Debug)]
pub struct MgmtReassociationResponse {
}

impl MgmtReassociationResponse {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<MgmtReassociationResponse, NetparseError> {
        Err(NetparseError::unimplemented("MgmtReassociationResponse"))
    }
}

#[derive(Debug)]
pub struct MgmtProbeRequest {
    duration: u16,
    receiver_address: [u8; 6],
    destination_address: [u8; 6],
    transmitter_address: [u8; 6],
    source_address: [u8; 6],
    bssid: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl MgmtProbeRequest {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>, to_ds: bool, from_ds: bool) -> Result<MgmtProbeRequest, NetparseError> {
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

        //TODO parse tagged parameters

        Ok(
            MgmtProbeRequest {
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
pub struct MgmtProbeResponse {
    duration: u16,
    receiver_address: [u8; 6],
    destination_address: [u8; 6],
    transmitter_address: [u8; 6],
    source_address: [u8; 6],
    bssid: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl MgmtProbeResponse {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>, to_ds: bool, from_ds: bool) -> Result<MgmtProbeResponse, NetparseError> {
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

        //TODO parse fixed and tagged parameters

        Ok(
            MgmtProbeResponse {
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
pub struct MgmtBeacon {
    pub duration: u16,
    pub receiver_address: [u8; 6],
    pub destination_address: [u8; 6],
    pub transmitter_address: [u8; 6],
    pub source_address: [u8; 6],
    pub bssid: [u8; 6],
    pub sequence_number: u16,
    pub frame_number: u8,
}

impl MgmtBeacon {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>, to_ds: bool, from_ds: bool) -> Result<MgmtBeacon, NetparseError> {
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
        
        //TODO parse fixed and tagged parameters

        Ok(
            MgmtBeacon {
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
pub struct MgmtAtim {
}

impl MgmtAtim {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<MgmtAtim, NetparseError> {
        Err(NetparseError::unimplemented("MgmtAtim"))
    }
}

#[derive(Debug)]
pub struct MgmtDisassociation {
}

impl MgmtDisassociation {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<MgmtDisassociation, NetparseError> {
        Err(NetparseError::unimplemented("MgmtDissassociation"))
    }
}

#[derive(Debug)]
pub struct MgmtAuthentication {
}

impl MgmtAuthentication {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<MgmtAuthentication, NetparseError> {
        Err(NetparseError::unimplemented("MgmtAuthentication"))
    }
}

#[derive(Debug)]
pub struct MgmtDeauthentication {
    duration: u16,
    receiver_address: [u8; 6],
    destination_address: [u8; 6],
    transmitter_address: [u8; 6],
    source_address: [u8; 6],
    bssid: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl MgmtDeauthentication {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>, to_ds: bool, from_ds: bool) -> Result<MgmtDeauthentication, NetparseError> {
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
        
        //TODO parse fixed parameters

        Ok(
            MgmtDeauthentication {
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
pub struct MgmtAction {
}

impl MgmtAction {
    pub fn parse(_: &mut Cursor<Vec<u8>>) -> Result<MgmtAction, NetparseError> {
        Err(NetparseError::unimplemented("MgmtAction"))
    }
}
