use aurora_hal::{init_io_tree, single_shot_io_event_gen};
use aurora_hal::signaling::Signaling;
use aurora_hal::hal_event_generators::IoEventGen;

#[derive(Default)]
struct Inputs {
    input1: Signaling<bool>
}

#[derive(Default)]
struct Outputs {}

init_io_tree!{
    inputs: Inputs,
    outputs: Outputs
}

#[test]
fn can_create_event_gen() {
    let _event_gen = single_shot_io_event_gen!(
        inputs.input1,
        |i| *i,
        |_| ()
    );
}