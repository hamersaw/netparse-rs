mod control;
mod data;
mod management;

use self::control::*;
use self::data::*;
use self::management::*;
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
    pub frame_type: IEEE802Dot11FrameType,
    /*pub bssid: Option<[u8; 6]>,
    pub source_address: Option<[u8; 6]>,
    pub destination_address: Option<[u8; 6]>,
    pub transmitter_address: Option<[u8; 6]>,
    pub receiver_address: Option<[u8; 6]>,
    pub sequence_number: Option<u16>,
    pub fragement_number: Option<u8>,*/
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

        let frame_type = match (packet_type, packet_subtype) {
            (0, 0) => IEEE802Dot11FrameType::MgmtAssociationRequest(try!(MgmtAssociationRequest::parse(input))),
            (0, 1) => IEEE802Dot11FrameType::MgmtAssociationResponse(try!(MgmtAssociationResponse::parse(input))),
            (0, 2) => IEEE802Dot11FrameType::MgmtReassociationRequest(try!(MgmtReassociationRequest::parse(input))),
            (0, 3) => IEEE802Dot11FrameType::MgmtReassociationResponse(try!(MgmtReassociationResponse::parse(input))),
            (0, 4) => IEEE802Dot11FrameType::MgmtProbeRequest(try!(MgmtProbeRequest::parse(input))),
            (0, 5) => IEEE802Dot11FrameType::MgmtProbeResponse(try!(MgmtProbeResponse::parse(input))),
            (0, 8) => IEEE802Dot11FrameType::MgmtBeacon(try!(MgmtBeacon::parse(input))),
            (0, 9) => IEEE802Dot11FrameType::MgmtAtim(try!(MgmtAtim::parse(input))),
            (0, 10) => IEEE802Dot11FrameType::MgmtDisassociation(try!(MgmtDisassociation::parse(input))),
            (0, 11) => IEEE802Dot11FrameType::MgmtAuthentication(try!(MgmtAuthentication::parse(input))),
            (0, 12) => IEEE802Dot11FrameType::MgmtDeauthentication(try!(MgmtDeauthentication::parse(input))),
            (0, 13) => IEEE802Dot11FrameType::MgmtAction(try!(MgmtAction::parse(input))),
            (1, 8) => IEEE802Dot11FrameType::CtrlBlockAckRequest(try!(CtrlBlockAckRequest::parse(input))),
            (1, 9) => IEEE802Dot11FrameType::CtrlBlockAck(try!(CtrlBlockAck::parse(input))),
            (1, 10) => IEEE802Dot11FrameType::CtrlPowerSavePoll(try!(CtrlPowerSavePoll::parse(input))),
            (1, 11) => IEEE802Dot11FrameType::CtrlRequestToSend(try!(CtrlRequestToSend::parse(input))),
            (1, 12) => IEEE802Dot11FrameType::CtrlClearToSend(try!(CtrlClearToSend::parse(input))),
            (1, 13) => IEEE802Dot11FrameType::CtrlAck(try!(CtrlAck::parse(input))),
            (1, 14) => IEEE802Dot11FrameType::CtrlCfEnd(try!(CtrlCfEnd::parse(input))),
            (1, 15) => IEEE802Dot11FrameType::CtrlCfEndPlusCfAck(try!(CtrlCfEndPlusCfAck::parse(input))),
            (2, 0) => IEEE802Dot11FrameType::Data(try!(Data::parse(input))),
            (2, 1) => IEEE802Dot11FrameType::DataPlusCfAck(try!(DataPlusCfAck::parse(input))),
            (2, 2) => IEEE802Dot11FrameType::DataPlusCfPoll(try!(DataPlusCfPoll::parse(input))),
            (2, 3) => IEEE802Dot11FrameType::DataPlusCfAckPlusCfPoll(try!(DataPlusCfAckPlusCfPoll::parse(input))),
            (2, 4) => IEEE802Dot11FrameType::DataNull(try!(DataNull::parse(input))),
            (2, 5) => IEEE802Dot11FrameType::DataCfAck(try!(DataCfAck::parse(input))),
            (2, 6) => IEEE802Dot11FrameType::DataCfPoll(try!(DataCfPoll::parse(input))),
            (2, 7) => IEEE802Dot11FrameType::DataCfAckPlusCfPoll(try!(DataCfAckPlusCfPoll::parse(input))),
            (2, 8) => IEEE802Dot11FrameType::DataQosData(try!(DataQosData::parse(input))),
            (2, 9) => IEEE802Dot11FrameType::DataQosDataPlusCfAck(try!(DataQosDataPlusCfAck::parse(input))),
            (2, 10) => IEEE802Dot11FrameType::DataQosDataPlusCfPoll(try!(DataQosDataPlusCfPoll::parse(input))),
            (2, 11) => IEEE802Dot11FrameType::DataQosDataPlusCfAckPlusCfPoll(try!(DataQosDataPlusCfAckPlusCfPoll::parse(input))),
            (2, 12) => IEEE802Dot11FrameType::DataQosNull(try!(DataQosNull::parse(input))),
            (2, 14) => IEEE802Dot11FrameType::DataQosPlusCfPollNoData(try!(DataQosPlusCfPollNoData::parse(input))),
            (2, 15) => IEEE802Dot11FrameType::DataQosPlusCfAckNoData(try!(DataQosPlusCfAckNoData::parse(input))),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "malformed frame")),
        };

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
                frame_type: frame_type,
                /*bssid: None,
                source_address: None,
                destination_address: None,
                transmitter_address: None,
                receiver_address: None,
                sequence_number: None,
                fragement_number: None,*/
            }
        )
    }
}

#[derive(Debug)]
pub enum IEEE802Dot11FrameType {
    MgmtAssociationRequest(MgmtAssociationRequest),
    MgmtAssociationResponse(MgmtAssociationResponse),
    MgmtReassociationRequest(MgmtReassociationRequest),
    MgmtReassociationResponse(MgmtReassociationResponse),
    MgmtProbeRequest(MgmtProbeRequest),
    MgmtProbeResponse(MgmtProbeResponse),
    MgmtBeacon(MgmtBeacon),
    MgmtAtim(MgmtAtim),
    MgmtDisassociation(MgmtDisassociation),
    MgmtAuthentication(MgmtAuthentication),
    MgmtDeauthentication(MgmtDeauthentication),
    MgmtAction(MgmtAction),
    CtrlBlockAckRequest(CtrlBlockAckRequest),
    CtrlBlockAck(CtrlBlockAck),
    CtrlPowerSavePoll(CtrlPowerSavePoll),
    CtrlRequestToSend(CtrlRequestToSend),
    CtrlClearToSend(CtrlClearToSend),
    CtrlAck(CtrlAck),
    CtrlCfEnd(CtrlCfEnd),
    CtrlCfEndPlusCfAck(CtrlCfEndPlusCfAck),
    Data(Data),
    DataPlusCfAck(DataPlusCfAck),
    DataPlusCfPoll(DataPlusCfPoll),
    DataPlusCfAckPlusCfPoll(DataPlusCfAckPlusCfPoll),
    DataNull(DataNull),
    DataCfAck(DataCfAck),
    DataCfPoll(DataCfPoll),
    DataCfAckPlusCfPoll(DataCfAckPlusCfPoll),
    DataQosData(DataQosData),
    DataQosDataPlusCfAck(DataQosDataPlusCfAck),
    DataQosDataPlusCfPoll(DataQosDataPlusCfPoll),
    DataQosDataPlusCfAckPlusCfPoll(DataQosDataPlusCfAckPlusCfPoll),
    DataQosNull(DataQosNull),
    DataQosPlusCfPollNoData(DataQosPlusCfPollNoData),
    DataQosPlusCfAckNoData(DataQosPlusCfAckNoData),
}
