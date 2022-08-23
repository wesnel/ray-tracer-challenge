use crate::{
    math::{matrix::Matrix, point::Point, vector::Vector},
    world::{canvas::Canvas, ray::Ray, World},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct View {
    pub transform: Matrix,
    pub inverse: Matrix,
}

impl View {
    pub fn transformed(from: Point, to: Point, up: Vector) -> View {
        let mut view = View::default();

        let forward = (to - from).normalized();
        let up = up.normalized();
        let left = forward.cross(&up);
        let up = left.cross(&forward);

        #[rustfmt::skip]
        let orientation = Matrix::new(
            left[0],     left[1],     left[2],     0.0,
            up[0],       up[1],       up[2],       0.0,
            -forward[0], -forward[1], -forward[2], 0.0,
        );

        view.transform = orientation * Matrix::translation(-from[0], -from[1], -from[2]);
        view.inverse = view.transform.inverse();

        view
    }

    pub fn transform(&mut self, from: Point, to: Point, up: Vector) -> &mut View {
        *self = View::transformed(from, to, up);
        self
    }
}

impl Default for View {
    fn default() -> View {
        View {
            transform: Matrix::identity(),
            inverse: Matrix::identity(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    pub field_of_view: f64,
    pub view: View,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(image_width: usize, image_height: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect_ratio = (image_width as f64) / (image_height as f64);

        let (half_width, half_height) = if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };

        Camera {
            image_width,
            image_height,
            field_of_view,
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / (image_width as f64),
            view: View::default(),
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let x_offset = ((x as f64) + 0.5) * self.pixel_size;
        let y_offset = ((y as f64) + 0.5) * self.pixel_size;

        // the un-transformed coordinates of the pixel in world space.
        // (the camera looks towards -z, so +x is to the left)
        let world_space_x = self.half_width - x_offset;
        let world_space_y = self.half_height - y_offset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (the canvas is at z = -1)
        let pixel = self.view.inverse * Point::new(world_space_x, world_space_y, -1.0);
        let origin = self.view.inverse * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalized();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let ray = self.ray_for_pixel(x, y);
                image[(x, y)] = world.cast_ray(ray);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::EPSILON, world::color::Color};
    use std::f64::consts;

    #[test]
    fn default_transformation() {
        let from = Point::zero();
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let view = View::transformed(from, to, up);
        assert_eq!(view.transform, Matrix::identity());
        assert_eq!(view.inverse, Matrix::identity());
    }

    #[test]
    fn looking_in_positive_z_direction() {
        let from = Point::zero();
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let view = View::transformed(from, to, up);
        let transform = Matrix::scaling(-1.0, 1.0, -1.0);
        assert_eq!(view.transform, transform);
        assert_eq!(view.inverse, transform.inverse());
    }

    #[test]
    fn transform_moves_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::zero();
        let up = Vector::new(0.0, 1.0, 0.0);
        let view = View::transformed(from, to, up);
        let transform = Matrix::translation(0.0, 0.0, -8.0);
        assert_eq!(view.transform, transform);
        assert_eq!(view.inverse, transform.inverse());
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let view = View::transformed(from, to, up);
        #[rustfmt::skip]
        let transform = Matrix::new(
            -0.50709, 0.50709, 0.67612, -2.36643,
            0.76772,  0.60609, 0.12122, -2.82843,
            -0.35857, 0.59761, -0.71714, 0.0,
        );
        assert_eq!(view.transform, transform);
        assert_eq!(view.inverse, transform.inverse());
    }

    #[test]
    fn construct_camera() {
        let width = 160;
        let height = 120;
        let field_of_view = consts::PI / 2.0;
        let c = Camera::new(width, height, field_of_view);
        assert_eq!(c.image_width, width);
        assert_eq!(c.image_height, height);
        assert_eq!(c.field_of_view, consts::PI / 2.0);
        assert_eq!(c.view.transform, Matrix::identity());
        assert_eq!(c.view.inverse, Matrix::identity());
    }

    #[test]
    fn pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, consts::PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, consts::PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn ray_through_canvas_center() {
        let c = Camera::new(201, 101, consts::PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::zero());
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_through_canvas_corner() {
        let c = Camera::new(201, 101, consts::PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Point::zero());
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_through_canvas_center_transformed() {
        let mut c = Camera::new(201, 101, consts::PI / 2.0);
        c.view.transform = *Matrix::identity()
            .translate(0.0, -2.0, 5.0)
            .rotate_y(consts::PI / 4.0);
        c.view.inverse = c.view.transform.inverse();
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(
                f64::from(2.0).sqrt() / 2.0,
                0.0,
                -f64::from(2.0).sqrt() / 2.0
            )
        );
    }

    #[test]
    fn render_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, consts::PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::zero();
        let up = Vector::new(0.0, 1.0, 0.0);
        c.view = View::transformed(from, to, up);
        let image = c.render(&w);
        assert_eq!(image[(5, 5)], Color::new(0.38066, 0.47583, 0.2855));
    }
}
