pub mod constant_guidance;

pub mod constant_yaw;
pub mod double_wall;
pub mod superior_double_wall;

pub trait Guidance {
    type State;
    type Reference;

    fn get_reference(&mut self, state: Self::State) -> Self::Reference;
}
