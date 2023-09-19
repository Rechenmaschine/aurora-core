use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use aurora_fsm::state::State;
use event_gen::event_generator::{EventGenerator, EventGenHandle};
use event_gen::generators::tick_generator::TickGenerator;
use crate::fsm::Event;
use crate::fsm::states::GroundRecovery;
use crate::get_io_tree;
use crate::guidance::Guidance;
use crate::hal::event_generators::IoEventGen;
use crate::guidance::double_wall::DoubleWallGuidance;

pub struct ControlLoop {
    event_gen_handles: Vec<Box<dyn EventGenHandle>>,
    guidance: Box<dyn Guidance>,
}

impl ControlLoop {
    pub fn new() -> Self {
        Self {
            event_gen_handles: Vec::new(),
            guidance: Box::new(DoubleWallGuidance::new()),
        }
    }
}

impl State<Event> for ControlLoop {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            Event::ControlLoopTick(delta_t) => {
                let pos = get_io_tree().position.get();
                let vel = get_io_tree().velocity.get();
                self.guidance.update(pos, vel, delta_t);

                if self.guidance.has_landed() {
                    Some(Box::new(GroundRecovery::new()))
                } else {
                    get_io_tree().steering_motor_target_pos.set(self.guidance.get_target_motor_pos());
                    None
                }
            },
            Event::Landed => {
                println!("System has landed, entering ground recovery state...");
                Some(Box::new(GroundRecovery::new()))
            }
            e => {
                eprintln!("Warning: Encountered unexpected event: {e:?} in state Coasting");
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = mpsc::channel();

        let control_loop_tick_gen = TickGenerator {
            min_duration: Duration::from_secs_f64(1.0/100.0),
            event_producer: |prev, now| -> Event {Event::ControlLoopTick(now - prev)}
        };

        self.event_gen_handles.push(Box::new(control_loop_tick_gen.start(sender.clone())));

        let ground_recovery_manual_trigger =
            IoEventGen::new_single_shot(
                |tree| &tree.ground_recovery_manually_triggered,
                |trig| *trig,
                |_| Event::Landed
            );

        self.event_gen_handles.push(Box::new(ground_recovery_manual_trigger.start(sender)));

        receiver
    }

    fn destroy_event_sources(&mut self) {
        for mut handle in self.event_gen_handles.drain(..) {
            handle.stop()
        }
    }
}