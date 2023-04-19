use crate::event_generator::EventGenerator;

use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};

pub struct TickGenerator {
    pub min_duration: Duration
}

impl EventGenerator<Duration,()> for TickGenerator {
    fn start(self, send_handle: Sender<Duration>) -> thread::JoinHandle<()> {
        thread::spawn(move|| {
            let mut last_time = Instant::now();
            loop {
                if last_time.elapsed() >= self.min_duration {
                    send_handle.send(last_time.elapsed()).unwrap();
                    last_time = Instant::now();
                }
            }
        })
    }
}
