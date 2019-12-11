pub mod rigidbody;
pub mod physworld;
pub mod collider;
pub mod material;

extern crate nphysics2d;
extern crate nalgebra;
extern crate ncollide2d;

use std::convert::From;

use nalgebra::RealField;

pub trait Number : From<f64> + RealField {}
impl<T> Number for T where T: From<f64> + RealField {}