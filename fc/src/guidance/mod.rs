use std::time::Duration;
use nalgebra::{Vector2, Vector3};

pub mod double_wall;

pub trait Guidance {
    fn update(&mut self, pos: Vector3<f64>, vel: Vector3<f64>, delta_t: Duration);
    fn get_target_motor_pos(&self) -> Vector2<f64>;
    fn has_landed(&self) -> bool;
}