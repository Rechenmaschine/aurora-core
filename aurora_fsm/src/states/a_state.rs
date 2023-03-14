use crate::state::{State, StateIdentifier};
use crate::states::event::Event;
use crate::states::Identifier;
use std::sync::mpsc::Receiver;

pub struct AState {}

impl State<Event, Identifier> for AState {
    fn handle_event(&mut self, event: Event) -> Option<Identifier> {
        todo!()
    }

    fn construct_successor(
        &mut self,
        next_state_ident: Identifier,
    ) -> Box<dyn State<Event, Identifier>> {
        todo!()
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        todo!()
    }

    fn destroy_event_sources(&mut self) {
        todo!()
    }

    fn identifier(&self) -> Identifier {
        Identifier::AState
    }

    fn enter(
        &mut self,
        prev_state_ident: Identifier,
        prev_state: Box<dyn State<Event, Identifier>>,
    ) {
        todo!()
    }

    fn exit(&mut self) {
        todo!()
    }
}
