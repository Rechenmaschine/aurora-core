use crate::model::Model;
use crate::{Deflections, SystemState};
use nalgebra::{Matrix, Matrix3, Rotation3, Vector3};
use std::alloc::System;

pub struct ThreeDof {
    state: SystemState,
}
const A: f64 = 0.0;     //Coefficient for roll rate in the ODE
const B1: f64 = 0.0;    //Coefficient for A-symmetric inputs in the ODE
const B2: f64 = 0.0;
const VELOCITY_VERTICAL: f64 = 0.0;
const AIRSPEED_HORIZONTAL: f64 = 0.0;
//Coefficient for symmetric inputs in the ODE
impl ThreeDof {
    pub fn new(state: SystemState) -> Self {
        Self {
            state
        }
    }
}

impl Model for ThreeDof {

    type State = SystemState;
    type Input = Deflections;

    fn get_state(&self) -> Self::State {
        self.state
    }
    fn step(&mut self, input: Self::Input, delta_t: f64) -> Self::State {

        let mut state:SystemState = self.state;

        //see Periphas report page 73 (7.1)
        state.inertial_frame.angle_acceleration.z =
            A * state.inertial_frame.angle_velocity.z
            + B1 * (input.asym / (1.0 + input.sym))
            + B2 * (input.asym / (1.0 + input.sym)).powf(3.0) ;


        state.inertial_frame.angle_velocity.x = 0.0;
        state.inertial_frame.angle_velocity.y = 0.0;
        state.inertial_frame.angle_velocity.z += delta_t * state.inertial_frame.angle_acceleration.z;

        //Velocities (Inertial frame)
        state.inertial_frame.velocity.x = AIRSPEED_HORIZONTAL * f64::cos(state.inertial_frame.angle.z);//TODO: add wind
        state.inertial_frame.velocity.y = AIRSPEED_HORIZONTAL * f64::sin(state.inertial_frame.angle.z);//TODO: add wind
        state.inertial_frame.velocity.z = VELOCITY_VERTICAL;

        state.inertial_frame.angle.x = 0.0;
        state.inertial_frame.angle.y = 0.0;
        state.inertial_frame.angle.z += delta_t * state.inertial_frame.angle_velocity.z;

        //TODO:transform to body frame
        state.body_frame.velocity = self.inertial_to_body() * state.inertial_frame.velocity;

        state.total_time += delta_t;
        return self.state;


    }
    fn inertial_to_body(&self) -> Rotation3<f64> {

        let roll:f64 = self.state.inertial_frame.angle.x;
        let pitch:f64 = self.state.inertial_frame.angle.y;
        let yaw:f64 = self.state.inertial_frame.angle.z;

        let R_roll:Matrix3<f64> = Matrix3::new(
            1.0, 0.0, 0.0,
            0.0, f64::cos(roll), f64::sin(roll),
            0.0, - f64::sin(roll), f64::cos(roll)
        );

        let R_pitch:Matrix3<f64> = Matrix3::new(
            f64::cos(pitch), 0.0, -f64::sin(pitch),
            0.0, 1.0, 0.0,
            f64::sin(pitch), 0.0, f64::cos(pitch)
        );

        let R_yaw: Matrix3<f64> = Matrix3::new(
            f64::cos(yaw), f64::sin(yaw), 0.0,
            -f64::sin(yaw), f64::cos(yaw), 0.0,
            0.0, 0.0, 1.0
        );
        let rot:Rotation3<f64> = Rotation3::new(self.state.inertial_frame.angle);
        return rot;
    }



    fn landed(&self) -> bool {
        self.state.inertial_frame.pos.z > 0.0
    }
}

