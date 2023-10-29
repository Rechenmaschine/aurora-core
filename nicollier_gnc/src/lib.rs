mod model;
mod controller;
mod guidance;


use crate::controller::p_controller::PController;
use crate::controller::Controller;
use crate::guidance::constant_guidance::ConstantGuidance;
use crate::guidance::Guidance;
use crate::model::three_dof::ThreeDof;
use crate::model::Model;
use anyhow::Result;
use nalgebra::{Vector2, Vector3};
use std::f64::consts::PI;
use std::ops::{Deref, DerefMut};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize)]
pub struct SystemState {
    pos: Vector3<f64>,
    angle: Vector3<f64>,
    angle_velocity: Vector3<f64>,
    total_time: f64
}

#[derive(Copy, Clone, Debug)]
pub struct Deflections {
    sym: f64,
    asym: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Reference(f64);



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
