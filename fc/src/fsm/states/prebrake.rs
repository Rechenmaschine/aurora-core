use aurora_fsm::state::State;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use nalgebra::Vector2;
use approx::AbsDiffEq;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use event_gen::generators::one_shot_generator::OneShotGenerator;
use crate::fsm::Event;
use crate::fsm::states::ControlLoop;
use crate::get_io_tree;
use crate::hal::event_generators::IoEventGen;

pub struct Prebrake {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
}

impl Prebrake {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
        }
    }

    fn target_motor_pos() -> Vector2<f64> {
        Vector2::new(-3.0, -3.0)
    }

    fn target_motor_delta() -> f64 {
        0.1
    }
}

impl State<Event> for Prebrake {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::PrebrakeTriggered => {
                println!("Setting motors to prebrake position...");
                get_io_tree().steering_motor_position.set(Prebrake::target_motor_pos());
                None
            },
            Event::PrebrakeComplete => {
                println!("Prebrake position reached, entering control loop...");
                Some(Box::new(ControlLoop::new()))
            },
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Prebrake");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let prebrake_trigger = OneShotGenerator {
            value: Event::PrebrakeTriggered
        };

        self.event_gen_handles.push(Box::new(prebrake_trigger.start(sender.clone())));

        let prebrake_complete_trigger = IoEventGen::new_single_shot(
            |tree| &tree.steering_motor_position,
            |pos| pos.abs_diff_eq(&Prebrake::target_motor_pos(), Prebrake::target_motor_delta()),
            |_| Event::PrebrakeComplete
        );

        self.event_gen_handles.push(Box::new(prebrake_complete_trigger.start(sender.clone())));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}
