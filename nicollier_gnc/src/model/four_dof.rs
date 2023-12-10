use std::arch::x86_64::_xgetbv;
use std::f64::consts::PI;
use crate::model::four_dof::model_params_parachute::{
    CANOPY_AREA, C_D_0, C_D_SYM_DEFLECTION, C_L_0, C_L_SYM_DEFLECTION, K_ROLL, MASS, T_ROLL,
};
use crate::model::four_dof::natural_constants::{AIR_DENSITY, GRAV};
use crate::model::Model;
use crate::{Deflections, SystemState,get_wind};
use nalgebra::{Rotation2, Rotation3, Vector2, Vector3};

pub struct FourDof {
    state: SystemState,
}
// Definition of Parameters, parameter needs to be included in the use statement above
mod model_params_parachute {
    // System parameters
    pub const CANOPY_AREA: f64 = 10.094; // [m²]
    pub const MASS: f64 = 50.0; // [kg]

    // Aerodynamic Coefficients

    // The following coefficients are determined by SysId
    pub const C_L_0: f64 = 0.4360; // zero-lift coefficient
    pub const C_L_SYM_DEFLECTION: f64 = 0.0065; // Lift-Coefficient derivative w.r.t. symmetric deflection
    pub const C_D_0: f64 = 0.2034; // zero-drag coefficient
    pub const C_D_SYM_DEFLECTION: f64 = 0.0000; // Drag-Coefficient derivative w.r.t. symmetric deflection
    pub const T_ROLL: f64 = 1.7203; // [s] roll-model time constant
    pub const K_ROLL: f64 = 0.5871; // [rad] roll-model gain
}
mod natural_constants {
    pub const GRAV: f64 = 9.81; // [m/s²] gravitational acceleration
    pub const AIR_DENSITY: f64 = 1.225; // [kg/m³]
}

impl FourDof {

    pub fn new(state: SystemState) -> Self {
        Self {
            state,
        }
    }
    fn get_inertial_airspeed(&self) -> Vector3<f64>{
        self.state.inertial_frame_velocity-get_wind(self.state.inertial_frame_position.z)
    }
    fn get_alpha(&self) -> f64 {
        let body_airvector= Rotation3::new(self.state.inertial_frame_angle)*self.get_inertial_airspeed();
        body_airvector.z.atan2(body_airvector.x)
    }
    fn get_force(&self, sym_deflections: f64) -> Vector3<f64> {
        //calculate the force vector which is acting on the parachute, unsure
        //get the Force
        let mut ret: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
        let body_airvector= Rotation3::new(self.state.inertial_frame_angle)*self.get_inertial_airspeed();
        let airspeed = (body_airvector.x * body_airvector.x + body_airvector.z * body_airvector.z).sqrt();
        let alpha = self.get_alpha();
        let roll = self.state.inertial_frame_angle.x;
        // calcultate the lift(L) and drag (D) coefficients
        let lift: f64 = 0.5
            * AIR_DENSITY
            * CANOPY_AREA
            * airspeed
            * airspeed
            * (C_L_0 + C_L_SYM_DEFLECTION * sym_deflections);
        let drag: f64 = 0.5
            * AIR_DENSITY
            * CANOPY_AREA
            * airspeed
            * airspeed
            * (C_D_0 + C_D_SYM_DEFLECTION * sym_deflections);
        //calculate the force vector
        ret.x = lift * alpha.sin() - drag * alpha.cos();
        ret.y = MASS * GRAV * roll.sin();
        ret.z = -lift * alpha.cos() - drag * alpha.sin() + MASS * GRAV * roll.cos();
        //ret=Rotation3::new(self.state.inertial_frame_angle)*ret;//different to periphase
        return ret;
    }
}
impl Model for FourDof {
    type State = SystemState;
    type Input = Deflections; //asym und sym
    fn get_state(&self) -> Self::State {
        self.state //letzte Zeile wird in Rust automatisch ausgegeben, wenn das semikolon fehlt  ->
    }
    fn step(&mut self, input: Self::Input, delta_t: f64) -> Self::State {
        //update accelerations (at the body frame)
        self.state.body_frame_acceleration.x = (1.0 / MASS) * self.get_force(input.sym).x
            - self.state.body_frame_velocity.z
                * self.state.inertial_frame_angle_velocity.z
                * self.state.inertial_frame_angle.x.sin();
        self.state.body_frame_acceleration.y = 0.0;
        self.state.body_frame_acceleration.z = (1.0 / MASS) * self.get_force(input.sym).z
            + self.state.body_frame_velocity.x
                * self.state.inertial_frame_angle_velocity.z
                * self.state.inertial_frame_angle.x.sin();
        //update angular velocities (at the inertial frame)
        self.state.inertial_frame_angle_velocity.x =
            (K_ROLL * input.asym - self.state.inertial_frame_angle.x) / T_ROLL;
        self.state.inertial_frame_angle_velocity.y = 0.0;
        if self.state.body_frame_velocity.x.abs() * self.state.inertial_frame_angle.x.cos().abs() > f64::EPSILON{
            self.state.inertial_frame_angle_velocity.z = (1.0 / MASS) * self.get_force(input.sym).y//=1/mass*Fg*tan(roll)/v_body_x
                / (self.state.body_frame_velocity.x * self.state.inertial_frame_angle.x.cos())
                 +(self.state.body_frame_velocity.z * self.state.inertial_frame_angle_velocity.x)//body_v_Z
                / (self.state.body_frame_velocity.x * self.state.inertial_frame_angle.x.cos()); //wouldnt it be better to use the intertial frame velocity here?, i didnt quite get this
        }else{
            println!("the absolut body frame velocity x is {} and absolut roll cos is {}", self.state.body_frame_velocity.x.abs(), self.state.inertial_frame_angle.x.cos().abs());
        }

        self.state.body_frame_velocity += delta_t * self.state.body_frame_acceleration; //integrate acceleration(body frame)
        self.state.inertial_frame_velocity = Rotation3::new(self.state.inertial_frame_angle).transpose() * self.state.body_frame_velocity;//change frames for velocities, from body to inertial frame,transpose=inverse of rotation matrix, so we are going from body to inertial frame
        self.state.inertial_frame_position += delta_t * self.state.inertial_frame_velocity; //Integrate Velocities (inertial frame)
        self.state.inertial_frame_angle += delta_t * self.state.inertial_frame_angle_velocity; //integrate Angular velocities (inertial frame)


        //we are joining swissloop due to stall(rocket is falling down)
        if self.state.body_frame_velocity.x < 0.02 || (Rotation3::new(self.state.inertial_frame_angle)*self.get_inertial_airspeed()).x < 0.02 {
            println!("the body_frame_velocity.x is {}, the speed relative to the wind is {}: we could be joining swissloop due to stall", self.state.body_frame_velocity.x, (Rotation3::new(self.state.inertial_frame_angle)*self.get_inertial_airspeed()).x); //self.state.body_frame_velocity.dot(&(self.wind - self.state.body_frame_velocity)) / self.state.body_frame_velocity.norm());
        }
        //the roll angle almost at 90 degrees, we are doing a flip
        if self.state.inertial_frame_angle.x.cos() < 0.2 {
            println!("the roll angle is almost at 90 degrees, the angle relative to the wind is {}: we might be doing a flip", "unknown");
        }
        self.state.total_time += delta_t;
        return self.state;
    }
    fn landed(&self) -> bool {

        println!("wind at the groud {:?},{}", get_wind(self.state.inertial_frame_position.z),self.state.inertial_frame_position.z);
        self.state.inertial_frame_position.z > 0.0 //&& self.state.inertial_frame_velocity.norm()<0.1
    }
}
