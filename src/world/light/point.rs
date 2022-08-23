use crate::{
    math,
    world::{intersection::Computations, Color, Material, Ray, World},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub position: math::Point,
    pub color: Color,
}

impl Point {
    pub fn new(position: math::Point, color: Color) -> Point {
        Point { position, color }
    }

    pub fn casts_shade(&self, world: &World, point: math::Point) -> bool {
        let to_light = self.position - point;
        let distance = to_light.magnitude();
        let direction = to_light.normalized();
        let ray_to_light = Ray::new(point, direction);

        if let Some(intersections) = world.hit(ray_to_light) {
            if let Some(intersection) = intersections.closest() {
                if intersection.time < distance {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{Form, Geometry, Vector},
        world::{
            pattern::{Pattern, Stripe},
            Light, Texture,
        },
    };

    fn setup() -> (Material, math::Point) {
        (Material::default(), math::Point::zero())
    }

    #[test]
    fn point_light_data() {
        let color = Color::new(1.0, 1.0, 1.0);
        let position = math::Point::zero();
        let light = Point::new(position, color);
        assert_eq!(light.position, position);
        assert_eq!(light.color, color);
    }

    #[test]
    fn eye_between_light_and_surface() {
        let (material, point) = setup();
        let to_eye = Vector::new(0.0, 0.0, -1.0);
        let surface_normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::point(Point::new(
            math::Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let world = World::new(vec![], vec![light]);
        let result = light.illuminate(
            &world,
            &Computations {
                point,
                to_eye,
                surface_normal,
                material,
                is_inside: true,
            },
        );
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn eye_between_light_and_surface_with_eye_offset() {
        let (material, point) = setup();
        let to_eye = Vector::new(
            0.0,
            f64::from(2.0).sqrt() / 2.0,
            -f64::from(2.0).sqrt() / 2.0,
        );
        let surface_normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::point(Point::new(
            math::Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let world = World::new(vec![], vec![light]);
        let result = light.illuminate(
            &world,
            &Computations {
                point,
                to_eye,
                surface_normal,
                material,
                is_inside: true,
            },
        );
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn eye_opposite_surface_with_light_offset() {
        let (material, point) = setup();
        let to_eye = Vector::new(0.0, 0.0, -1.0);
        let surface_normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::point(Point::new(
            math::Point::new(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let world = World::new(vec![], vec![light]);
        let result = light.illuminate(
            &world,
            &Computations {
                point,
                to_eye,
                surface_normal,
                material,
                is_inside: true,
            },
        );
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn eye_in_path_of_reflection() {
        let (material, point) = setup();
        let to_eye = Vector::new(
            0.0,
            -f64::from(2.0).sqrt() / 2.0,
            -f64::from(2.0).sqrt() / 2.0,
        );
        let surface_normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::point(Point::new(
            math::Point::new(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let world = World::new(vec![], vec![light]);
        let result = light.illuminate(
            &world,
            &Computations {
                point,
                to_eye,
                surface_normal,
                material,
                is_inside: true,
            },
        );
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn light_behind_surface() {
        let (material, point) = setup();
        let to_eye = Vector::new(0.0, 0.0, -1.0);
        let surface_normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::point(Point::new(
            math::Point::new(0.0, 0.0, 10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let world = World::new(vec![], vec![light]);
        let result = light.illuminate(
            &world,
            &Computations {
                point,
                to_eye,
                surface_normal,
                material,
                is_inside: false,
            },
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_shadowed_surface() {
        let (material, point) = setup();
        let to_eye = Vector::new(0.0, 0.0, -1.0);
        let surface_normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::point(Point::new(
            math::Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let world = World::new(
            vec![Geometry::default().with_form(Form::Sphere)],
            vec![light],
        );
        let result = light.illuminate(
            &world,
            &Computations {
                point,
                to_eye,
                surface_normal,
                material,
                is_inside: false,
            },
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_patterned_surface() {
        let (mut material, _) = setup();
        material.texture =
            Texture::pattern(Pattern::stripe(Stripe::new(Color::white(), Color::black())));
        material.ambient = 1.0;
        material.diffuse = 0.0;
        material.specular = 0.0;
        let to_eye = Vector::new(0.0, 0.0, -1.0);
        let surface_normal = Vector::new(0.0, 0.0, -1.0);
        let light = Light::point(Point::new(
            math::Point::new(0.0, 0.0, -10.0),
            Color::white(),
        ));
        let world = World::new(vec![], vec![light]);
        let c1 = light.illuminate(
            &world,
            &Computations {
                point: math::Point::new(0.9, 0.0, 0.0),
                to_eye,
                surface_normal,
                material,
                is_inside: false,
            },
        );
        let c2 = light.illuminate(
            &world,
            &Computations {
                point: math::Point::new(1.1, 0.0, 0.0),
                to_eye,
                surface_normal,
                material,
                is_inside: false,
            },
        );
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }
}
