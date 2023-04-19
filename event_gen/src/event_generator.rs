use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub trait EventGenerator<T: std::marker::Send, U> {
    fn start(self, send_handle: Sender<T>) -> JoinHandle<U>;
}