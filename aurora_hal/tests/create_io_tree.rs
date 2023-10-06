use aurora_hal::init_io_tree;

#[derive(Default)]
struct Inputs {}

#[derive(Default)]
struct Outputs {}

init_io_tree!{
    inputs: Inputs,
    outputs: Outputs
}

#[test]
fn io_tree_exists() {
    let io_tree = get_io_tree();
    let _inputs = &io_tree.inputs;
    let _outputs = &io_tree.outputs;
}