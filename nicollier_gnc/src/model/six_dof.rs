//dont know if I get it finished until christmas
/*
use std::arch::x86_64::_xgetbv;
use std::f64::consts::PI;
use crate::model::six_dof::model_params::*;
use crate::model::Model;
use crate::{Deflections, SystemState,get_wind};
use nalgebra::{Rotation2, Rotation3, Vector2, Vector3, Matrix3, Vector6, Matrix6,LU,DMatrix};

pub struct SixDof {
    state: SystemState,
}
// Definition of Parameters, parameter needs to be included in the use statement above
mod model_params {
    use nalgebra::{Vector3, Matrix3, DMatrix, OMatrix, Dyn};
    //natural constants
    pub const GRAV: f64 = 9.81; // [m/s²] gravitational acceleration
    pub const AIR_DENSITY: f64 = 1.225; // [kg/m³]
    // Parachute parameters
    pub const CANOPY_AREA: f64 = 10.094;
    pub const MASS: f64 = 36.4;
    pub const CANOPY_SPAN: f64 = 4.624;
    pub const CANOPY_CHORD: f64 = 2.183;
    pub const CANOPY_THICKNESS: f64 = 0.200;
    pub const RIGGING_ANGLE: f64 = -0.2094; // -12 deg

    // Position vector for R_BM (Body Mass Center?)
    pub fn m_vector_mc_amc() -> Vector3<f64> {
        Vector3::new(0.5511, 0.0, -3.4796)//R_BM_X, R_BM_Y, R_BM_Z
    }//may be mutable

    // Inertia tensor (must be symmetric)
    pub fn m_inertia() -> Matrix3<f64> {
        Matrix3::new(
            18.608,  0.0005,  0.0034,
            0.0005, 18.6083, -0.00003,
            0.0034, -0.00003, 0.2435
        )
    }

    pub fn m_apparent_mass() -> OMatrix<f64, Dyn, Dyn> {//the type is kinda weird, i think it stands for dynamic sizing or something
        DMatrix::from_diagonal(&Vector3::new(0.1642,2.0841,14.8661))
    }//may be mutable! <-----------

    m_ApparentInertia.diagonal() << I_AI_XX, I_AI_YY, I_AI_ZZ;
    pub fn m_apparent_inertia()->OMatrix<f64,Dyn,Dyn>{
        DMatrix::from_diagonal(&Vector3::new(1.2705,2.6937,0.3400))//I_AI_XX, I_AI_YY, I_AI_ZZ
    }//may be mutable


    // Aerodynamic coefficients obtained using SysID
    pub const C_D_0: f64 = 0.1543;
    pub const C_D_ALPHA_SQUARED: f64 = -0.1445;
    pub const C_D_DELTA_S: f64 = 0.0502;
    pub const C_Y_BETA: f64 = -0.7570;
    pub const C_L_0: f64 = 0.3005;
    pub const C_L_ALPHA: f64 = 0.1815;
    pub const C_M_0: f64 = 0.3016;
    pub const C_M_ALPHA: f64 = -0.7879;
    pub const C_M_Q: f64 = -0.7906;
    pub const C_L_BETA: f64 = -0.1218;
    pub const C_L_P: f64 = -1.4232;
    pub const C_L_R: f64 = -0.0313;
    pub const C_L_DELTA_A: f64 = 0.0033;
    pub const C_L_DELTA_S: f64 = 0.0;
    pub const C_N_BETA: f64 = -0.0695;
    pub const C_N_P: f64 = -0.8029;
    pub const C_N_R: f64 = -0.0553;
    pub const C_N_DELTA_A: f64 = 0.0070;
}


impl SixDof {
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

        return ret;
    }
    fn getA(&self)->Matrix6<f64>{
        let eye = Matrix3::<f64>::identity();

        //let A= Matrix6::<f64>::zeros(); //TODO
        let A=Matrix6::<f64>new(//inertia matrix, its adding 3*3 matrices, z-form starting from the top left
            (MASS) * eye + change_tensor_frame(m_ApparentMass),
            -change_tensor_frame(m_ApparentMass) * skew_vector_bm(),
            skew_vector_bm() * change_tensor_frame(m_ApparentMass),
            m_Inertia + change_tensor_frame(m_ApparentInertia) - skew_vector_bm() * change_tensor_frame(m_ApparentMass) * skew_vector_bm(),
        )
        return A;
    }
    fn getB(&self)->Vector6<f64>{
        Vector6::<f64>::zeros() //TODO
    }

}
impl Model for SixDof {
    type State = SystemState;
    type Input = Deflections; //asym und sym
    fn get_state(&self) -> Self::State {
        self.state //letzte Zeile wird in Rust automatisch ausgegeben, wenn das semikolon fehlt  ->
    }
    fn step(&mut self, input: Self::Input, delta_t: f64) -> Self::State {
        //all computations are done in the body frame

        //update Accelerations
        //solve linear system A*x=b, where x are the Accelerations, A is mass/inertia and b are the forces
        //let x=Vector6::<f64>::zeros();//angular and linear accelerations
        let A=self.getA();
        let B=self.getB();
        let mut x:Vector6<f64> = Default::default();//what is the default value of a vector?->0.0 ...
        match A.lu().solve(&B) {
            Some(solution) => {
                x=solution;
            },
            None => {
                println!("No solution could be found, or the matrix is singular.");
            }
        }
        self.state.body_frame_acceleration=x[0..2];
        self.state.body_frame_angle_acceleration=x[3..5];
        //update body frame velocities with Euler forward,
        self.state.body_frame_velocity+=self.state.body_frame_acceleration*delta_t;
        self.state.body_frame_angle_velocity+=self.state.body_frame_angle_acceleration*delta_t;
        //change to inertial frame
        self.state.inertial_frame_velocity=Rotation3::new(self.state.inertial_frame_angle)*self.state.body_frame_velocity;
        self.state.inertial_frame_angle_velocity=Rotation3::new(self.state.inertial_frame_angle)*self.state.body_frame_angle_velocity;
        //update positions with Euler forward
        self.state.inertial_frame_position+=self.state.inertial_frame_velocity*delta_t;
        self.state.inertial_frame_angle+=self.state.inertial_frame_angle_velocity*delta_t;


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
*/