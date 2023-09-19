pub mod event_generators;
mod signal_var;

pub use crate::hal::signal_var::Signaling;
use std::sync::OnceLock;
use nalgebra::{Vector2, Vector3};

pub struct IoTree {
    // Inputs (process variables)
    pub armed: Signaling<bool>,
    pub acceleration: Signaling<Vector3<f64>>,
    pub position: Signaling<Vector3<f64>>,
    pub velocity: Signaling<Vector3<f64>>, // NED (North-East-Down) coordinate frame,
    pub separated_nosecone: Signaling<bool>,
    pub steering_motor_position: Signaling<Vector2<f64>>,
    pub ground_recovery_manually_triggered: Signaling<bool>,

    // Outputs (control variables)
    pub separation_signal: Signaling<bool>,
    pub main_deployment_signal: Signaling<bool>,
    pub steering_motor_target_pos: Signaling<Vector2<f64>>,
}

static IO_TREE: OnceLock<IoTree> = OnceLock::new();

pub fn get_io_tree() -> &'static IoTree {
    IO_TREE.get_or_init(IoTree::new)
}

impl IoTree {
    fn new() -> Self {
        Self {
            armed: Signaling::new(false),
            acceleration: Signaling::new(Vector3::new(0.0, 0.0, 0.0)),
            position: Signaling::new(Vector3::new(0.0, 0.0, 0.0)),
            velocity: Signaling::new(Vector3::new(0.0, 0.0, 0.0)),
            separated_nosecone: Signaling::new(false),
            steering_motor_position: Signaling::new(Vector2::new(0.0, 0.0)),
            ground_recovery_manually_triggered: Signaling::new(false),

            separation_signal: Signaling::new(false),
            main_deployment_signal: Signaling::new(false),
            steering_motor_target_pos: Signaling::new(Vector2::new(0.0, 0.0)),

        }
    }
}
