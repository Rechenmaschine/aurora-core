#![feature(once_cell)]

use crate::fsm::states::Idle;
use crate::hal::get_io_tree;
use aurora_fsm::state_machine::StateMachine;
use std::thread;
use std::time::Duration;

mod fsm;
mod hal;

fn main() {
    // Start FSM
    let mut state_machine = StateMachine::new(Idle::new());

    thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        println!("[DEBUG] Sending arming signal from test thread...");
        get_io_tree().armed.set(true);
    });

    loop {
        state_machine.step();
    }
}
