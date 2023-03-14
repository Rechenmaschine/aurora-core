use std::sync::mpsc::Receiver;

pub trait State<E> {
    fn handle_event(&mut self, event: E) -> Option<Box<dyn State<E>>>;

    fn create_event_sources(&mut self) -> Receiver<E>;

    fn destroy_event_sources(&mut self);
}
