pub mod controller;
pub mod guidance;
pub mod model;

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::controller::p_controller::PController;
use crate::controller::Controller;
use crate::guidance::double_wall::DoubleWallGuidance;
use crate::guidance::Guidance;
use crate::model::three_dof::ThreeDof;
use crate::model::Model;

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
            inertial_frame_position: Vector3::new(0.0, 0.0, -1500.0),
            inertial_frame_velocity: Vector3::zeros(),
            inertial_frame_acceleration: Vector3::zeros(),
            inertial_frame_angle: Vector3::new(2.0,1.0,1.0),
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

pub struct Simulation {
    pub guidance: DoubleWallGuidance,
    pub controller: PController,
    pub model: ThreeDof,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            guidance: DoubleWallGuidance::new(60.0, 10.0, 5.0, 100.0, 0.5),
            controller: PController::new(),
            model: ThreeDof::new(SystemState::initial_state()),
        }
    }

    pub fn step(&mut self) -> (SystemState, Reference, Deflections, SystemState) {
        let delta_t = 0.01;

        let state = self.model.get_state();
        let reference = self.guidance.get_reference(state);
        let control_inputs = self.controller.step(state, reference, delta_t);

        let new_state = self.model.step(control_inputs, delta_t);

        (state, reference, control_inputs, new_state)
    }

    pub fn done(&self) -> bool {
        self.model.landed()
    }
}
