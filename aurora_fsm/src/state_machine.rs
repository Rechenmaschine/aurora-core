use crate::state::{ReceiveAny, UntypedState};

pub struct StateMachine {
    current_state: Box<dyn UntypedState>,
    event_queue: Box<dyn ReceiveAny>,
}

impl StateMachine {

    pub fn new(mut initial_state: impl UntypedState + 'static) -> Self {
        let event_queue = initial_state.create_untyped_sources();
        Self {
            current_state: Box::new(initial_state),
            event_queue,
        }
    }

    pub fn current_state(&self) -> &Box<dyn UntypedState> {
        &self.current_state
    }

    pub fn step(&mut self) {
        match self.event_queue.recv_any() {
            Ok(event) => {
                if let Some(next_state) = self.current_state.handle_untyped(event) {
                    self.current_state.destroy_untyped_sources();
                    self.current_state = next_state;
                    self.event_queue = self.current_state.create_untyped_sources();
                }
            }
            Err(_recv_err) => {
                panic!("The event queue has no senders left, no events are being generated, the FSM is stuck.")
            }
        }
    }
}
