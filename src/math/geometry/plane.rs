use crate::{
    math::{Form, Geometry, Hittable, Point, Vector, EPSILON},
    world::{Intersection, Intersections, Ray},
};

pub struct Plane {}

impl Plane {
    pub fn new() -> Plane {
        Plane {}
    }
}

impl Hittable for Plane {
    fn hit(self, object_space_ray: Ray) -> Option<Intersections> {
        if object_space_ray.direction[1].abs() < EPSILON {
            None
        } else {
            let t = -object_space_ray.origin[1] / object_space_ray.direction[1];
            if t < 0.0 {
                None
            } else {
                Some(Intersections::with(vec![Intersection::new(
                    t,
                    object_space_ray,
                    Geometry::default().with_form(Form::Plane),
                )]))
            }
        }
    }

    fn normal_at(self, _object_space_point: Point) -> Option<Vector> {
        Some(Vector::new(0.0, 1.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_is_constant() {
        let p = Geometry::default().with_form(Form::Plane);
        let n1 = p.normal_at(Point::zero()).unwrap();
        let n2 = p.normal_at(Point::new(10.0, 0.0, -10.0)).unwrap();
        let n3 = p.normal_at(Point::new(-5.0, 0.0, 150.0)).unwrap();
        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_ray_parallel() {
        let p = Geometry::default().with_form(Form::Plane);
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.hit(r);
        assert!(xs.is_none());
    }

    #[test]
    fn intersect_ray_coplanar() {
        let p = Geometry::default().with_form(Form::Plane);
        let r = Ray::new(Point::zero(), Vector::new(0.0, 0.0, 1.0));
        let xs = p.hit(r);
        assert!(xs.is_none());
    }

    #[test]
    fn intersect_ray_above() {
        let p = Geometry::default().with_form(Form::Plane);
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let mut xs = p.hit(r).unwrap();
        assert_eq!(xs.count(), 1);
        assert_eq!(xs.pop().unwrap(), Intersection::new(1.0, r, p));
    }

    #[test]
    fn intersect_ray_below() {
        let p = Geometry::default().with_form(Form::Plane);
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let mut xs = p.hit(r).unwrap();
        assert_eq!(xs.count(), 1);
        assert_eq!(xs.pop().unwrap(), Intersection::new(1.0, r, p));
    }
}
