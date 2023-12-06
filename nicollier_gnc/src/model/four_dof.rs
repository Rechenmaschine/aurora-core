//
//     _    ____  ___ ____
//    / \  |  _ \|_ _/ ___|
//   / _ \ | |_) || |\___ \
//  / ___ \|  _ < | | ___) |
// /_/   \_\_| \_\___|____/
//by GNC Crew
use crate::model::four_dof::model_params_parachute::{
    CANOPY_AREA, C_D_0, C_D_SYM_DEFLECTION, C_L_0, C_L_SYM_DEFLECTION, K_ROLL, MASS, T_ROLL,
};
use crate::model::four_dof::natural_constants::{AIR_DENSITY, GRAV};
use crate::model::Model;
use crate::{Deflections, SystemState};
use nalgebra::{Rotation3, Vector3};

pub struct FourDof {
    state: SystemState,
    wind: Vector3<f64>, //definition von wind vektor vom type vector3<f64>, relative to inertial frame
}
// Definition of Parameters, how to access: e.g. let gravitational_acceleration = natural_constants::GRAV;
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
            state,                             //nix gut diese
            wind: Vector3::new(0.0, 0.0, 0.0), //two-dimensional vector at 3dof, but here three
        }
    }
    pub fn set_wind(&mut self, wind: Vector3<f64>) {
        self.wind = wind;
    }

    fn get_airspeed(&self) -> f64 {
        //(self.wind+self.state.body_frame_velocity).norm()//norm is the length of the vector, calculates airspeed from wind and body frame velocity
        (self.state.body_frame_velocity.x * self.state.body_frame_velocity.x
            + self.state.body_frame_velocity.z * self.state.body_frame_velocity.z)
            .sqrt()
    }
    fn get_alpha(&self) -> f64 {
        //get alpha, rotation is from inertial to body
        let body_air_speed = self.state.body_frame_velocity; //+Rotation3::new(self.state.inertial_frame_angle)*self.wind*0; //peri had no wind included, but copilot said it should be included, makes sense because drag and lift depend on airspeed
        body_air_speed.z.atan2(body_air_speed.x)
    }
    fn get_force(&self, sym_deflections: f64) -> Vector3<f64> {
        //calculate the force vector which is acting on the parachute, unsure
        //get the Force
        let mut ret: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
        let airspeed = self.get_airspeed();
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
        //ret=Rotation3::new(self.state.inertial_frame_angle)*ret;//dtp=difference to periphas
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
        //wieso muss hier Self gorssgeschrieben werden

        //if self.state.inertial_frame_position.z< -1400.0{
        //    self.state.inertial_frame_position.z=-1200.0;
        //}
        //update accelerations (at the body frame)
        self.state.body_frame_acceleration.x = (1.0 / MASS) * self.get_force(input.sym).x
            - self.state.body_frame_velocity.z
                * self.state.inertial_frame_angle_velocity.z
                * self.state.inertial_frame_angle.x.sin();
        self.state.body_frame_acceleration.y = 0.0; //could be changed (1.0 / MASS) * self.get_force(input.sym).y;//not like peri
        self.state.body_frame_acceleration.z = (1.0 / MASS) * self.get_force(input.sym).z
            + self.state.body_frame_velocity.x
                * self.state.inertial_frame_angle_velocity.z
                * self.state.inertial_frame_angle.x.sin();

        self.state.inertial_frame_angle_velocity.x =
            (K_ROLL * input.asym - self.state.inertial_frame_angle.x) / T_ROLL;
        self.state.inertial_frame_angle_velocity.y = 0.0;
        //self.state.inertial_frame_angle_velocity.z=0.0;
        if self.state.body_frame_velocity.x * self.state.inertial_frame_angle.x.cos().abs()//this line is not in peri, but otherwise there is a divide by zero, how did peri handle this?, how does this line make any sense?
            != 0.0
        {
            self.state.inertial_frame_angle_velocity.z = (1.0 / MASS) * self.get_force(input.sym).y
                / (self.state.body_frame_velocity.x * self.state.inertial_frame_angle.x.cos())
                + (self.state.body_frame_velocity.z * self.state.inertial_frame_angle_velocity.x)
                    / (self.state.body_frame_velocity.x * self.state.inertial_frame_angle.x.cos());
            //wouldnt it be better to use the intertial frame velocity here?, i didnt quite get this
        }

        self.state.inertial_frame_acceleration = Rotation3::new(self.state.inertial_frame_angle)
            .transpose()
            * self.state.body_frame_acceleration;
        //println!("input:{:?},inertial_acceleration{:?}",input,self.state.inertial_frame_acceleration);
        println!("z:{}", self.state.inertial_frame_position.z);
        //update angular velocities(inertial frame) //isnt this body frame

        //println!("inertial angle velocity z{}",self.state.inertial_frame_angle_velocity.z);
        println!(
            "force:{}, body frame vel.x:{}, body frame vel.z:{}inertial frame angle cos:{}",
            self.get_force(input.sym).y,
            self.state.body_frame_velocity.x,
            self.state.body_frame_velocity.z,
            self.state.inertial_frame_angle.x.cos()
        );

        //integrate accelerations(body frame)

        self.state.body_frame_velocity += delta_t * self.state.body_frame_acceleration;

        //change frames for velocities, from body to inertial frame,transpose=inverse of rotation matrix, so we are going from body to inertial frame
        self.state.inertial_frame_velocity = Rotation3::new(self.state.inertial_frame_angle)
            .transpose()
            * self.state.body_frame_velocity;
        //println!("{}",self.state.inertial_frame_acceleration.z);
        //Integrate Velocities (inertial frame)
        self.state.inertial_frame_position += delta_t * self.state.inertial_frame_velocity;
        //integrate Angular velocities (inertial frame)
        self.state.inertial_frame_angle += delta_t * self.state.inertial_frame_angle_velocity;
        //we are joining swissloop due to stall(rocket is falling down)
        if self.state.body_frame_velocity.x < 0.02 {
            //println!("the total forward velocity is lower than 0.02m/s, the speed relative to the wind is {}: we could be joining swissloop due to stall", self.state.body_frame_velocity.dot(&(self.wind - self.state.body_frame_velocity)) / self.state.body_frame_velocity.norm());//unsure if my calculation is correct
        }
        //we could be flying hearts or other symbols as our energy phase, would be dope
        //the roll angle almost at 90 degrees, we are doing a flip, wuuuhuuu
        if self.state.inertial_frame_angle.x.cos() < 0.2 {
            //println!("the roll angle is almost at 90 degrees, the angle relative to the wind is {}: we my be doing a flip, wuuuhuuu", "unknown");
        }
        //self.state.inertial_frame_position.z-=10.0;
        self.state.total_time += delta_t;
        return self.state; //wieso
    }
    fn landed(&self) -> bool {
        //why is here a ref
        if self.state.inertial_frame_position.z > 0.0 {
            println!("landed,z: {}", self.state.inertial_frame_position.z);
        }
        self.state.inertial_frame_position.z > 0.0 //&& self.state.inertial_frame_velocity.norm()<0.1
    }
}
