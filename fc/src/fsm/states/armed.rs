use aurora_fsm::state::State;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use crate::fsm::Event;
use crate::fsm::states::Thrusting;
use crate::hal::event_generators::IoEventGen;

pub struct Armed {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl Armed {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }
}

impl State<Event> for Armed {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::LiftoffDetected => {
                println!("Detected Liftoff, entering THRUSTING state...");
                Some(Box::new(Thrusting::new()))
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Armed");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let liftoff_detection_event_gen =
            IoEventGen::new_single_shot(
                |tree| &tree.acceleration,
                |acceleration| acceleration.norm() > 20.0,
                |_| Event::LiftoffDetected
            );

        self.event_gen_handles.push(Box::new(liftoff_detection_event_gen.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}
