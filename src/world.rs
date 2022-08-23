pub mod camera;
pub use camera::{Camera, View};

pub mod canvas;
pub use canvas::Canvas;

pub mod color;
pub use color::Color;

pub mod intersection;
pub use intersection::{Intersection, Intersections};

pub mod light;
pub use light::Light;

pub mod material;
pub use material::Material;

pub mod pattern;
pub use pattern::Pattern;

pub mod ray;
pub use ray::Ray;

pub mod texture;
pub use texture::{Texture, Textured};

use std::{cmp::Reverse, collections::BinaryHeap};

use crate::math::{Form, Geometry, Hittable, Matrix, Point, Transformable};

pub struct World {
    pub objects: Vec<Geometry>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(objects: Vec<Geometry>, lights: Vec<Light>) -> World {
        World { objects, lights }
    }

    pub fn cast_ray(&self, ray: Ray) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);

        if let Some(intersections) = self.hit(ray) {
            if let Some(intersection) = intersections.closest() {
                for light in &self.lights {
                    color += light.illuminate(self, &intersection.compute());
                }
            }
        }

        color
    }

    pub fn hit(&self, ray: Ray) -> Option<Intersections> {
        let mut heap: BinaryHeap<Reverse<Intersection>> = BinaryHeap::new();

        for object in self.objects.iter() {
            if let Some(mut hits) = object.hit(ray) {
                heap.append(&mut hits.heap);
            }
        }

        if !heap.is_empty() {
            Some(Intersections::new(heap))
        } else {
            None
        }
    }
}

impl Default for World {
    fn default() -> World {
        let mut outer = Geometry::default().with_form(Form::Sphere);
        outer.material.texture = Texture::pattern(Pattern::solid(Color::new(0.8, 1.0, 0.6)));
        outer.material.diffuse = 0.7;
        outer.material.specular = 0.2;
        let mut inner = Geometry::default().with_form(Form::Sphere);
        inner.transform(Matrix::scaling(0.5, 0.5, 0.5));
        let sun = Light::point(light::Point::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        World::new(vec![outer, inner], vec![sun])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vector;

    #[test]
    fn empty_world() {
        let w = World::new(vec![], vec![]);
        assert!(w.objects.is_empty());
        assert!(w.lights.is_empty());
    }

    #[test]
    fn default_world() {
        let light = Light::point(light::Point::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let mut s1 = Geometry::default().with_form(Form::Sphere);
        s1.material.texture = Texture::pattern(Pattern::solid(Color::new(0.8, 1.0, 0.6)));
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Geometry::default().with_form(Form::Sphere);
        s2.transform(Matrix::scaling(0.5, 0.5, 0.5));
        let w = World::default();
        assert_eq!(w.lights, vec![light]);
        assert_eq!(w.objects, vec![s1, s2]);
    }

    #[test]
    fn intersect_with_world() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut xs = w.hit(r).unwrap();
        assert_eq!(xs.count(), 4);
        assert_eq!(xs.pop().unwrap().time, 4.0);
        assert_eq!(xs.pop().unwrap().time, 4.5);
        assert_eq!(xs.pop().unwrap().time, 5.5);
        assert_eq!(xs.pop().unwrap().time, 6.0);
    }

    #[test]
    fn shading_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let result = w.cast_ray(r);
        assert_eq!(result, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default();
        w.lights = vec![Light::point(light::Point::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ))];
        let r = Ray::new(Point::zero(), Vector::new(0.0, 0.0, 1.0));
        let result = w.cast_ray(r);
        assert_eq!(result, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.cast_ray(r);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.cast_ray(r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.cast_ray(r);
        assert_eq!(c, w.objects[1].color_at(Point::zero()));
    }

    #[test]
    fn no_shadow_when_nothing_blocks_light() {
        let w = World::default();
        let point = Point::new(0.0, 10.0, 0.0);
        assert_eq!(w.lights[0].casts_shade(&w, point), false);
    }

    #[test]
    fn shadow_when_object_blocks_light() {
        let w = World::default();
        let point = Point::new(10.0, -10.0, 10.0);
        assert_eq!(w.lights[0].casts_shade(&w, point), true);
    }

    #[test]
    fn no_shadow_when_light_blocks_object() {
        let w = World::default();
        let point = Point::new(-20.0, 20.0, -20.0);
        assert_eq!(w.lights[0].casts_shade(&w, point), false);
    }

    #[test]
    fn no_shadow_when_point_blocks_object() {
        let w = World::default();
        let point = Point::new(-2.0, 2.0, -2.0);
        assert_eq!(w.lights[0].casts_shade(&w, point), false);
    }

    #[test]
    fn intersection_in_shadow() {
        let mut w = World::default();
        w.lights = vec![Light::point(light::Point::new(
            Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ))];
        let s1 = Geometry::default().with_form(Form::Sphere);
        w.objects.push(s1);
        let s2 = Geometry::default()
            .with_form(Form::Sphere)
            .transformed(Matrix::translation(0.0, 0.0, 10.0));
        w.objects.push(s2);
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, r, s2);
        let comps = i.compute();
        let c = w.lights[0].illuminate(&w, &comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
