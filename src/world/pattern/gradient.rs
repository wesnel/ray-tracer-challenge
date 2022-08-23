use crate::{
    math::{Matrix, Point, Transformable},
    world::{Color, Textured},
};

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Gradient {
    a: Color,
    b: Color,
    pub transform: Matrix,
    pub inverse: Matrix,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Gradient {
        Gradient {
            a,
            b,
            transform: Matrix::identity(),
            inverse: Matrix::identity(),
        }
    }
}

impl Transformable for Gradient {
    fn transformed(self, transform: Matrix) -> Gradient {
        Gradient {
            a: self.a,
            b: self.b,
            transform,
            inverse: transform.inverse(),
        }
    }

    fn transform(&mut self, transform: Matrix) -> &mut Gradient {
        *self = self.transformed(transform);
        self
    }
}

impl Textured for Gradient {
    fn color_at(&self, object_space_point: Point) -> Color {
        let pattern_space_point = self.inverse * object_space_point;
        let distance = self.b - self.a;
        let fraction = pattern_space_point[0] - pattern_space_point[0].floor();
        self.a + distance * fraction
    }
}

impl Index<usize> for Gradient {
    type Output = Color;

    fn index(&self, i: usize) -> &Self::Output {
        unsafe { &std::mem::transmute::<&Gradient, &[Color; 2]>(self)[i] }
    }
}

impl IndexMut<usize> for Gradient {
    fn index_mut(&mut self, i: usize) -> &mut Color {
        unsafe { &mut std::mem::transmute::<&mut Gradient, &mut [Color; 2]>(self)[i] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_interpolation() {
        let pattern = Gradient::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(Point::zero()), Color::white());
        assert_eq!(
            pattern.color_at(Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
