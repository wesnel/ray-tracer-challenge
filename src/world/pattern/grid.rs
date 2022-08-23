use crate::{
    math::{Matrix, Point, Transformable},
    world::{Color, Textured},
};

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Grid {
    a: Color,
    b: Color,
    pub transform: Matrix,
    pub inverse: Matrix,
}

impl Grid {
    pub fn new(a: Color, b: Color) -> Grid {
        Grid {
            a,
            b,
            transform: Matrix::identity(),
            inverse: Matrix::identity(),
        }
    }
}

impl Transformable for Grid {
    fn transformed(self, transform: Matrix) -> Grid {
        Grid {
            a: self.a,
            b: self.b,
            transform,
            inverse: transform.inverse(),
        }
    }

    fn transform(&mut self, transform: Matrix) -> &mut Grid {
        *self = self.transformed(transform);
        self
    }
}

impl Textured for Grid {
    fn color_at(&self, object_space_point: Point) -> Color {
        let pattern_space_point = self.inverse * object_space_point;
        self[((pattern_space_point[0].floor()
            + pattern_space_point[1].floor()
            + pattern_space_point[2].floor())
        .rem_euclid(2.0)
        .floor()) as usize]
    }
}

impl Index<usize> for Grid {
    type Output = Color;

    fn index(&self, i: usize) -> &Self::Output {
        unsafe { &std::mem::transmute::<&Grid, &[Color; 2]>(self)[i] }
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, i: usize) -> &mut Color {
        unsafe { &mut std::mem::transmute::<&mut Grid, &mut [Color; 2]>(self)[i] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeats_in_x() {
        let pattern = Grid::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(Point::zero()), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(1.01, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn repeats_in_y() {
        let pattern = Grid::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(Point::zero()), Color::white());
        assert_eq!(
            pattern.color_at(Point::new(0.0, 0.99, 0.00)),
            Color::white()
        );
        assert_eq!(
            pattern.color_at(Point::new(0.0, 1.01, 0.00)),
            Color::black()
        );
    }

    #[test]
    fn repeats_in_z() {
        let pattern = Grid::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(Point::zero()), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 1.01)), Color::black());
    }
}
