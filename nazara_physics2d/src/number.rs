use std::convert::From;

use nalgebra::RealField;

pub trait Number : From<f64> + RealField {}
impl<T> Number for T where T: From<f64> + RealField {}