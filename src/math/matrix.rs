use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

use super::{point::Point, vector::Vector, EPSILON};

/// 4-by-4 matrix that represents both a transformation and a translation by using
/// homogeneous coordinates (https://en.wikipedia.org/wiki/Homogeneous_coordinates).
/// the fourth row is always implied to be `{ 0, 0, 0, 1 }`, so therefore the matrix
/// is stored as a sequence of three vectors (the first three columns) followed by
/// a point (the final column). the first three column vectors create a 3-by-3
/// sub-matrix representing the transformation, and the final column represents the
/// translation.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix {
    a: Vector,
    b: Vector,
    c: Vector,
    pub translation: Point,
}

impl Matrix {
    #[rustfmt::skip]
    pub fn new(
        n00: f64, n01: f64, n02: f64, n03: f64,
        n10: f64, n11: f64, n12: f64, n13: f64,
        n20: f64, n21: f64, n22: f64, n23: f64,
    ) -> Matrix {
        Matrix::with_columns(
            Vector::new(n00, n10, n20),
            Vector::new(n01, n11, n21),
            Vector::new(n02, n12, n22),
            Point::new(n03, n13, n23),
        )
    }

    pub fn with_columns(a: Vector, b: Vector, c: Vector, translation: Point) -> Matrix {
        Matrix {
            a,
            b,
            c,
            translation,
        }
    }

    pub fn identity() -> Matrix {
        #[rustfmt::skip]
        Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
        )
    }

    pub fn translation(dx: f64, dy: f64, dz: f64) -> Matrix {
        #[rustfmt::skip]
        Matrix::new(
            1.0, 0.0, 0.0, dx,
            0.0, 1.0, 0.0, dy,
            0.0, 0.0, 1.0, dz,
        )
    }

    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) -> &mut Matrix {
        *self = Matrix::translation(dx, dy, dz) * *self;
        self
    }

    pub fn scaling(dx: f64, dy: f64, dz: f64) -> Matrix {
        #[rustfmt::skip]
        Matrix::new(
            dx,  0.0, 0.0, 0.0,
            0.0, dy,  0.0, 0.0,
            0.0, 0.0, dz,  0.0,
        )
    }

    pub fn scale(&mut self, dx: f64, dy: f64, dz: f64) -> &mut Matrix {
        *self = Matrix::scaling(dx, dy, dz) * *self;
        self
    }

    pub fn rotation_x(radians: f64) -> Matrix {
        let (s, c) = radians.sin_cos();

        #[rustfmt::skip]
        Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, c,   -s,  0.0,
            0.0, s,   c,   0.0,
        )
    }

    pub fn rotate_x(&mut self, radians: f64) -> &mut Matrix {
        *self = Matrix::rotation_x(radians) * *self;
        self
    }

    pub fn rotation_y(radians: f64) -> Matrix {
        let (s, c) = radians.sin_cos();

        #[rustfmt::skip]
        Matrix::new(
            c,   0.0, s,   0.0,
            0.0, 1.0, 0.0, 0.0,
            -s,  0.0, c,   0.0,
        )
    }

    pub fn rotate_y(&mut self, radians: f64) -> &mut Matrix {
        *self = Matrix::rotation_y(radians) * *self;
        self
    }

    pub fn rotation_z(radians: f64) -> Matrix {
        let (s, c) = radians.sin_cos();

        #[rustfmt::skip]
        Matrix::new(
            c,   -s,  0.0, 0.0,
            s,   c,   0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
        )
    }

    pub fn rotate_z(&mut self, radians: f64) -> &mut Matrix {
        *self = Matrix::rotation_z(radians) * *self;
        self
    }

    /// a specialized way to find the inverse of matrices of this specific form.
    /// taken from "foundations of game engine development; volume 1: mathematics"
    /// by eric lengyel.
    pub fn inverse(&self) -> Matrix {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = Vector::new(
            self.translation[0],
            self.translation[1],
            self.translation[2],
        );

        let mut s = a.cross(&b);
        let mut t = c.cross(&d);
        let i = 1.0 / s.dot(&c);

        s *= i;
        t *= i;

        let v = c * i;
        let r0 = b.cross(&v);
        let r1 = v.cross(&a);

        #[rustfmt::skip]
        Matrix::new(
            r0[0], r0[1], r0[2], -b.dot(&t),
            r1[0], r1[1], r1[2], a.dot(&t),
            s[0],  s[1],  s[2],  -d.dot(&s),
        )
    }

    pub fn invert(&mut self) -> &mut Matrix {
        *self = self.inverse();
        self
    }

    /// returns this matrix, but with the 3-by-3 transformation sub-matrix transposed.
    /// the fourth row is still `{ 0, 0, 0, 1 }`, and the translation column is unchanged.
    pub fn transposed(&self) -> Matrix {
        #[rustfmt::skip]
        Matrix::new(
            self[(0, 0)], self[(1, 0)], self[(2, 0)], self.translation[0],
            self[(0, 1)], self[(1, 1)], self[(2, 1)], self.translation[1],
            self[(0, 2)], self[(1, 2)], self[(2, 2)], self.translation[2],
        )
    }

    pub fn transpose(&mut self) -> &mut Matrix {
        *self = self.transposed();
        self
    }

    /// calculates the determinant by finding the equivalent determinant of the 3-by-3
    /// transformation sub-matrix.
    pub fn determinant(&self) -> f64 {
        self[(0, 0)] * (self[(1, 1)] * self[(2, 2)] - self[(1, 2)] * self[(2, 1)])
            + self[(0, 1)] * (self[(1, 2)] * self[(2, 0)] - self[(1, 0)] * self[(2, 2)])
            + self[(0, 2)] * (self[(1, 0)] * self[(2, 1)] - self[(1, 1)] * self[(2, 0)])
    }

    /// uses the determinant to say if an inverse exists.
    pub fn is_invertible(&self) -> bool {
        EPSILON < self.determinant().abs()
    }
}

/* indexing operations */

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    /// access the elements of the 3-by-3 transformation sub-matrix by their `(i, j)` index.
    /// does not allow for accessing of elements in the translation column, which must instead
    /// be accessed via `self.translation`. also does not allow for accessing the fourth row,
    /// which is always implied to be `{ 0, 0, 0, 1 }`.
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        unsafe { &std::mem::transmute::<&Matrix, &[[f64; 3]; 3]>(self)[j][i] }
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    /// access the elements of the 3-by-3 transformation sub-matrix by their `(i, j)` index.
    /// does not allow for accessing of elements in the translation column, which must instead
    /// be accessed via `self.translation`. also does not allow for accessing the fourth row,
    /// which is always implied to be `{ 0, 0, 0, 1 }`.
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f64 {
        unsafe { &mut std::mem::transmute::<&mut Matrix, &mut [[f64; 3]; 3]>(self)[j][i] }
    }
}

impl Index<usize> for Matrix {
    type Output = Vector;

    /// access the column vectors of the 3-by-3 transformation sub-matrix by their column index `j`.
    /// does not allow for accessing the translation column, which is a point instead of a vector and
    /// must be accessed via `self.translation`.
    fn index(&self, j: usize) -> &Self::Output {
        unsafe {
            std::mem::transmute::<&f64, &Vector>(
                &std::mem::transmute::<&Matrix, &[f64; 9]>(self)[j * 3],
            )
        }
    }
}

impl IndexMut<usize> for Matrix {
    /// access the column vectors of the 3-by-3 transformation sub-matrix by their column index `j`.
    /// does not allow for accessing the translation column, which is a point instead of a vector and
    /// must be accessed via `self.translation`.
    fn index_mut(&mut self, j: usize) -> &mut Vector {
        unsafe {
            std::mem::transmute::<&mut f64, &mut Vector>(
                &mut std::mem::transmute::<&mut Matrix, &mut [f64; 9]>(self)[j * 3],
            )
        }
    }
}

/* matrix-matrix operations */

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Matrix::with_columns(
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            Point::new(
                self.translation[0] + other.translation[0],
                self.translation[1] + other.translation[1],
                self.translation[2] + other.translation[2],
            ),
        )
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Matrix::with_columns(
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            Point::new(
                self.translation[0] - other.translation[0],
                self.translation[1] - other.translation[1],
                self.translation[2] - other.translation[2],
            ),
        )
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        #[rustfmt::skip]
        Matrix::new(
            // first row
            self[(0, 0)] * other[(0, 0)] + self[(0, 1)] * other[(1, 0)] + self[(0, 2)] * other[(2, 0)],
            self[(0, 0)] * other[(0, 1)] + self[(0, 1)] * other[(1, 1)] + self[(0, 2)] * other[(2, 1)],
            self[(0, 0)] * other[(0, 2)] + self[(0, 1)] * other[(1, 2)] + self[(0, 2)] * other[(2, 2)],
            self[(0, 0)] * other.translation[0] + self[(0, 1)] * other.translation[1] + self[(0, 2)] * other.translation[2] + self.translation[0],
            // second row
            self[(1, 0)] * other[(0, 0)] + self[(1, 1)] * other[(1, 0)] + self[(1, 2)] * other[(2, 0)],
            self[(1, 0)] * other[(0, 1)] + self[(1, 1)] * other[(1, 1)] + self[(1, 2)] * other[(2, 1)],
            self[(1, 0)] * other[(0, 2)] + self[(1, 1)] * other[(1, 2)] + self[(1, 2)] * other[(2, 2)],
            self[(1, 0)] * other.translation[0] + self[(1, 1)] * other.translation[1] + self[(1, 2)] * other.translation[2] + self.translation[1],
            // third row
            self[(2, 0)] * other[(0, 0)] + self[(2, 1)] * other[(1, 0)] + self[(2, 2)] * other[(2, 0)],
            self[(2, 0)] * other[(0, 1)] + self[(2, 1)] * other[(1, 1)] + self[(2, 2)] * other[(2, 1)],
            self[(2, 0)] * other[(0, 2)] + self[(2, 1)] * other[(1, 2)] + self[(2, 2)] * other[(2, 2)],
            self[(2, 0)] * other.translation[0] + self[(2, 1)] * other.translation[1] + self[(2, 2)] * other.translation[2] + self.translation[2],
        )
    }
}

/* matrix-vector operations */

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Self::Output {
        #[rustfmt::skip]
        Vector::new(
            self[(0, 0)] * vector[0] + self[(0, 1)] * vector[1] + self[(0, 2)] * vector[2],
            self[(1, 0)] * vector[0] + self[(1, 1)] * vector[1] + self[(1, 2)] * vector[2],
            self[(2, 0)] * vector[0] + self[(2, 1)] * vector[1] + self[(2, 2)] * vector[2],
        )
    }
}

/* matrix-point operations */

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, point: Point) -> Self::Output {
        #[rustfmt::skip]
        Point::new(
            self[(0, 0)] * point[0] + self[(0, 1)] * point[1] + self[(0, 2)] * point[2] + self.translation[0],
            self[(1, 0)] * point[0] + self[(1, 1)] * point[1] + self[(1, 2)] * point[2] + self.translation[1],
            self[(2, 0)] * point[0] + self[(2, 1)] * point[1] + self[(2, 2)] * point[2] + self.translation[2],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts;

    #[test]
    fn construct_matrix() {
        #[rustfmt::skip]
        let m = Matrix::new(
            1.0,  2.0,  3.0,  4.0,
            5.5,  6.5,  7.5,  8.5,
            9.0,  10.0, 11.0, 12.0,
        );

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m.translation[0], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
    }

    #[test]
    fn matrix_equality() {
        #[rustfmt::skip]
        let a = Matrix::new(
            1.0, 2.0,  3.0,  4.0,
            5.0, 6.0,  7.0,  8.0,
            9.0, 10.0, 11.0, 12.0,
        );
        #[rustfmt::skip]
        let b = Matrix::new(
            1.0, 2.0,  3.0,  4.0,
            5.0, 6.0,  7.0,  8.0,
            9.0, 10.0, 11.0, 12.0,
        );
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_inequality() {
        #[rustfmt::skip]
        let a = Matrix::new(
            1.0, 2.0,  3.0,  4.0,
            5.0, 6.0,  7.0,  8.0,
            9.0, 10.0, 11.0, 12.0,
        );
        #[rustfmt::skip]
        let b = Matrix::new(
            2.0,  3.0,  4.0,  5.0,
            6.0,  7.0,  8.0,  9.0,
            10.0, 11.0, 12.0, 13.0,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn multiply_two_matrices() {
        #[rustfmt::skip]
        let a = Matrix::new(
            1.0, 2.0,  3.0,  4.0,
            5.0, 6.0,  7.0,  8.0,
            9.0, 10.0, 11.0, 12.0,
        );
        #[rustfmt::skip]
        let b = Matrix::new(
            -2.0, 1.0, 2.0, 3.0,
            3.0,  2.0, 1.0, -1.0,
            4.0,  3.0, 6.0, 5.0,
        );
        assert_eq!(
            a * b,
            #[rustfmt::skip]
            Matrix::new(
                16.0, 14.0, 22.0, 20.0,
                36.0, 38.0, 58.0, 52.0,
                56.0, 62.0, 94.0, 84.0,
            ),
        );
    }

    #[test]
    fn multiply_vector_by_matrix() {
        #[rustfmt::skip]
        let a = Matrix::new(
            1.0, 2.0,  3.0,  4.0,
            5.0, 6.0,  7.0,  8.0,
            9.0, 10.0, 11.0, 12.0,
        );
        let b = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(a * b, Vector::new(14.0, 38.0, 62.0));
    }

    #[test]
    fn multiply_matrix_by_identity() {
        #[rustfmt::skip]
        let a = Matrix::new(
            1.0, 2.0,  3.0,  4.0,
            5.0, 6.0,  7.0,  8.0,
            9.0, 10.0, 11.0, 12.0,
        );
        assert_eq!(a * Matrix::identity(), a);
    }

    #[test]
    fn multiply_vector_by_identity() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(Matrix::identity() * a, a);
    }

    #[test]
    fn transpose_matrix() {
        #[rustfmt::skip]
        let a = Matrix::new(
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
        );
        assert_eq!(
            a.transposed(),
            #[rustfmt::skip]
            Matrix::new(
                0.0, 9.0, 1.0, 0.0,
                9.0, 8.0, 8.0, 8.0,
                3.0, 0.0, 5.0, 3.0,
            ),
        );
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(Matrix::identity().transposed(), Matrix::identity());
    }

    #[test]
    fn invertible_matrix_determinant() {
        #[rustfmt::skip]
        let a = Matrix::new(
            6.0, 4.0,  4.0, 4.0,
            5.0, 5.0,  7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
        );
        assert_eq!(a.determinant(), 260.0);
        assert_eq!(a.is_invertible(), true);
    }

    #[test]
    fn singular_matrix_determinant() {
        #[rustfmt::skip]
        let a = Matrix::new(
            6.0, 4.0,  4.0, 4.0,
            3.0, 2.0,  2.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
        );
        assert_eq!(a.determinant(), 0.0);
        assert_eq!(a.is_invertible(), false);
    }

    #[test]
    fn matrix_inverse() {
        #[rustfmt::skip]
        let a = Matrix::new(
            6.0, 4.0,  4.0, 4.0,
            5.0, 5.0,  7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
        );
        assert_eq!(
            a.inverse(),
            #[rustfmt::skip]
            Matrix::new(
                3.0 / 10.0, -12.0 / 65.0, 2.0 / 65.0,    8.0 / 65.0,
                1.0 / 20.0, 1.0 / 130.0,  -11.0 / 130.0, -109.0 / 130.0,
                -1.0 / 4.0, 7.0 / 26.0,   1.0 / 26.0,    -9.0 / 26.0,
            ),
        );
    }

    #[test]
    fn multiply_matrix_by_inverse() {
        #[rustfmt::skip]
        let a = Matrix::new(
            6.0, 4.0,  4.0, 4.0,
            5.0, 5.0,  7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
        );
        assert_eq!(a * a.inverse(), Matrix::identity());
    }

    #[test]
    fn multiply_matrix_product_by_inverse() {
        #[rustfmt::skip]
        let a = Matrix::new(
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
        );
        #[rustfmt::skip]
        let b = Matrix::new(
            6.0, 4.0,  4.0, 4.0,
            5.0, 5.0,  7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
        );
        assert_eq!(a * b * b.inverse(), a);
    }

    #[test]
    fn multiply_point_by_translation() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_point_by_translation_inverse() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(inverse * p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn vectors_ignore_translation() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn multiply_point_by_scaling() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiply_vector_by_scaling() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiply_vector_by_scaling_inverse() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inverse = transform.inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(inverse * v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn negative_scaling_is_reflection() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotation_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(consts::PI / 4.0);
        let full_quarter = Matrix::rotation_x(consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Point::new(
                0.0,
                f64::from(2.0).sqrt() / 2.0,
                f64::from(2.0).sqrt() / 2.0,
            ),
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotation_x_inverse() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(consts::PI / 4.0);
        let inverse = half_quarter.inverse();
        assert_eq!(
            inverse * p,
            Point::new(
                0.0,
                f64::from(2.0).sqrt() / 2.0,
                -f64::from(2.0).sqrt() / 2.0,
            ),
        );
    }

    #[test]
    fn rotation_y() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(consts::PI / 4.0);
        let full_quarter = Matrix::rotation_y(consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Point::new(
                f64::from(2.0).sqrt() / 2.0,
                0.0,
                f64::from(2.0).sqrt() / 2.0,
            ),
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotation_z() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(consts::PI / 4.0);
        let full_quarter = Matrix::rotation_z(consts::PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Point::new(
                -f64::from(2.0).sqrt() / 2.0,
                f64::from(2.0).sqrt() / 2.0,
                0.0,
            ),
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn transformations_in_sequence() {
        let p1 = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let p2 = a * p1;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));
        let p3 = b * p2;
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));
        let p4 = c * p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn fluent_api() {
        let a = Matrix::rotation_x(consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let mut d = Matrix::identity();
        let d = *d
            .translate(10.0, 5.0, 7.0)
            .scale(5.0, 5.0, 5.0)
            .rotate_x(consts::PI / 2.0);
        assert_eq!(a * b * c, d);
    }
}
