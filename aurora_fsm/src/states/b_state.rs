use crate::state::State;
use crate::states::event::Event;
use std::sync::mpsc::Receiver;

pub struct BState {}

impl State<Event> for BState {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            _ => None,
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        todo!()
    }

    fn destroy_event_sources(&mut self) {
        todo!()
    }
}
