use crate::state::{State, StateIdentifier};
use std::sync::mpsc::Receiver;

struct StateMachine<E, I> {
    current_state: Option<Box<dyn State<E, I>>>, // This is an option so the state can be empty during a transition
    event_queue: Receiver<E>,
}

impl<E, I> StateMachine<E, I>
where
    I: StateIdentifier<E>,
{
    fn step(&mut self) {
        match self.event_queue.recv() {
            Ok(event) => {
                // Unwrap safety: Unwrap is safe as self.current_state is only None _during_ a state transition
                if let Some(next_state) = self.current_state.as_mut().unwrap().handle_event(event) {
                    let (new_state, new_evt_queue) =
                        next_state.transition_from(self.current_state.take().unwrap());
                    self.current_state = Some(new_state);
                    self.event_queue = new_evt_queue;
                }
            }
            Err(_recv_err) => {
                todo!()
            }
        }
    }
}
