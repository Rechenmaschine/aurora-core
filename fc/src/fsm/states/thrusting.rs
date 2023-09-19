use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use aurora_fsm::state::State;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use crate::fsm::Event;
use crate::fsm::states::Coasting;
use crate::hal::event_generators::IoEventGen;

pub struct Thrusting {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl Thrusting {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }
}

impl State<Event> for Thrusting {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::WeightlessnessDetected => {
                println!("Detected Weightlessness, entering Coasting state...");
                Some(Box::new(Coasting::new()))
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Thrusting");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let coasting_detection_event_gen =
            IoEventGen::new_single_shot(
                |tree| &tree.acceleration,
                |acceleration| acceleration.norm() < 5.0,
                |_| Event::WeightlessnessDetected
            );

        self.event_gen_handles.push(Box::new(coasting_detection_event_gen.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}