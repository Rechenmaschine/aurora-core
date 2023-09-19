use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use aurora_fsm::state::State;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use crate::fsm::Event;
use crate::fsm::states::Separation;
use crate::hal::event_generators::IoEventGen;

pub struct Coasting {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl Coasting {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }
}

impl State<Event> for Coasting {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::ApogeeDetected => {
                println!("Detected apogee, triggering separation...");
                Some(Box::new(Separation::new()))
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Coasting");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let apogee_detection_event_gen =
            IoEventGen::new_single_shot(
                |tree| &tree.velocity,
                |velocity| velocity.z > 0.0,
                |_| Event::ApogeeDetected
            );

        self.event_gen_handles.push(Box::new(apogee_detection_event_gen.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}