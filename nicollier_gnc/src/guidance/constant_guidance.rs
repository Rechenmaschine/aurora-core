use std::f32::consts::PI;
use nalgebra::{Rotation2, Vector2, Vector3};
use crate::guidance::Guidance;
use crate::{Reference, SystemState};

pub struct ConstantGuidance(Reference);

impl ConstantGuidance {
    pub fn new(r: Reference) -> Self {
        ConstantGuidance(r)
    }
}

const TARGET:Vector2<f64> = Vector2::new(100.0, 100.0);

impl Guidance for ConstantGuidance {
    type State = SystemState;
    type Reference = Reference;



    fn get_reference(&mut self, state: Self::State) -> Self::Reference {
        let relative_vector:Vector2<f64> = Vector2::new(TARGET.x - state.inertial_frame_position.x, TARGET.y - state.inertial_frame_position.y);

        let heading = Rotation2::rotation_between(&relative_vector,&Vector2::x());


        self.0 = Reference(-heading.angle());
        self.0
    }
}
