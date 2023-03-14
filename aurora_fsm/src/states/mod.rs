use crate::state::{State, StateIdentifier};
use event::Event;

pub mod a_state;
pub mod b_state;
pub mod event;

enum Identifier {
    AState,
    BState,
}

impl StateIdentifier<Event> for Identifier {
    fn construct_state(self) -> Box<dyn State<Event, Self>> {
        todo!()
    }
}
