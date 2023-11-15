use std::marker::Send;
use std::sync::mpsc::Sender;

/// Generates events and sends them asynchronously to a receiver.
pub trait EventGenerator<T: Send, U> {
    type Handle: EventGenHandle;
    /// Registers a [Sender] and starts generating events. Returns a handle that can stop the generator.
    fn start(self, send_handle: Sender<T>) -> Self::Handle;
}

pub trait EventGenHandle {
    /// Stops the generator.
    fn stop(&mut self);
}

impl EventGenHandle for () {
    fn stop(&mut self) {
        // Do nothing.
    }
}
