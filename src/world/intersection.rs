use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use crate::{
    math::{Geometry, Hittable, Point, Vector, EPSILON},
    world::{Material, Ray},
};

#[derive(Copy, Clone, Debug)]
pub struct Computations {
    pub point: Point,
    pub to_eye: Vector,
    pub surface_normal: Vector,
    pub is_inside: bool,
    pub material: Material,
}

impl Computations {
    pub fn new(intersection: &Intersection) -> Computations {
        let point = intersection.ray.at(intersection.time);
        let to_eye = -intersection.ray.direction;

        let mut surface_normal = intersection.object.normal_at(point).unwrap();
        let mut is_inside = false;
        if surface_normal.dot(&to_eye) < 0.0 {
            is_inside = true;
            surface_normal = -surface_normal;
        }

        Computations {
            point: point + (surface_normal * EPSILON),
            to_eye,
            surface_normal,
            is_inside,
            material: intersection.object.material,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub time: f64,
    pub ray: Ray,
    pub object: Geometry,
}

impl Intersection {
    pub fn new(time: f64, ray: Ray, object: Geometry) -> Intersection {
        Intersection { time, ray, object }
    }

    pub fn compute(&self) -> Computations {
        Computations::new(self)
    }
}

/// HACK: this would imply that two different intersections are equal
///       if their respective rays intersect objects at the same time,
///       even if those objects are not the same.
///       this is useful for intersecting with a world that contains
///       multiple objects inside of it.
impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        (self.time - other.time).abs() < EPSILON
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.partial_cmp(&other.time).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Intersections {
    pub heap: BinaryHeap<Reverse<Intersection>>,
}

impl Default for Intersections {
    fn default() -> Intersections {
        Intersections::new(BinaryHeap::new())
    }
}

impl Intersections {
    pub fn new(heap: BinaryHeap<Reverse<Intersection>>) -> Intersections {
        Intersections { heap }
    }

    pub fn with(intersections: Vec<Intersection>) -> Intersections {
        let mut result = Intersections::default();

        for intersection in intersections {
            result.insert(intersection);
        }

        result
    }

    pub fn insert(&mut self, intersection: Intersection) -> &mut Intersections {
        if intersection.time > 0.0 {
            self.heap.push(Reverse(intersection));
        }

        self
    }

    pub fn closest(&self) -> Option<Intersection> {
        if let Some(&Reverse(intersection)) = self.heap.peek() {
            Some(intersection)
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.heap.len()
    }

    pub fn pop(&mut self) -> Option<Intersection> {
        if let Some(Reverse(intersection)) = self.heap.pop() {
            Some(intersection)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Form, Geometry, Matrix, Point, Transformable, Vector};

    #[test]
    fn intersection_encapsulates_object() {
        let s = Geometry::default().with_form(Form::Sphere);
        let r = Ray::new(Point::zero(), Vector::zero());
        let i = Intersection::new(3.5, r, s);
        assert_eq!(i.time, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Geometry::default().with_form(Form::Sphere);
        let r = Ray::new(Point::zero(), Vector::zero());
        let i1 = Intersection::new(1.0, r, s);
        let i2 = Intersection::new(2.0, r, s);
        let mut xs = Intersections::with(vec![i2, i1]);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.pop().unwrap().time, i1.time);
        assert_eq!(xs.pop().unwrap().time, i2.time);
    }

    #[test]
    fn intersections_encapsulates_objects() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Geometry::default().with_form(Form::Sphere);
        let mut xs = s.hit(r).unwrap();
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.pop().unwrap().object, s);
        assert_eq!(xs.pop().unwrap().object, s);
    }

    #[test]
    fn closest_hit_multiple_options() {
        let s = Geometry::default().with_form(Form::Sphere);
        let r = Ray::new(Point::zero(), Vector::zero());
        let i1 = Intersection::new(1.0, r, s);
        let i2 = Intersection::new(2.0, r, s);
        let xs = Intersections::with(vec![i1, i2]);
        assert_eq!(xs.closest().unwrap(), i1);
    }

    #[test]
    fn closest_hit_one_option() {
        let s = Geometry::default().with_form(Form::Sphere);
        let r = Ray::new(Point::zero(), Vector::zero());
        let i1 = Intersection::new(-1.0, r, s);
        let i2 = Intersection::new(1.0, r, s);
        let xs = Intersections::with(vec![i1, i2]);
        assert_eq!(xs.closest().unwrap(), i2);
    }

    #[test]
    fn closest_hit_no_options() {
        let s = Geometry::default().with_form(Form::Sphere);
        let r = Ray::new(Point::zero(), Vector::zero());
        let i1 = Intersection::new(-1.0, r, s);
        let i2 = Intersection::new(-2.0, r, s);
        let xs = Intersections::with(vec![i1, i2]);
        assert!(xs.closest().is_none());
    }

    #[test]
    fn closest_hit_has_lowest_nonnegative_time() {
        let s = Geometry::default().with_form(Form::Sphere);
        let r = Ray::new(Point::zero(), Vector::zero());
        let i1 = Intersection::new(5.0, r, s);
        let i2 = Intersection::new(7.0, r, s);
        let i3 = Intersection::new(-3.0, r, s);
        let i4 = Intersection::new(2.0, r, s);
        let xs = Intersections::with(vec![i1, i2, i3, i4]);
        assert_eq!(xs.closest().unwrap(), i4);
    }

    #[test]
    fn compute_intersection_data() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Geometry::default().with_form(Form::Sphere);
        let i = Intersection::new(4.0, r, shape);
        let comps = i.compute();
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.to_eye, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.surface_normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn intersection_on_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Geometry::default().with_form(Form::Sphere);
        let i = Intersection::new(4.0, r, shape);
        let comps = i.compute();
        assert_eq!(comps.is_inside, false);
    }

    #[test]
    fn intersection_on_inside() {
        let r = Ray::new(Point::zero(), Vector::new(0.0, 0.0, 1.0));
        let shape = Geometry::default().with_form(Form::Sphere);
        let i = Intersection::new(1.0, r, shape);
        let comps = i.compute();
        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.to_eye, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.is_inside, true);
        assert_eq!(comps.surface_normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn intersection_offsets_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Geometry::default()
            .with_form(Form::Sphere)
            .transformed(Matrix::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, r, shape);
        let comps = i.compute();
        assert!(comps.point[2] < (-EPSILON / 2.0));
    }
}
