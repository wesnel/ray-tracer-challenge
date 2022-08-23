use crate::{
    math::{Matrix, Point, Transformable},
    world::{Color, Textured},
};

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ring {
    a: Color,
    b: Color,
    pub transform: Matrix,
    pub inverse: Matrix,
}

impl Ring {
    pub fn new(a: Color, b: Color) -> Ring {
        Ring {
            a,
            b,
            transform: Matrix::identity(),
            inverse: Matrix::identity(),
        }
    }
}

impl Transformable for Ring {
    fn transformed(self, transform: Matrix) -> Ring {
        Ring {
            a: self.a,
            b: self.b,
            transform,
            inverse: transform.inverse(),
        }
    }

    fn transform(&mut self, transform: Matrix) -> &mut Ring {
        *self = self.transformed(transform);
        self
    }
}

impl Textured for Ring {
    fn color_at(&self, object_space_point: Point) -> Color {
        let pattern_space_point = self.inverse * object_space_point;
        self[((pattern_space_point[0] * pattern_space_point[0]
            + pattern_space_point[2] * pattern_space_point[2])
            .sqrt()
            .rem_euclid(2.0)
            .floor()) as usize]
    }
}

impl Index<usize> for Ring {
    type Output = Color;

    fn index(&self, i: usize) -> &Self::Output {
        unsafe { &std::mem::transmute::<&Ring, &[Color; 2]>(self)[i] }
    }
}

impl IndexMut<usize> for Ring {
    fn index_mut(&mut self, i: usize) -> &mut Color {
        unsafe { &mut std::mem::transmute::<&mut Ring, &mut [Color; 2]>(self)[i] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_extends_in_x_and_z() {
        let pattern = Ring::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(Point::zero()), Color::white());
        assert_eq!(pattern.color_at(Point::new(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 1.0)), Color::black());
        assert_eq!(
            pattern.color_at(Point::new(0.708, 0.0, 0.708)),
            Color::black()
        );
    }
}
