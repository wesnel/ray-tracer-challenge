use crate::{
    math::{Matrix, Point, Transformable},
    world::{Color, Textured},
};

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Stripe {
    a: Color,
    b: Color,
    pub transform: Matrix,
    pub inverse: Matrix,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe {
            a,
            b,
            transform: Matrix::identity(),
            inverse: Matrix::identity(),
        }
    }
}

impl Transformable for Stripe {
    fn transformed(self, transform: Matrix) -> Stripe {
        Stripe {
            a: self.a,
            b: self.b,
            transform,
            inverse: transform.inverse(),
        }
    }

    fn transform(&mut self, transform: Matrix) -> &mut Stripe {
        *self = self.transformed(transform);
        self
    }
}

impl Textured for Stripe {
    fn color_at(&self, object_space_point: Point) -> Color {
        let pattern_space_point = self.inverse * object_space_point;
        self[(pattern_space_point[0].rem_euclid(2.0).floor()) as usize]
    }
}

impl Index<usize> for Stripe {
    type Output = Color;

    fn index(&self, i: usize) -> &Self::Output {
        unsafe { &std::mem::transmute::<&Stripe, &[Color; 2]>(self)[i] }
    }
}

impl IndexMut<usize> for Stripe {
    fn index_mut(&mut self, i: usize) -> &mut Color {
        unsafe { &mut std::mem::transmute::<&mut Stripe, &mut [Color; 2]>(self)[i] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Color, Color) {
        (Color::black(), Color::white())
    }

    #[test]
    fn stripe_is_constant_in_y() {
        let (black, white) = setup();
        let stripe = Stripe::new(white, black);
        assert_eq!(stripe.color_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(stripe.color_at(Point::new(0.0, 1.0, 0.0)), white);
        assert_eq!(stripe.color_at(Point::new(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn stripe_is_constant_in_z() {
        let (black, white) = setup();
        let stripe = Stripe::new(white, black);
        assert_eq!(stripe.color_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(stripe.color_at(Point::new(0.0, 0.0, 1.0)), white);
        assert_eq!(stripe.color_at(Point::new(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn stripe_alternates_in_x() {
        let (black, white) = setup();
        let stripe = Stripe::new(white, black);
        assert_eq!(stripe.color_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(stripe.color_at(Point::new(0.9, 0.0, 0.0)), white);
        assert_eq!(stripe.color_at(Point::new(1.0, 0.0, 0.0)), black);
        assert_eq!(stripe.color_at(Point::new(-0.1, 0.0, 0.0)), black);
        assert_eq!(stripe.color_at(Point::new(-1.0, 0.0, 0.0)), black);
        assert_eq!(stripe.color_at(Point::new(-1.1, 0.0, 0.0)), white);
    }

    #[test]
    fn default_transformation() {
        let (black, white) = setup();
        let stripe = Stripe::new(white, black);
        assert_eq!(stripe.transform, Matrix::identity());
    }

    #[test]
    fn transformable() {
        let (black, white) = setup();
        let stripe = Stripe::new(white, black).transformed(Matrix::translation(1.0, 2.0, 3.0));
        assert_eq!(stripe.transform, Matrix::translation(1.0, 2.0, 3.0));
    }
}
