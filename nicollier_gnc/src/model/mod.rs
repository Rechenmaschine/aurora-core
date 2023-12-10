pub mod three_dof;
pub mod four_dof;
mod six_dof;

pub trait Model {
    type State;
    type Input;

    fn get_state(&self) -> Self::State;

    fn step(&mut self, input: Self::Input, delta_t: f64) -> Self::State;

    fn landed(&self) -> bool;
}
