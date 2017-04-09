use bytes::{BigEndian, Buf, LittleEndian};

mod control;
mod data;
mod management;

use self::control::*;
use self::data::*;
use self::management::*;

use std;
use std::io::{Cursor, Read};

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
}

impl IEEE802Dot11Frame {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<IEEE802Dot11Frame, std::io::Error> {
        //parse frame control
        let frame_control = cursor.get_u8();
        let protocol_version = frame_control & 3u8;
        let packet_type = (frame_control & 12u8) >> 2;
        let packet_subtype = (frame_control & 240u8) >> 4;

        let frame_control_flags = cursor.get_u8();
        let to_ds = (frame_control_flags & 1) == 1;
        let from_ds = (frame_control_flags & 2) == 2;
        let more_fragements = (frame_control_flags & 4) == 4;
        let retry = (frame_control_flags & 8) == 8;
        let power_management = (frame_control_flags & 16) == 16;
        let more_data = (frame_control_flags & 32) == 32;
        let wep = (frame_control_flags & 64) == 64;
        let order = (frame_control_flags & 128) == 128;

        let frame_type = match (packet_type, packet_subtype) {
            (0, 0) => IEEE802Dot11FrameType::MgmtAssociationRequest(try!(MgmtAssociationRequest::parse(cursor))),
            (0, 1) => IEEE802Dot11FrameType::MgmtAssociationResponse(try!(MgmtAssociationResponse::parse(cursor))),
            (0, 2) => IEEE802Dot11FrameType::MgmtReassociationRequest(try!(MgmtReassociationRequest::parse(cursor))),
            (0, 3) => IEEE802Dot11FrameType::MgmtReassociationResponse(try!(MgmtReassociationResponse::parse(cursor))),
            (0, 4) => IEEE802Dot11FrameType::MgmtProbeRequest(try!(MgmtProbeRequest::parse(cursor))),
            (0, 5) => IEEE802Dot11FrameType::MgmtProbeResponse(try!(MgmtProbeResponse::parse(cursor))),
            (0, 8) => IEEE802Dot11FrameType::MgmtBeacon(try!(MgmtBeacon::parse(cursor))),
            (0, 9) => IEEE802Dot11FrameType::MgmtAtim(try!(MgmtAtim::parse(cursor))),
            (0, 10) => IEEE802Dot11FrameType::MgmtDisassociation(try!(MgmtDisassociation::parse(cursor))),
            (0, 11) => IEEE802Dot11FrameType::MgmtAuthentication(try!(MgmtAuthentication::parse(cursor))),
            (0, 12) => IEEE802Dot11FrameType::MgmtDeauthentication(try!(MgmtDeauthentication::parse(cursor))),
            (0, 13) => IEEE802Dot11FrameType::MgmtAction(try!(MgmtAction::parse(cursor))),
            (1, 8) => IEEE802Dot11FrameType::CtrlBlockAckRequest(try!(CtrlBlockAckRequest::parse(cursor))),
            (1, 9) => IEEE802Dot11FrameType::CtrlBlockAck(try!(CtrlBlockAck::parse(cursor))),
            (1, 10) => IEEE802Dot11FrameType::CtrlPowerSavePoll(try!(CtrlPowerSavePoll::parse(cursor))),
            (1, 11) => IEEE802Dot11FrameType::CtrlRequestToSend(try!(CtrlRequestToSend::parse(cursor))),
            (1, 12) => IEEE802Dot11FrameType::CtrlClearToSend(try!(CtrlClearToSend::parse(cursor))),
            (1, 13) => IEEE802Dot11FrameType::CtrlAck(try!(CtrlAck::parse(cursor))),
            (1, 14) => IEEE802Dot11FrameType::CtrlCfEnd(try!(CtrlCfEnd::parse(cursor))),
            (1, 15) => IEEE802Dot11FrameType::CtrlCfEndPlusCfAck(try!(CtrlCfEndPlusCfAck::parse(cursor))),
            (2, 0) => IEEE802Dot11FrameType::Data(try!(Data::parse(cursor))),
            (2, 1) => IEEE802Dot11FrameType::DataPlusCfAck(try!(DataPlusCfAck::parse(cursor))),
            (2, 2) => IEEE802Dot11FrameType::DataPlusCfPoll(try!(DataPlusCfPoll::parse(cursor))),
            (2, 3) => IEEE802Dot11FrameType::DataPlusCfAckPlusCfPoll(try!(DataPlusCfAckPlusCfPoll::parse(cursor))),
            (2, 4) => IEEE802Dot11FrameType::DataNull(try!(DataNull::parse(cursor))),
            (2, 5) => IEEE802Dot11FrameType::DataCfAck(try!(DataCfAck::parse(cursor))),
            (2, 6) => IEEE802Dot11FrameType::DataCfPoll(try!(DataCfPoll::parse(cursor))),
            (2, 7) => IEEE802Dot11FrameType::DataCfAckPlusCfPoll(try!(DataCfAckPlusCfPoll::parse(cursor))),
            (2, 8) => IEEE802Dot11FrameType::DataQosData(try!(DataQosData::parse(cursor))),
            (2, 9) => IEEE802Dot11FrameType::DataQosDataPlusCfAck(try!(DataQosDataPlusCfAck::parse(cursor))),
            (2, 10) => IEEE802Dot11FrameType::DataQosDataPlusCfPoll(try!(DataQosDataPlusCfPoll::parse(cursor))),
            (2, 11) => IEEE802Dot11FrameType::DataQosDataPlusCfAckPlusCfPoll(try!(DataQosDataPlusCfAckPlusCfPoll::parse(cursor))),
            (2, 12) => IEEE802Dot11FrameType::DataQosNull(try!(DataQosNull::parse(cursor))),
            (2, 14) => IEEE802Dot11FrameType::DataQosPlusCfPollNoData(try!(DataQosPlusCfPollNoData::parse(cursor))),
            (2, 15) => IEEE802Dot11FrameType::DataQosPlusCfAckNoData(try!(DataQosPlusCfAckNoData::parse(cursor))),
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
