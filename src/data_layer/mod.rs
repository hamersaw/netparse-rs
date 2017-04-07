pub mod ieee_802_dot_11;

use self::ieee_802_dot_11::IEEE802Dot11Frame;

#[derive(Debug)]
pub enum DataLayerType {
    IEEE802Dot11(IEEE802Dot11Frame),
}
