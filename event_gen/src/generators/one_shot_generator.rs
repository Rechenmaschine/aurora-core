use crate::event_generator::EventGenerator;

use std::marker::Send;
use std::sync::mpsc::Sender;
use std::thread;

pub struct OneShotGenerator<T: Send> {
    pub value: T,
}

impl<T: 'static + std::marker::Send> EventGenerator<T, ()> for OneShotGenerator<T> {
    fn start(self, send_handle: Sender<T>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            send_handle.send(self.value).unwrap();
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn generates_one_event() {
        let (s, r) = mpsc::channel();
        let one_shot = OneShotGenerator { value: 42 };
        one_shot.start(s);
        assert_eq!(r.recv().unwrap(), 42);
    }

    #[test]
    fn generates_only_one_event() {
        let (s, r) = mpsc::channel();
        let one_shot = OneShotGenerator { value: 42 };
        one_shot.start(s);
        assert_eq!(r.recv().unwrap(), 42);

        assert!(
            r.recv().is_err(),
            "The one shot event generator produced more than one message on the channel"
        );
    }
}
