use crate::guidance::Guidance;
use crate::{Reference, SystemState};

pub struct ConstantGuidance(Reference);

impl ConstantGuidance {
    pub fn new(r: Reference) -> Self {
        ConstantGuidance(r)
    }
}

impl Guidance for ConstantGuidance {
    type State = SystemState;
    type Reference = Reference;

    fn get_reference(&mut self, state: Self::State) -> Self::Reference {
        self.0
    }
}
