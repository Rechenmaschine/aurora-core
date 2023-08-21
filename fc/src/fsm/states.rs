use crate::fsm::Event;
use crate::hal::event_generators::IoEventGen;
use aurora_fsm;
use aurora_fsm::state::State;
use event_gen::event_generator::{EventGenHandle, EventGenerator};
use event_gen::generators::one_shot_generator::OneShotGenerator;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

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

pub struct Armed {}

impl Armed {
    fn new() -> Self {
        Self {}
    }
}

impl State<Event> for Armed {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::Exit => {
                println!("Entered ARMED state, exiting...");
                std::process::exit(0);
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Armed");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let step_event_gen = OneShotGenerator { value: Event::Exit };

        step_event_gen.start(sender);

        receiver
    }

    fn destroy_event_sources(&mut self) {
        // Empty.
    }
}
