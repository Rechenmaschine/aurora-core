use std::any::Any;
use std::sync::mpsc::{Receiver, RecvError};

pub trait State: UntypedState {
    type Event;
    fn handle_event(&mut self, event: Self::Event) -> Option<Box<dyn UntypedState>>;

    fn create_event_sources(&mut self) -> Receiver<Self::Event>;

    fn destroy_event_sources(&mut self);
}

pub trait UntypedState {
    fn handle_untyped(&mut self, event: Box<dyn Any>) -> Option<Box<dyn UntypedState>>;

    fn create_untyped_sources(&mut self) -> Box<dyn ReceiveAny>;

    fn destroy_untyped_sources(&mut self);
}

impl<T, E: 'static> UntypedState for T
    where
        T: State<Event=E>,
{
    fn handle_untyped(&mut self, event: Box<dyn Any>) -> Option<Box<dyn UntypedState>> {
        self.handle_event(
            *event
                .downcast::<E>()
                .expect("Event type mismatch. This is an implementation error, please report it."),
        )
    }

    fn create_untyped_sources(&mut self) -> Box<dyn ReceiveAny> {
        Box::new(self.create_event_sources())
    }

    fn destroy_untyped_sources(&mut self) {
        self.destroy_event_sources();
    }
}

pub trait ReceiveAny {
    fn recv_any(&mut self) -> Result<Box<dyn Any>, RecvError>;
}

impl<T: 'static> ReceiveAny for Receiver<T> {
    fn recv_any(&mut self) -> Result<Box<dyn Any>, RecvError> {
        self.recv().map(|x| Box::new(x) as Box<dyn Any>)
    }
}
