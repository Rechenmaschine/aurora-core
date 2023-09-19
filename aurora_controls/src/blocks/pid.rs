use std::ops::{Add, Mul, Sub};
use crate::block::{Block, UpdateBlock};
use crate::blocks::differentiate::Differentiator;
use crate::blocks::integrate::Integrator;

pub struct PIDController<KP, KI, KD, I, D> {
    k_p: KP,
    k_i: KI,
    k_d: KD,
    integrator: I,
    differentiator: D
}

impl<KP, KI, KD, I, D> PIDController<KP, KI, KD, I, D> where I: Integrator, D: Differentiator  {
    pub fn new(k_p: KP, k_i: KI, k_d: KD, integrator: I, differentiator: D) -> Self {
        Self {
            k_p,
            k_i,
            k_d,
            integrator,
            differentiator
        }
    }
}

impl<R, Y, U, E, KP, KI, KD, I, D, IO, DO> Block<(R, Y), U> for PIDController<KP, KI, KD, I, D>
    where R: Copy + Sub<Y, Output = E>,
          E: Copy,
          I: Integrator + UpdateBlock<InputType = E, OutputType = IO>,
          D: Differentiator + UpdateBlock<InputType = E, OutputType = DO>,
          KP: Copy + Mul<E, Output = U>,
          KI: Copy + Mul<IO, Output = U>,
          KD: Copy + Mul<DO, Output = U>,
          U: Add<U, Output = U> {
    fn step(&mut self, input: (R, Y)) -> U {
        let (r, y) = input;
        let e = r - y;
        let p = self.k_p * e;
        let i = self.k_i * self.integrator.step(e);
        let d = self.k_d * self.differentiator.step(e);

        p + i + d
    }
}

