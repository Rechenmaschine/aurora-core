use std::any::Any;
use std::sync::mpsc::Receiver;

pub trait State {
    type Event;
    fn handle_event(&mut self, event: Self::Event) -> Option<Box<dyn Any + State>>;

    fn create_event_sources(&mut self) -> Receiver<Self::Event>;

    fn destroy_event_sources(&mut self);
}
