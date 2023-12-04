use arrayvec::ArrayVec;
use std::time::Duration;

pub mod protocol;

type CANFDPayload = ArrayVec<u8, 64>;

#[derive(Debug)]
pub struct CANFDFrame {
    id: u16,
    payload: CANFDPayload,
}

pub trait CANInterface {
    fn send(&mut self, frame: CANFDFrame);

    fn recv(&mut self, timeout: Option<Duration>) -> Option<CANFDFrame>;
}
