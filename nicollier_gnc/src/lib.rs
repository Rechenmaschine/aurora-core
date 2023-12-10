pub mod controller;
pub mod guidance;
pub mod model;

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::controller::p_controller::PController;
use crate::controller::Controller;
use crate::guidance::double_wall::DoubleWallGuidance;
use crate::guidance::Guidance;
use crate::guidance::superior_double_wall::Axis::{X, Y};
use crate::guidance::superior_double_wall::SuperiorDoubleWallGuidance;
use crate::model::four_dof::FourDof;
use crate::model::three_dof::ThreeDof;
use crate::model::Model;
use std::f64::consts::PI;


// Function to calculate the wind vector at a given height in mountainous terrain
// Parameter: height_m - Height in meters for which the wind vector is calculated
// Source for the model: https://windroseexcel.com/guides/vertical-extrapolation-of-wind-speed/
// Note: This model is simplified and may not be precise for heights above 1000m, especially in complex terrains.

pub fn get_wind(height_m: f64)-> Vector3<f64> {
    // Constants
    const GROUND_SPEED_KM_H: f64 = 4.0;  // Wind speed at ground level in km/h
    const ALPHA: f64 = 0.25;             // Wind shear exponent for mountainous terrain
    const THETA_RAD: f64 = -PI/2.0;     // Wind direction in rad, cos(theta_angle)=x_komponente

    // Convert ground speed to m/s and degrees to radians
    let v0 = GROUND_SPEED_KM_H /3.7;
    let mut speed=v0;
    // Calculate wind speed at the given height using the Power Law
    if height_m.abs()>10.0 {
        speed = v0 * (height_m.abs() / 1.0).powf(ALPHA);
    }//else { speed=v0; }//this is not covered by the model, because the model is not precise very low heights
    // Calculate wind vector components
    let vx = speed * THETA_RAD.cos(); // x component
    let vy = speed * THETA_RAD.sin(); // y component
    Vector3::new(vx, vy, 0.0) // Assuming no vertical component
    //Vector3::new(0.0, 0.0, 0.0)
}





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
            inertial_frame_position: Vector3::new(0.0, 0.0, -2000.0),
            inertial_frame_velocity: Vector3::new(0.0,0.0,0.0),
            inertial_frame_acceleration: Vector3::zeros(),
            inertial_frame_angle: Vector3::new(0.0,0.0,0.0),
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
    pub guidance: SuperiorDoubleWallGuidance,
    pub controller: PController,
    pub model: FourDof,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            guidance: SuperiorDoubleWallGuidance::new(X, -30.0, 30.0, 1000.0, 10.0),
            controller: PController::new(),
            model: FourDof::new(SystemState::initial_state()),
        }
    }

    pub fn step(&mut self) -> (SystemState, Reference, Deflections, SystemState) {
        let delta_t = 0.01;

        let mut state = self.model.get_state();
        let reference = self.guidance.get_reference(state);
        let control_inputs = self.controller.step(state, reference, delta_t);

        let new_state = self.model.step(control_inputs, delta_t);

        (state, reference, control_inputs, new_state)
    }

    pub fn done(&self) -> bool {
        self.model.landed()
    }
}
