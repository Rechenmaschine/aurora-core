use std::marker::Send;
use std::sync::mpsc::Sender;

pub trait EventGenerator<T: Send, U> {
    type Handle: EventGenHandle;
    fn start(self, send_handle: Sender<T>) -> Self::Handle;
}

pub trait EventGenHandle {
    fn stop(self);
}
