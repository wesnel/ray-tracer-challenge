use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use super::EPSILON;

/// 4-dimensional vector which always has a fourth component of 0.
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Vector {
        Vector::new(1.0, 1.0, 1.0)
    }

    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Vector {
        self / self.magnitude()
    }

    pub fn normalize(&mut self) -> &mut Vector {
        *self = self.normalized();
        self
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    /// reflect this vector across another vector
    pub fn reflect_across(self, vector: Vector) -> Vector {
        self - (vector * 2.0 * self.dot(&vector))
    }
}

/* equality operation */

impl PartialEq for Vector {
    /// test for equality using approximate comparison of floating point numbers.
    fn eq(&self, other: &Self) -> bool {
        (self[0] - other[0]).abs() < EPSILON
            && (self[1] - other[1]).abs() < EPSILON
            && (self[2] - other[2]).abs() < EPSILON
    }
}

/* indexing operations */

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        unsafe { &std::mem::transmute::<&Vector, &[f64; 3]>(self)[i] }
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        unsafe { &mut std::mem::transmute::<&mut Vector, &mut [f64; 3]>(self)[i] }
    }
}

/* scalar operations */

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector::new(self[0] * scalar, self[1] * scalar, self[2] * scalar)
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scalar: f64) {
        self[0] *= scalar;
        self[1] *= scalar;
        self[2] *= scalar;
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Vector::new(self[0] / scalar, self[1] / scalar, self[2] / scalar)
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, scalar: f64) {
        self[0] /= scalar;
        self[1] /= scalar;
        self[2] /= scalar;
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector::new(-self[0], -self[1], -self[2])
    }
}

/* vector operations */

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_vectors() {
        let a1 = Vector::new(3.0, -2.0, 5.0);
        let a2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = Vector::zero();
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_vector() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-a, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(a * 3.5, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn multiply_vector_by_fraction() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(a * 0.5, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn divide_vector_by_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(a / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn magnitude_of_x() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_y() {
        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_z() {
        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_positive_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), f64::from(14.0).sqrt());
    }

    #[test]
    fn magnitude_of_negative_vector() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), f64::from(14.0).sqrt());
    }

    #[test]
    fn normalize_single_component_vector() {
        let v = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(v.normalized(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(
            v.normalized(),
            Vector::new(
                1.0 / f64::from(14).sqrt(),
                2.0 / f64::from(14).sqrt(),
                3.0 / f64::from(14).sqrt(),
            ),
        );
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let normalized = v.normalized();
        assert_eq!(normalized.magnitude(), 1.0);
    }

    #[test]
    fn dot_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflect_45_degrees() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = v.reflect_across(n);
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_off_slant() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(
            f64::from(2.0).sqrt() / 2.0,
            f64::from(2.0).sqrt() / 2.0,
            0.0,
        );
        let r = v.reflect_across(n);
        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}
