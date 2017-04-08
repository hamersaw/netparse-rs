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
}

impl MgmtBeacon {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtBeacon, std::io::Error> {
        unimplemented!();
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
}

impl MgmtDeauthentication {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<MgmtDeauthentication, std::io::Error> {
        unimplemented!();
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

