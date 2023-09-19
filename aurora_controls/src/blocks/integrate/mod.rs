mod forward_euler;

pub use self::forward_euler::ForwardEulerIntegrator;

pub trait Integrator {}