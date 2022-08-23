use crate::math::{matrix::Matrix, point::Point, vector::Vector};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, time: f64) -> Point {
        self.origin + (self.direction * time)
    }

    pub fn transformed(&self, matrix: Matrix) -> Ray {
        Ray::new(matrix * self.origin, matrix * self.direction)
    }

    pub fn transform(mut self, matrix: Matrix) -> Ray {
        self = self.transformed(matrix);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn points_along_ray() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(ray.at(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.at(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.at(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.at(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate_ray() {
        let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let r2 = r1.transformed(m);
        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_ray() {
        let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let r2 = r1.transformed(m);
        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
