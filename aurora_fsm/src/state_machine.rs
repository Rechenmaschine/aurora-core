use crate::state::State;
use std::sync::mpsc::Receiver;

struct StateMachine<E> {
    current_state: Box<dyn State<E>>,
    event_queue: Receiver<E>,
}

impl<E> StateMachine<E> {
    fn step(&mut self) {
        match self.event_queue.recv() {
            Ok(event) => {
                // Unwrap safety: Unwrap is safe as self.current_state is only None _during_ a state transition
                if let Some(mut next_state) = self.current_state.handle_event(event) {
                    self.current_state.destroy_event_sources();
                    self.event_queue = next_state.create_event_sources();
                    self.current_state = next_state;
                }
            }
            Err(_recv_err) => {
                todo!()
            }
        }
    }
}
