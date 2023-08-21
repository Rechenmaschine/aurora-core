pub mod event_generators;
mod signal_var;

pub use crate::hal::signal_var::Signaling;
use std::sync::OnceLock;

pub struct IoTree {
    pub armed: Signaling<bool>,
}

static IO_TREE: OnceLock<IoTree> = OnceLock::new();

pub fn get_io_tree() -> &'static IoTree {
    IO_TREE.get_or_init(IoTree::new)
}

impl IoTree {
    fn new() -> Self {
        Self {
            armed: Signaling::new(false),
        }
    }
}
