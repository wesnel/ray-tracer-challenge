use std::ops::{Add, Index, IndexMut, Sub};

use super::vector::Vector;

/// 4-dimensional vector which always has a fourth component of 1.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(Vector);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(Vector::new(x, y, z))
    }

    pub fn zero() -> Point {
        Point::new(0.0, 0.0, 0.0)
    }

    pub fn with_translation(t: Vector) -> Point {
        Point(t)
    }
}

/* indexing operations */

impl Index<usize> for Point {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.0[i]
    }
}

/* point-vector operations */

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, vector: Vector) -> Self::Output {
        Point::with_translation(self.0 + vector)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, vector: Vector) -> Self::Output {
        Point::with_translation(self.0 - vector)
    }
}

/* point-point operations */

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        self.0 - other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtract_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }
}
