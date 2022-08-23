use crate::{
    math::{Matrix, Point, Transformable},
    world::{Color, Textured},
};

pub mod gradient;
pub use gradient::Gradient;

pub mod grid;
pub use grid::Grid;

pub mod ring;
pub use ring::Ring;

pub mod solid;
pub use solid::Solid;

pub mod stripe;
pub use stripe::Stripe;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pattern {
    Gradient(Gradient),
    Grid(Grid),
    Ring(Ring),
    Solid(Solid),
    Stripe(Stripe),
}

impl Pattern {
    pub fn gradient(gradient: Gradient) -> Pattern {
        Pattern::Gradient(gradient)
    }

    pub fn grid(grid: Grid) -> Pattern {
        Pattern::Grid(grid)
    }

    pub fn ring(ring: Ring) -> Pattern {
        Pattern::Ring(ring)
    }

    pub fn solid(color: Color) -> Pattern {
        Pattern::Solid(Solid::new(color))
    }

    pub fn stripe(stripe: Stripe) -> Pattern {
        Pattern::Stripe(stripe)
    }
}

impl Transformable for Pattern {
    fn transformed(self, transform: Matrix) -> Pattern {
        match self {
            Pattern::Gradient(gradient) => Pattern::gradient(gradient.transformed(transform)),
            Pattern::Grid(grid) => Pattern::grid(grid.transformed(transform)),
            Pattern::Ring(ring) => Pattern::ring(ring.transformed(transform)),
            Pattern::Solid(_) => self,
            Pattern::Stripe(stripe) => Pattern::stripe(stripe.transformed(transform)),
        }
    }

    fn transform(&mut self, transform: Matrix) -> &mut Pattern {
        *self = match self {
            Pattern::Gradient(gradient) => Pattern::gradient(gradient.transformed(transform)),
            Pattern::Grid(grid) => Pattern::grid(grid.transformed(transform)),
            Pattern::Ring(ring) => Pattern::ring(ring.transformed(transform)),
            Pattern::Solid(_) => *self,
            Pattern::Stripe(stripe) => Pattern::stripe(stripe.transformed(transform)),
        };
        self
    }
}

impl Textured for Pattern {
    fn color_at(&self, object_space_point: Point) -> Color {
        match self {
            Pattern::Gradient(gradient) => gradient.color_at(object_space_point),
            Pattern::Grid(grid) => grid.color_at(object_space_point),
            Pattern::Ring(ring) => ring.color_at(object_space_point),
            Pattern::Solid(solid) => solid.color_at(object_space_point),
            Pattern::Stripe(stripe) => stripe.color_at(object_space_point),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{Form, Geometry, Matrix},
        world::{Material, Texture},
    };

    fn setup() -> (Color, Color) {
        (Color::black(), Color::white())
    }

    #[test]
    fn create_stripes() {
        let (black, white) = setup();
        let pattern = Pattern::stripe(Stripe::new(white, black));
        if let Pattern::Stripe(stripe) = pattern {
            assert_eq!(stripe[0], white);
            assert_eq!(stripe[1], black);
        } else {
            panic!();
        }
    }

    #[test]
    fn pattern_on_transformed_object() {
        let (black, white) = setup();
        let object = Geometry::default()
            .with_form(Form::Sphere)
            .with_material(
                Material::default()
                    .with_texture(Texture::pattern(Pattern::stripe(Stripe::new(white, black)))),
            )
            .transformed(Matrix::scaling(2.0, 2.0, 2.0));
        assert_eq!(object.color_at(Point::new(1.5, 0.0, 0.0)), Color::white(),);
    }

    #[test]
    fn transformed_pattern_on_object() {
        let (black, white) = setup();
        let object = Geometry::default().with_form(Form::Sphere).with_material(
            Material::default().with_texture(
                Texture::pattern(Pattern::stripe(Stripe::new(white, black)))
                    .transformed(Matrix::scaling(2.0, 2.0, 2.0)),
            ),
        );
        assert_eq!(object.color_at(Point::new(1.5, 0.0, 0.0)), Color::white(),);
    }

    #[test]
    fn transformed_pattern_on_transformed_object() {
        let (black, white) = setup();
        let object = Geometry::default()
            .with_form(Form::Sphere)
            .with_material(
                Material::default().with_texture(
                    Texture::pattern(Pattern::stripe(Stripe::new(white, black)))
                        .transformed(Matrix::translation(0.5, 0.0, 0.0)),
                ),
            )
            .transformed(Matrix::scaling(2.0, 2.0, 2.0));
        assert_eq!(object.color_at(Point::new(2.5, 0.0, 0.0)), Color::white(),);
    }
}
