use crate::{
    math::{Form, Geometry, Hittable, Matrix, Point, Vector},
    world::{Intersection, Intersections, Material, Ray},
};

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }
}

impl Hittable for Sphere {
    fn hit(self, object_space_ray: Ray) -> Option<Intersections> {
        let origin = Point::zero();
        let displacement = object_space_ray.origin - origin;
        let a = object_space_ray.direction.dot(&object_space_ray.direction);
        let b = 2.0 * object_space_ray.direction.dot(&displacement);
        let c = displacement.dot(&displacement) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        if 0.0 <= discriminant {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let hits = Intersections::with(
                vec![t1, t2]
                    .iter()
                    .filter(|t| t.is_sign_positive())
                    .map(|&t| {
                        Intersection::new(
                            t,
                            object_space_ray,
                            Geometry::default().with_form(Form::Sphere),
                        )
                    })
                    .collect(),
            );
            if hits.count() == 0 {
                None
            } else {
                Some(hits)
            }
        } else {
            None
        }
    }

    fn normal_at(self, object_space_point: Point) -> Option<Vector> {
        Some(object_space_point - Point::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Transformable;
    use std::f64::consts;

    #[test]
    fn ray_intersects_sphere_twice() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Geometry::default().with_form(Form::Sphere);
        let mut xs = sphere.hit(ray).unwrap();
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.pop().unwrap().time, 4.0);
        assert_eq!(xs.pop().unwrap().time, 6.0);
    }

    #[test]
    fn tangent_ray_intersection() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Geometry::default().with_form(Form::Sphere);
        let mut xs = sphere.hit(ray).unwrap();
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.pop().unwrap().time, 5.0);
        assert_eq!(xs.pop().unwrap().time, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Geometry::default().with_form(Form::Sphere);
        assert!(sphere.hit(ray).is_none());
    }

    #[test]
    fn ray_hit_from_inside() {
        let ray = Ray::new(Point::zero(), Vector::new(0.0, 0.0, 1.0));
        let sphere = Geometry::default().with_form(Form::Sphere);
        let mut xs = sphere.hit(ray).unwrap();
        assert_eq!(xs.count(), 1);
        assert_eq!(xs.pop().unwrap().time, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Geometry::default().with_form(Form::Sphere);
        assert!(sphere.hit(ray).is_none());
    }

    #[test]
    fn default_transformation() {
        let s = Geometry::default().with_form(Form::Sphere);
        assert_eq!(s.transform, Matrix::identity());
        assert_eq!(s.inverse, Matrix::identity());
    }

    #[test]
    fn transform_sphere() {
        let mut s = Geometry::default().with_form(Form::Sphere);
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.transform(t);
        assert_eq!(s.transform, t);
        assert_eq!(s.inverse, t.inverse());
    }

    #[test]
    fn intersect_scaled_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Geometry::default().with_form(Form::Sphere);
        s.transform(Matrix::scaling(2.0, 2.0, 2.0));
        let mut xs = s.hit(r).unwrap();
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.pop().unwrap().time, 3.0);
        assert_eq!(xs.pop().unwrap().time, 7.0);
    }

    #[test]
    fn intersect_translated_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Geometry::default().with_form(Form::Sphere);
        s.transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.hit(r);
        assert!(xs.is_none());
    }

    #[test]
    fn normal_on_x_axis() {
        let s = Geometry::default().with_form(Form::Sphere);
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0)).unwrap();
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_y_axis() {
        let s = Geometry::default().with_form(Form::Sphere);
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0)).unwrap();
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_z_axis() {
        let s = Geometry::default().with_form(Form::Sphere);
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0)).unwrap();
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_nonaxial() {
        let s = Geometry::default().with_form(Form::Sphere);
        let n = s
            .normal_at(Point::new(
                f64::from(3.0).sqrt() / 3.0,
                f64::from(3.0).sqrt() / 3.0,
                f64::from(3.0).sqrt() / 3.0,
            ))
            .unwrap();
        assert_eq!(
            n,
            Vector::new(
                f64::from(3.0).sqrt() / 3.0,
                f64::from(3.0).sqrt() / 3.0,
                f64::from(3.0).sqrt() / 3.0,
            )
        );
    }

    #[test]
    fn normal_is_normalized() {
        let s = Geometry::default().with_form(Form::Sphere);
        let n = s
            .normal_at(Point::new(
                f64::from(3.0).sqrt() / 3.0,
                f64::from(3.0).sqrt() / 3.0,
                f64::from(3.0).sqrt() / 3.0,
            ))
            .unwrap();
        assert_eq!(n, n.normalized());
    }

    #[test]
    fn translated_normal() {
        let mut s = Geometry::default().with_form(Form::Sphere);
        s.transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711)).unwrap();
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn transformed_normal() {
        let mut s = Geometry::default().with_form(Form::Sphere);
        s.transform(
            *Matrix::identity()
                .rotate_z(consts::PI / 5.0)
                .scale(1.0, 0.5, 1.0),
        );
        let n = s
            .normal_at(Point::new(
                0.0,
                f64::from(2.0).sqrt() / 2.0,
                -f64::from(2.0).sqrt() / 2.0,
            ))
            .unwrap();
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn default_material() {
        let s = Geometry::default().with_form(Form::Sphere);
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn assign_material() {
        let mut s = Geometry::default().with_form(Form::Sphere);
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
}
