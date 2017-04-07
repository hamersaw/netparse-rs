pub mod ieee_802_dot_11;

use self::ieee_802_dot_11::IEEE802Dot11Frame;

pub enum FrameType {
    IEEE802Dot11(IEEE802Dot11Frame),
}
