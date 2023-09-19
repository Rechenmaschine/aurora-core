use std::process::exit;
use aurora_fsm::state::State;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use std::time::Duration;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use event_gen::generators::tick_generator::TickGenerator;
use crate::fsm::Event;

pub struct GroundRecovery {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl GroundRecovery {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }
}

impl State<Event> for GroundRecovery {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::SystemShutdownTriggered => {
                println!("Timer ran out, triggering system shutdown...");
                exit(0);
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Armed");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let system_shutdown_timer =
            TickGenerator {
                min_duration: Duration::from_secs(60 * 15),
                event_producer: |_, _| -> Event { Event::SystemShutdownTriggered }
            };

        self.event_gen_handles.push(Box::new(system_shutdown_timer.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}
