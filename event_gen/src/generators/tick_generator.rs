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
                thread::sleep(self.min_duration);
                send_handle
                    .send((self.event_producer)(Instant::now(), last_time))
                    .unwrap();
                last_time = Instant::now();
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn generates_at_least_20_events() {
        let (s, r) = mpsc::channel::<i32>();

        let tick_gen = TickGenerator {
            min_duration: Duration::from_millis(1),
            event_producer: |_now, _prev| 42,
        };
        tick_gen.start(s);

        for _ in 0..20 {
            assert_eq!(r.recv().unwrap(), 42);
        }
    }

    #[test]
    fn is_not_too_fast() {
        let test_duration = Duration::from_millis(100);
        let (s, r) = mpsc::channel::<Duration>();

        let tick_gen = TickGenerator {
            min_duration: test_duration,
            event_producer: |now, prev| now - prev,
        };

        tick_gen.start(s);

        for _ in 0..4 {
            let iter_duration = r.recv().unwrap();
            assert!(
                iter_duration > test_duration,
                "iter_duration: {:?}, expected_duration: {:?}",
                iter_duration,
                test_duration
            );
        }
    }

    #[test]
    fn is_not_too_slow() {
        let test_duration = Duration::from_millis(100);
        let acceptable_error = Duration::from_millis(15);
        let (s, r) = mpsc::channel::<Duration>();

        let tick_gen = TickGenerator {
            min_duration: test_duration,
            event_producer: |now, prev| now - prev,
        };

        tick_gen.start(s);

        for _ in 0..4 {
            let iter_duration = r.recv().unwrap();
            assert!(
                iter_duration - test_duration < acceptable_error,
                "expected: {:?}, actual: {:?}, deviation: {:?}, acceptable delta: {:?}",
                test_duration,
                iter_duration,
                iter_duration - test_duration,
                acceptable_error
            );
        }
    }
}
