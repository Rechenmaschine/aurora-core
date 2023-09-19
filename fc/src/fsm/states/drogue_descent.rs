use aurora_fsm::state::State;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use crate::fsm::Event;
use crate::fsm::states::MainDeployment;
use crate::get_io_tree;
use crate::hal::event_generators::IoEventGen;

pub struct DrogueDescent {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl DrogueDescent {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }
}

impl State<Event> for DrogueDescent {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::MainDeploymentAltitudeDetected => {
                println!("Detected low altitude, entering MainDeployment state...");
                get_io_tree().main_deployment_signal.set(true);
                Some(Box::new(MainDeployment::new()))
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Armed");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let main_altitude_detection_event_gen =
            IoEventGen::new_single_shot(
                |tree| &tree.position,
                |position| position.z.abs() < 900.0,
                |_| Event::MainDeploymentAltitudeDetected
            );

        self.event_gen_handles.push(Box::new(main_altitude_detection_event_gen.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}
