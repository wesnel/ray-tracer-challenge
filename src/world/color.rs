use std::{
    f64,
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::math::{change_interval, clamp_between, Vector};

pub const MIN_COLOR: f64 = 0.0;
pub const MAX_COLOR: f64 = 255.0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(Vector);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vector::new(r, g, b))
    }

    pub fn from_vector(vector: Vector) -> Color {
        Color(vector)
    }

    pub fn black() -> Color {
        Color::from_vector(Vector::zero())
    }

    pub fn white() -> Color {
        Color::from_vector(Vector::ones())
    }

    pub fn red(&self) -> f64 {
        self.0[0]
    }

    pub fn green(&self) -> f64 {
        self.0[1]
    }

    pub fn blue(&self) -> f64 {
        self.0[2]
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            change_interval(
                clamp_between(self.red(), 0.0, 1.0),
                (0.0, 1.0),
                (MIN_COLOR, MAX_COLOR)
            )
            .round() as i64,
            change_interval(
                clamp_between(self.green(), 0.0, 1.0),
                (0.0, 1.0),
                (MIN_COLOR, MAX_COLOR)
            )
            .round() as i64,
            change_interval(
                clamp_between(self.blue(), 0.0, 1.0),
                (0.0, 1.0),
                (MIN_COLOR, MAX_COLOR)
            )
            .round() as i64,
        )
    }
}

/* indexing operations */

impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.0[i]
    }
}

/* scalar operations */

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Color::from_vector(self.0 * scalar)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, scalar: f64) {
        self.0 *= scalar;
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Color::from_vector(self.0 / scalar)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, scalar: f64) {
        self.0 /= scalar;
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Color::from_vector(-self.0)
    }
}

/* color operations */

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Color::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Self) {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Color::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_vectors() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn add_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_two_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
