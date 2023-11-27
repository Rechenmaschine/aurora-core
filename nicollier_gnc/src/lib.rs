pub mod controller;
pub mod guidance;
pub mod model;

use crate::controller::Controller;
use crate::guidance::Guidance;
use crate::model::Model;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize)]
pub struct SystemState {
    pub inertial_frame_position: Vector3<f64>,
    pub inertial_frame_velocity: Vector3<f64>,
    pub inertial_frame_acceleration: Vector3<f64>,

    pub inertial_frame_angle: Vector3<f64>,
    pub inertial_frame_angle_velocity: Vector3<f64>,
    pub inertial_frame_angle_acceleration: Vector3<f64>,

    pub total_time: f64,

    //body frame pos is always 0,0,0
    pub body_frame_velocity: Vector3<f64>,
    pub body_frame_angle_velocity: Vector3<f64>,
    pub body_frame_angle_acceleration: Vector3<f64>,
    pub body_frame_acceleration: Vector3<f64>,
}

impl SystemState {
    pub fn initial_state() -> Self {
        Self {
            inertial_frame_position: Vector3::new(100.0, 100.0, -1000.0),
            inertial_frame_velocity: Vector3::zeros(),
            inertial_frame_acceleration: Vector3::zeros(),
            inertial_frame_angle: Vector3::zeros(),
            inertial_frame_angle_velocity: Vector3::zeros(),
            inertial_frame_angle_acceleration: Vector3::zeros(),

            body_frame_velocity: Vector3::zeros(),
            body_frame_angle_velocity: Vector3::zeros(),
            body_frame_angle_acceleration: Vector3::zeros(),
            body_frame_acceleration: Vector3::zeros(),

            total_time: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Deflections {
    pub sym: f64,
    pub asym: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Reference(pub f64);
