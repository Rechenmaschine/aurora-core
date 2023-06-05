use aurora_fsm::state::State;
use aurora_fsm::state_machine::StateMachine;
use event_gen::event_generator::EventGenerator;
use event_gen::generators::one_shot_generator::OneShotGenerator;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

#[non_exhaustive]
#[derive(Debug)]
enum Event {
    OneShot,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum StateIdent {
    AInit,
    BInit,
    BRun,
}

struct AState {
    state_ident: Rc<RefCell<StateIdent>>,
}

impl AState {
    fn new(state_ident: Rc<RefCell<StateIdent>>) -> Self {
        Self { state_ident }
    }
}

impl State<Event> for AState {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            _ => Some(Box::new(BState {
                state_ident: Rc::clone(&self.state_ident),
            })),
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        let (sender, receiver) = std::sync::mpsc::channel();

        let one_shot = OneShotGenerator {
            value: Event::OneShot,
        };
        one_shot.start(sender);

        return receiver;
    }

    fn destroy_event_sources(&mut self) {
        // Empty
    }
}

struct BState {
    state_ident: Rc<RefCell<StateIdent>>,
}

impl State<Event> for BState {
    fn handle_event(&mut self, event: Event) -> Option<Box<dyn State<Event>>> {
        match event {
            _ => {
                self.state_ident.borrow_mut().replace(StateIdent::BRun);
                None
            }
        }
    }

    fn create_event_sources(&mut self) -> Receiver<Event> {
        self.state_ident.borrow_mut().replace(StateIdent::BInit);

        let (sender, receiver) = std::sync::mpsc::channel();

        let one_shot = OneShotGenerator {
            value: Event::OneShot,
        };
        one_shot.start(sender);

        return receiver;
    }

    fn destroy_event_sources(&mut self) {
        // Empty
    }
}

#[test]
fn run_basic_fsm() {
    let state_ident = Rc::new(RefCell::new(StateIdent::AInit));
    let mut fsm = StateMachine::new(AState::new(state_ident.clone()));

    assert_eq!(
        *(RefCell::borrow(Rc::borrow(&state_ident))),
        StateIdent::AInit
    );

    fsm.step(); // Enter BState

    assert_eq!(
        *(RefCell::borrow(Rc::borrow(&state_ident))),
        StateIdent::BInit
    );

    fsm.step(); // Do step inside BState

    assert_eq!(
        *(RefCell::borrow(Rc::borrow(&state_ident))),
        StateIdent::BRun
    );
}

#[test]
#[should_panic]
fn run_basic_fsm_into_invalid_state() {
    let state_ident = Rc::new(RefCell::new(StateIdent::AInit));
    let mut fsm = StateMachine::new(AState::new(state_ident.clone()));
    fsm.step(); // Enter BState
    fsm.step(); // Do step inside BState
    fsm.step(); // Do another step after the only event generator from BState has been exhausted
}
