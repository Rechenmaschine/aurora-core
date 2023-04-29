use std::sync::mpsc::Sender;
use std::marker::Send;
use std::thread::JoinHandle;

pub trait EventGenerator<T: Send, U> {
    fn start(self, send_handle: Sender<T>) -> JoinHandle<U>;
}