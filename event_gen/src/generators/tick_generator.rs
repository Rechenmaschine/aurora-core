use crate::event_generator::{EventGenHandle, EventGenerator};

use std::marker::Send;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

pub struct TickGenerator<T: Send> {
    pub min_duration: Duration,
    pub event_producer: fn(Instant, Instant) -> T,
}

pub struct TickGenHandle {
    join_handle: Option<thread::JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
}

impl EventGenHandle for TickGenHandle {
    fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(join_handle) = self.join_handle.take() {
            join_handle
                .join()
                .expect("Failed to join thread of tick genenerator after sending exit signal");
        }
    }
}

impl<T: 'static + Send> EventGenerator<T, ()> for TickGenerator<T> {
    type Handle = TickGenHandle;
    fn start(self, send_handle: Sender<T>) -> Self::Handle {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_2 = stop_flag.clone();

        let join_handle = thread::spawn(move || {
            let mut last_time = Instant::now();
            while !stop_flag_2.load(Ordering::Relaxed) {
                thread::sleep(self.min_duration);
                send_handle
                    .send((self.event_producer)(Instant::now(), last_time))
                    .unwrap();
                last_time = Instant::now();
            }
        });

        Self::Handle {
            join_handle: Some(join_handle),
            stop_flag,
        }
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

    #[test]
    fn does_stop() {
        let (s, r) = mpsc::channel::<i32>();

        let tick_gen = TickGenerator {
            min_duration: Duration::from_millis(1),
            event_producer: |_now, _prev| 42,
        };

        let mut handle = tick_gen.start(s);
        handle.stop();

        let stop_time = Instant::now();
        while Instant::now() - stop_time < Duration::from_millis(500) {
            if let Err(_) = r.recv() {
                // We have stopped receiving events
                assert!(
                    true,
                    "Stopped successfully after {:?}",
                    Instant::now() - stop_time
                );
                return;
            }
        }

        assert!(false, "Failed to stop within 500 ms");
    }
}
