use aurora_fsm::state::State;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use crate::fsm::Event;
use crate::fsm::states::Armed;
use crate::hal::event_generators::IoEventGen;

pub struct Idle {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl Idle {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }
}

impl State<Event> for Idle {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::Arm => {
                println!("Received ARM signal, arming...");
                Some(Box::new(Armed::new()))
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Idle");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let arm_event_gen =
            IoEventGen::new_single_shot(|tree| &tree.armed, |armed| *armed, |_| Event::Arm);

        self.event_gen_handles
            .push(Box::new(arm_event_gen.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}
