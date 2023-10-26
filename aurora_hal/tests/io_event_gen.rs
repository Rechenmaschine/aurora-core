use aurora_hal::hal_event_generators::IoEventGen;
use aurora_hal::signaling::Signaling;
use aurora_hal::{init_io_tree, single_shot_io_event_gen};
use event_gen::event_generator::{EventGenHandle, EventGenerator};
use std::sync::mpsc;
use std::time::Duration;

#[derive(Default)]
struct Inputs {
    input1: Signaling<bool>,
}

#[derive(Default)]
struct Outputs {}

init_io_tree! {
    inputs: Inputs,
    outputs: Outputs
}

#[test]
fn can_create_event_gen() {
    let _event_gen = single_shot_io_event_gen!(inputs.input1, |i| *i, |_| ());
}

#[test]
fn can_handle_single_shot_event() {
    let event_gen = single_shot_io_event_gen!(inputs.input1, |i| *i, |_| true);
    let (sender, receiver) = mpsc::channel();
    let _handle = event_gen.start(sender);

    __iotree::get_io_tree().inputs.input1.set(true);
    assert_eq!(receiver.recv().unwrap(), true);
}

#[test]
fn not_sending_if_stopped() {
    let event_gen = single_shot_io_event_gen!(inputs.input1, |i| *i, |_| true);
    let (sender, receiver) = mpsc::channel();
    let mut handle = event_gen.start(sender);
    handle.stop();

    get_io_tree().inputs.input1.set(true);
    assert_eq!(
        receiver.recv_timeout(Duration::from_millis(400)),
        Err(mpsc::RecvTimeoutError::Disconnected)
    )
}
