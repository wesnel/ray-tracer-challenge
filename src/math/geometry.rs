pub mod plane;
pub use plane::Plane;

pub mod sphere;
pub use sphere::Sphere;

use crate::{
    math::{Matrix, Point, Vector},
    world::{Color, Intersection, Intersections, Material, Ray, Textured},
};

use std::cmp::Reverse;

pub trait Transformable {
    fn transformed(self, transform: Matrix) -> Self;
    fn transform(&mut self, transform: Matrix) -> &mut Self;
}

/// enum representing the possible geometry objects.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Form {
    Plane,
    Sphere,
    None,
}

/// trait outlining the functionality of a geometry object.
pub trait Hittable {
    fn hit(self, object_space_ray: Ray) -> Option<Intersections>;
    fn normal_at(self, object_space_point: Point) -> Option<Vector>;
}

/// encapsulates the geometry variant along with associated data.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Geometry {
    pub form: Form,
    pub transform: Matrix,
    pub inverse: Matrix,
    pub material: Material,
}

impl Geometry {
    pub fn new(form: Form, transform: Matrix, inverse: Matrix, material: Material) -> Geometry {
        Geometry {
            form,
            transform,
            inverse,
            material,
        }
    }

    pub fn with_form(self, form: Form) -> Geometry {
        Geometry {
            form,
            transform: self.transform,
            inverse: self.inverse,
            material: self.material,
        }
    }

    pub fn change_form(&mut self, form: Form) -> &mut Geometry {
        *self = self.with_form(form);
        self
    }

    pub fn with_material(self, material: Material) -> Geometry {
        Geometry {
            material,
            transform: self.transform,
            inverse: self.inverse,
            form: self.form,
        }
    }

    pub fn change_material(&mut self, material: Material) -> &mut Geometry {
        *self = self.with_material(material);
        self
    }
}

impl Transformable for Geometry {
    fn transformed(self, transform: Matrix) -> Geometry {
        Geometry {
            transform,
            form: self.form,
            inverse: transform.inverse(),
            material: self.material,
        }
    }

    fn transform(&mut self, transform: Matrix) -> &mut Geometry {
        *self = self.transformed(transform);
        self
    }
}

impl Textured for Geometry {
    fn color_at(&self, world_space_point: Point) -> Color {
        let object_space_point = self.inverse * world_space_point;
        self.material.color_at(object_space_point)
    }
}

impl Hittable for Geometry {
    fn hit(self, world_space_ray: Ray) -> Option<Intersections> {
        let object_space_ray = world_space_ray.transformed(self.inverse);

        if let Some(intersections) = match self.form {
            Form::Sphere => Sphere::new().hit(object_space_ray),
            Form::Plane => Plane::new().hit(object_space_ray),
            Form::None => None,
        } {
            Some(Intersections::with(
                intersections
                    .heap
                    .iter()
                    .map(|&Reverse(intersection)| {
                        Intersection::new(intersection.time, world_space_ray, self)
                    })
                    .collect(),
            ))
        } else {
            None
        }
    }

    fn normal_at(self, world_space_point: Point) -> Option<Vector> {
        let object_space_point = self.inverse * world_space_point;

        if let Some(normal) = match self.form {
            Form::Sphere => Sphere::new().normal_at(object_space_point),
            Form::Plane => Plane::new().normal_at(object_space_point),
            Form::None => None,
        } {
            Some((self.inverse.transposed() * normal).normalized())
        } else {
            None
        }
    }
}

impl Default for Geometry {
    fn default() -> Self {
        Geometry {
            form: Form::None,
            transform: Matrix::identity(),
            inverse: Matrix::identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_transformation() {
        let s = Geometry::default();
        assert_eq!(s.transform, Matrix::identity());
        assert_eq!(s.inverse, Matrix::identity());
    }

    #[test]
    fn assigning_transformation() {
        let m = Matrix::translation(2.0, 3.0, 4.0);
        let s = Geometry::default().transformed(m);
        assert_eq!(s.transform, m);
        assert_eq!(s.inverse, m.inverse());
    }

    #[test]
    fn default_material() {
        let s = Geometry::default();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn assigning_material() {
        let mut m = Material::default();
        m.ambient = 1.0;
        let s = Geometry::default().with_material(m);
        assert_eq!(s.material, m);
    }
}
