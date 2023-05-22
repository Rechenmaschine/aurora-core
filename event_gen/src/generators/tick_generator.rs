use crate::event_generator::EventGenerator;

use std::marker::Send;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};

pub struct TickGenerator<T: Send> {
    pub min_duration: Duration,
    pub event_producer: fn(Instant, Instant) -> T,
}

impl<T: 'static + Send> EventGenerator<T, ()> for TickGenerator<T> {
    fn start(self, send_handle: Sender<T>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut last_time = Instant::now();
            loop {
                send_handle
                    .send((self.event_producer)(Instant::now(), last_time))
                    .unwrap();
                last_time = Instant::now();
                thread::sleep(self.min_duration)
            }
        })
    }
}
