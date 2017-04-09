use bytes::{Buf, LittleEndian};

use std;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct MgmtAssociationRequest {
}

impl MgmtAssociationRequest {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtAssociationRequest, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtAssociationResponse {
}

impl MgmtAssociationResponse{
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtAssociationResponse, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtReassociationRequest {
}

impl MgmtReassociationRequest {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtReassociationRequest, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtReassociationResponse {
}

impl MgmtReassociationResponse {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtReassociationResponse, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtProbeRequest {
}

impl MgmtProbeRequest {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtProbeRequest, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtProbeResponse {
}

impl MgmtProbeResponse {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtProbeResponse, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtBeacon {
    duration: u16,
    destination_address: [u8; 6],
    source_address: [u8; 6],
    bss_id: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl MgmtBeacon {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtBeacon, std::io::Error> {
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
            MgmtBeacon {
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
pub struct MgmtAtim {
}

impl MgmtAtim {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtAtim, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtDisassociation {
}

impl MgmtDisassociation {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtDisassociation, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtAuthentication {
}

impl MgmtAuthentication {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtAuthentication, std::io::Error> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct MgmtDeauthentication {
    duration: u16,
    destination_address: [u8; 6],
    source_address: [u8; 6],
    bss_id: [u8; 6],
    sequence_number: u16,
    frame_number: u8,
}

impl MgmtDeauthentication {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtDeauthentication, std::io::Error> {
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
        
        //TODO parse fixed parameters

        Ok(
            MgmtDeauthentication {
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
pub struct MgmtAction {
}

impl MgmtAction {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtAction, std::io::Error> {
        unimplemented!();
    }
}
