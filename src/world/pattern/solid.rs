use crate::{
    math::Point,
    world::{Color, Textured},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Solid {
    pub color: Color,
}

impl Solid {
    pub fn new(color: Color) -> Solid {
        Solid { color }
    }
}

impl Textured for Solid {
    fn color_at(&self, point: Point) -> Color {
        self.color
    }
}
