use std::time::Duration;
use nalgebra::{Vector2, Vector3};
use crate::guidance::Guidance;

pub struct DoubleWallGuidance {}

impl DoubleWallGuidance {
    pub fn new() -> Self {
        Self {}
    }
}

impl Guidance for DoubleWallGuidance {
    fn update(&mut self, pos: Vector3<f64>, vel: Vector3<f64>, delta_t: Duration) {
        todo!()
    }

    fn get_target_motor_pos(&self) -> Vector2<f64> {
        todo!()
    }

    fn has_landed(&self) -> bool {
        todo!()
    }
}