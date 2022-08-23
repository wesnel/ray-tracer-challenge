use crate::{
    math::{Point, EPSILON},
    world::{Color, Pattern, Texture, Textured},
};

/// contains required data for the phong reflection model.
/// (https://en.wikipedia.org/wiki/Phong_reflection_model)
#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub texture: Texture,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(
        texture: Texture,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Material {
        Material {
            texture,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn with_texture(&self, texture: Texture) -> Material {
        Material::new(
            texture,
            self.ambient,
            self.diffuse,
            self.specular,
            self.shininess,
        )
    }
}

impl Default for Material {
    fn default() -> Material {
        Material::new(
            Texture::pattern(Pattern::solid(Color::new(1.0, 1.0, 1.0))),
            0.1,
            0.9,
            0.9,
            200.0,
        )
    }
}

impl Textured for Material {
    fn color_at(&self, object_space_point: Point) -> Color {
        self.texture.color_at(object_space_point)
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.texture == other.texture
            && (self.ambient - other.ambient).abs() < EPSILON
            && (self.diffuse - other.diffuse).abs() < EPSILON
            && (self.specular - other.specular).abs() < EPSILON
            && (self.shininess - other.shininess).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(
            m.texture,
            Texture::pattern(Pattern::solid(Color::new(1.0, 1.0, 1.0))),
        );
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
