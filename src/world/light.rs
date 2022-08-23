use crate::{
    math,
    world::{intersection::Computations, Color, Textured, World},
};

pub mod point;
pub use point::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Light {
    Point(Point),
}

impl Light {
    pub fn point(point: Point) -> Light {
        Self::Point(point)
    }

    pub fn illuminate(&self, world: &World, computations: &Computations) -> Color {
        let variant = match self {
            Self::Point(point) => point,
        };

        // combine the surface color with the light's color with respect to its intensity
        let effective_color = computations.material.color_at(computations.point) * variant.color;
        // find the direction to the light source
        let to_light = (variant.position - computations.point).normalized();
        // compute the ambient contribution
        let ambient = effective_color * computations.material.ambient;
        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. a negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = to_light.dot(&computations.surface_normal);

        let (diffuse, specular) = if light_dot_normal >= 0.0 {
            // compute the diffuse contribution
            let diffuse = effective_color * computations.material.diffuse * light_dot_normal;
            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. a negative number means the
            // light reflects away from the eye.
            let reflected_light = (-to_light).reflect_across(computations.surface_normal);
            let reflect_dot_eye = reflected_light.dot(&computations.to_eye);
            if reflect_dot_eye <= 0.0 {
                (diffuse, Color::new(0.0, 0.0, 0.0))
            } else {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(computations.material.shininess);
                (
                    diffuse,
                    variant.color * computations.material.specular * factor,
                )
            }
        } else {
            (Color::new(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0))
        };

        if variant.casts_shade(world, computations.point) {
            // the point is in the shadow cast by this light
            ambient
        } else {
            // add the three contributions together to get the final shading
            ambient + diffuse + specular
        }
    }

    pub fn casts_shade(&self, world: &World, point: math::Point) -> bool {
        match self {
            Self::Point(p) => p.casts_shade(world, point),
        }
    }
}
