use aurora_fsm::state::{State, UntypedState};
use aurora_fsm::state_machine::StateMachine;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

enum EventsA {
    GotoB,
    StayLoopState,
}

struct LoopState {
    rounds: u32,
}

impl State for LoopState {
    type Event = EventsA;

    fn handle_event(&mut self, event: Self::Event) -> Option<Box<dyn UntypedState>> {
        match event {
            EventsA::GotoB => Some(Box::new(BState)),
            EventsA::StayLoopState => None
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Self::Event> {
        let (tx, rx) = mpsc::channel();
        let rounds = self.rounds;
        thread::spawn(move || {
            for _ in 0..rounds {
                tx.send(EventsA::StayLoopState).unwrap();
            }
            tx.send(EventsA::GotoB).unwrap();
        });
        rx
    }

    fn destroy_event_sources(&mut self) {}
}

enum EventsB {
    GotoInvalid,
}

struct BState;

impl State for BState {
    type Event = EventsB;

    fn handle_event(&mut self, event: Self::Event) -> Option<Box<dyn UntypedState>> {
        match event {
            EventsB::GotoInvalid => {
                panic!("State transition into invalid state");
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Self::Event> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(EventsB::GotoInvalid).unwrap();
        });
        rx
    }

    fn destroy_event_sources(&mut self) {}
}

#[test]
#[should_panic]
fn test_basic_fsm_1() {
    let mut sm = StateMachine::new(LoopState { rounds: 0 });

    sm.step(); // 0 rounds, therefore goto B
    sm.step(); // goto invalid
}

#[test]
fn test_basic_fsm_2() {
    let mut sm = StateMachine::new(LoopState { rounds: 100 });
    for _ in 0..100 {
        sm.step(); // stay in LoopState
    }
    sm.step(); // goto B
}

#[test]
#[should_panic]
fn test_basic_fsm_3() {
    let mut sm = StateMachine::new(LoopState { rounds: 100 });
    for _ in 0..100 {
        sm.step(); // stay in LoopState
    }
    sm.step(); // goto B
    sm.step(); // goto invalid
}

#[test]
fn test_large_fsm() {
    let mut sm = StateMachine::new(LoopState { rounds: 1_000_000 });
    for _ in 0..1_000_000 {
        sm.step(); // stay in LoopState
    }
    sm.step(); // goto B
}

#[test]
#[should_panic]
fn test_large_fsm_panics() {
    let mut sm = StateMachine::new(LoopState { rounds: 1_000_000 });
    for _ in 0..1_000_000 {
        sm.step(); // stay in LoopState
    }
    sm.step(); // goto B
    sm.step(); // goto invalid
}