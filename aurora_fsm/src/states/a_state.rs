use crate::state::State;
use crate::states::b_state::BState;
use crate::states::event::Event;
use std::sync::mpsc::Receiver;

pub struct AState {}

impl State<Event> for AState {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            _ => Some(Box::new(BState {})),
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        todo!()
    }

    fn destroy_event_sources(&mut self) {
        todo!()
    }
}
