pub mod geometry;
pub use geometry::{Form, Geometry, Hittable, Transformable};

pub mod matrix;
pub use matrix::Matrix;

pub mod point;
pub use point::Point;

pub mod vector;
pub use vector::Vector;

/// value for minimum floating point precision; used for approximate equality checks.
pub const EPSILON: f64 = 0.0001;

pub fn clamp_between(to_clamp: f64, min: f64, max: f64) -> f64 {
    if to_clamp < min {
        min
    } else if max < to_clamp {
        max
    } else {
        to_clamp
    }
}

pub fn change_interval(
    to_change: f64,
    (old_min, old_max): (f64, f64),
    (new_min, new_max): (f64, f64),
) -> f64 {
    new_min + (((new_max - new_min) / (old_max - old_min)) * (to_change - old_min))
}
