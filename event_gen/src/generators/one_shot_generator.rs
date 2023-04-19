use crate::event_generator::EventGenerator;

use std::sync::mpsc::Sender;
use std::thread;

pub struct OneShotGenerator<T: std::marker::Send> {
    pub value: T,
}

impl<T: 'static + std::marker::Send> EventGenerator<T,()> for OneShotGenerator<T> {
    fn start(self, send_handle: Sender<T>) -> thread::JoinHandle<()> {
        thread::spawn(move|| {
            send_handle.send(self.value).unwrap();
        })
    }
}