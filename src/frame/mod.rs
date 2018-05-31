pub mod ieee_802_dot_11;
pub mod radiotap;

use self::ieee_802_dot_11::IEEE802Dot11Frame;
use self::radiotap::RadiotapFrame;

#[derive(Debug)]
pub enum FrameProtocol {
    IEEE802Dot11(IEEE802Dot11Frame),
    Radiotap(RadiotapFrame),
}
