use nalgebra::Vector2;
use crate::guidance::Guidance;
use crate::{Reference, SystemState};

pub struct ConstantGuidance(Reference);

impl ConstantGuidance {
    pub fn new(r: Reference) -> Self {
        ConstantGuidance(r)
    }
}
const nodeOne:Vector2<f64> = Vector2::new(100.0,100.0);
const nodeTwo:Vector2<f64> = Vector2::new(-100.0,-100.0);


impl Guidance for ConstantGuidance {
    type State = SystemState;
    type Reference = Reference;



    fn get_reference(&mut self, state: Self::State) -> Self::Reference {

        self.0
    }
}
