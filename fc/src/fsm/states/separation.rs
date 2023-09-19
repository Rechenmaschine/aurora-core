use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use aurora_fsm::state::State;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use event_gen::generators::one_shot_generator::OneShotGenerator;
use crate::fsm::Event;
use crate::fsm::states::DrogueDescent;
use crate::get_io_tree;
use crate::hal::event_generators::IoEventGen;

pub struct Separation {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl Separation {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }
}

impl State<Event> for Separation {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::SeparationTriggered => {
                println!("Triggering separation...");
                get_io_tree().separation_signal.set(true);
                None
            }
            Event::SeparationDetected => {
                println!("Separation completed, entering drogue descent...");
                Some(Box::new(DrogueDescent::new()))
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Coasting");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let separation_trigger_event_gen = OneShotGenerator {
            value: Event::SeparationTriggered
        };

        self.event_gen_handles.push(Box::new(separation_trigger_event_gen.start(sender.clone())));

        let separation_complete_event_gen =
            IoEventGen::new_single_shot(
                |tree| &tree.separated_nosecone,
                |sep| *sep,
                |_| Event::SeparationDetected
            );

        self.event_gen_handles.push(Box::new(separation_complete_event_gen.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}