use bytes::{Buf, LittleEndian};

use error::NetparseError;
use super::ieee_802_dot_11::IEEE802Dot11Frame;

use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct RadiotapFrame {
    pub header_revision: u8,
    pub header_pad: u8,
    pub header_length: u16,
    pub tsft: Option<u64>,
    pub cfp: Option<bool>,
    pub preamble: Option<bool>,
    pub wep: Option<bool>,
    pub fragmentation: Option<bool>,
    pub fcs_at_end: Option<bool>,
    pub data_pad: Option<bool>,
    pub bad_fcs: Option<bool>,
    pub short_gi: Option<bool>,
    pub rate: Option<u8>,
    pub channel: Option<u16>,
    pub turbo: Option<bool>,
    pub complementary_code_keying: Option<bool>,
    pub orthogonal_frequency_division_multiplexing: Option<bool>,
    pub ghz2: Option<bool>,
    pub ghz5: Option<bool>,
    pub passive: Option<bool>,
    pub dynamic_cck_ofdm: Option<bool>,
    pub gaussian_frequency_shift_keying: Option<bool>,
    pub gsm: Option<bool>,
    pub static_turbo: Option<bool>,
    pub half_rate_channel: Option<bool>,
    pub quarter_rate_channel: Option<bool>,
    pub dbm_antenna_signal: Option<u8>,
    pub antenna: Option<u8>,
    pub bad_plcp: Option<bool>,
    pub ieee802dot11frame: IEEE802Dot11Frame,
}

impl RadiotapFrame {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<RadiotapFrame, NetparseError> {
        let header_revision = cursor.get_u8();
        let header_pad = cursor.get_u8();
        let header_length = cursor.get_u16_le();

        // present flags
        let present_flags = cursor.get_u32_le();
        let flag_tsft = (present_flags & 1) == 1;
        let flag_flags = (present_flags & 2) == 2;
        let flag_rate = (present_flags & 4) == 4;
        let flag_channel = (present_flags & 8) == 8;
        let flag_fhss = (present_flags & 16) == 16;
        let flag_dbm_antenna_signal = (present_flags & 32) == 32;
        let flag_dbm_antenna_noise = (present_flags & 64) == 64;
        let flag_lock_quality = (present_flags & 128) == 128;
        let flag_tx_attenuation = (present_flags & 256) == 256;
        let flag_db_tx_attenuation = (present_flags & 512) == 512;
        let flag_dbm_tx_power = (present_flags & 1024) == 1024;
        let flag_antenna = (present_flags & 2048) == 2048;
        let flag_db_antenna_signal = (present_flags & 4096) == 4096;
        let flag_db_antenna_noise = (present_flags & 8192) == 8192;
        let flag_rx_flags = (present_flags & 16384) == 16384;
        let flag_channel_plus = (present_flags & 262144) == 262144;
        let flag_mcs_information = (present_flags & 524288) == 524288;
        let flag_ampdu_status = (present_flags & 1048576) == 1048576;
        let flag_vht_information = (present_flags & 2097152) == 2097152;
        let flag_radiotap_ns_next = (present_flags & 536870912) == 536870912;
        let flag_vendor_ns_next = (present_flags & 1073741824) == 1073741824;
        let flag_ext = (present_flags & 2147483648) == 2147483648;

        // tsft
        let tsft = match flag_tsft {
            true => Some(cursor.get_u64_le()),
            false => None,
        };

        // flags
        let (cfp, preamble, wep, fragmentation, fcs_at_end,
                data_pad, bad_fcs, short_gi) = match flag_flags {
            true => {
                let flags = cursor.get_u8();
                /*pub cfp: Option<bool>,
                pub preamble: Option<bool>,
                pub wep: Option<bool>,
                pub fragmentation: Option<bool>,
                pub fcs_at_end: Option<bool>,
                pub data_pad: Option<bool>,
                pub bad_fcs: Option<bool>,
                pub short_gi: Option<bool>,*/
                (None, None, None, None, None, None, None, None)
            },
            false => (None, None, None, None, None, None, None, None),
        };

        // rate
        let rate = match flag_rate {
            true => Some(cursor.get_u8()),
            false => None,
        };
        
        // channel
        let channel = match flag_channel {
            true => Some(cursor.get_u16_le()),
            false => None,
        };

        let (turbo, complementary_code_keying,
                orthogonal_frequency_division_multiplexing, ghz2, ghz5, passive,
                dynamic_cck_ofdm, gaussian_frequency_shift_keying, gsm, static_turbo,
                half_rate_channel, quarter_rate_channel) = match flag_channel {
            true => {
                let channel_flags = cursor.get_u16_le();
                /*pub turbo: Option<bool>,
                pub complementary_code_keying: Option<bool>,
                pub orthogonal_requency_division_multiplexing: Option<bool>,
                pub ghz2: Option<bool>,
                pub ghz5: Option<bool>,
                pub passive: Option<bool>,
                pub dynamic_cck_ofdm: Option<bool>,
                pub faussian_requency_shift_keying: Option<bool>,
                pub gsm: Option<bool>,
                pub static_turbo: Option<bool>,
                pub half_rate_channel: Option<bool>,
                pub quarter_rate_channel: Option<bool>,*/
                (None, None, None, None, None, None, None,
                    None, None, None, None, None)
            },
            false => (None, None, None, None, None, None, None,
                    None, None, None, None, None),
        };

        // dbm_antenna_signal
        let dbm_antenna_signal = match flag_dbm_antenna_signal {
            true => Some(cursor.get_u8()),
            false => None,
        };

        // antenna
        let antenna = match flag_antenna {
            true => Some(cursor.get_u8()),
            false => None,
        };
        
        // rx flags
        let bad_plcp = match flag_rx_flags {
            true => {
                let rx_flags = cursor.get_u16_le();
                let bad_plcp = (rx_flags & 2) == 2;
                Some(bad_plcp)
            },
            false => None,
        };
        
        // parse IEEE802Dot11Frame
        let ieee802dot11frame = try!(IEEE802Dot11Frame::parse(cursor));

        Ok(
            RadiotapFrame {
                header_revision: header_revision,
                header_pad: header_pad,
                header_length: header_length,
                tsft: tsft,
                cfp: cfp,
                preamble: preamble,
                wep: wep,
                fragmentation: fragmentation,
                fcs_at_end: fcs_at_end,
                data_pad: data_pad,
                bad_fcs: bad_fcs,
                short_gi: short_gi,
                rate: rate,
                channel: channel,
                turbo: turbo,
                complementary_code_keying: complementary_code_keying,
                orthogonal_frequency_division_multiplexing:
                    orthogonal_frequency_division_multiplexing,
                ghz2: ghz2,
                ghz5: ghz5,
                passive: passive,
                dynamic_cck_ofdm: dynamic_cck_ofdm,
                gaussian_frequency_shift_keying: gaussian_frequency_shift_keying,
                gsm: gsm,
                static_turbo: static_turbo,
                half_rate_channel: half_rate_channel,
                quarter_rate_channel: quarter_rate_channel,
                dbm_antenna_signal: dbm_antenna_signal,
                antenna: antenna,
                bad_plcp: bad_plcp,
                ieee802dot11frame: ieee802dot11frame,
            }
        )
    }
}
