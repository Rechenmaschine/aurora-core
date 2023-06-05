use crate::state::State;
use std::sync::mpsc::Receiver;

pub struct StateMachine<E> {
    current_state: Box<dyn State<E>>,
    event_queue: Receiver<E>,
}

impl<E> StateMachine<E> {
    pub fn new(initial_state: impl State<E> + 'static) -> Self {
        let mut boxed = Box::new(initial_state);
        let queue = boxed.create_event_sources();
        Self {
            current_state: boxed,
            event_queue: queue,
        }
    }

    pub fn step(&mut self) {
        match self.event_queue.recv() {
            Ok(event) => {
                if let Some(mut next_state) = self.current_state.handle_event(event) {
                    self.current_state.destroy_event_sources();
                    self.event_queue = next_state.create_event_sources();
                    self.current_state = next_state;
                }
            }
            Err(_recv_err) => {
                panic!("The event queue has no senders left, no events are being generated, the FSM is stuck.")
            }
        }
    }
}
