use nalgebra::{Vector2, Vector3};

pub mod constant_guidance;
//pub mod constant_yaw;




pub trait Guidance {
    type State;
    type Reference;

    fn get_reference(&mut self, state: Self::State) -> Self::Reference;
}
