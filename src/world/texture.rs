use crate::{
    math::{Matrix, Point, Transformable},
    world::{Color, Pattern},
};

pub trait Textured {
    fn color_at(&self, point: Point) -> Color;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Texture {
    Pattern(Pattern),
}

impl Texture {
    pub fn pattern(pattern: Pattern) -> Texture {
        Texture::Pattern(pattern)
    }
}

impl Transformable for Texture {
    fn transformed(self, transform: Matrix) -> Texture {
        match self {
            Texture::Pattern(pattern) => Texture::pattern(pattern.transformed(transform)),
        }
    }

    fn transform(&mut self, transform: Matrix) -> &mut Texture {
        *self = match self {
            Texture::Pattern(pattern) => Texture::pattern(pattern.transformed(transform)),
        };
        self
    }
}

impl Textured for Texture {
    fn color_at(&self, object_space_point: Point) -> Color {
        match self {
            Texture::Pattern(pattern) => pattern.color_at(object_space_point),
        }
    }
}
