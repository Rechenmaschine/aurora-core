use aurora_fsm::state::State;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use std::time::Duration;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use event_gen::generators::tick_generator::TickGenerator;
use crate::fsm::Event;
use crate::fsm::states::Prebrake;

pub struct MainDeployment {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl MainDeployment {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }
}

impl State<Event> for MainDeployment {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::MainDeploymentComplete => {
                println!("MainDeployment complete, entering prebrake state...");
                Some(Box::new(Prebrake::new()))
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state MainDeployment");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let main_deployment_timer =
            TickGenerator {
                min_duration: Duration::from_secs_f64(2.5),
                event_producer: |_, _| -> Event { Event::MainDeploymentComplete },
            };

        self.event_gen_handles.push(Box::new(main_deployment_timer.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}
