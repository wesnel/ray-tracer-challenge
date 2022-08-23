#![feature(stmt_expr_attributes)]

use std::f64::consts;

mod math;
mod world;

use crate::{
    math::{Form, Geometry, Matrix, Point, Transformable, Vector},
    world::{
        light::{self, Light},
        pattern::{Gradient, Grid, Stripe},
        Camera, Color, Pattern, Texture, View, World,
    },
};

fn main() {
    let mut floor = Geometry::default().with_form(Form::Plane);
    floor.material.texture = Texture::pattern(Pattern::grid(Grid::new(
        Color::new(0.5, 0.1, 0.5),
        Color::new(0.1, 0.1, 0.1),
    )));

    let mut middle = Geometry::default()
        .with_form(Form::Sphere)
        .transformed(Matrix::translation(-0.5, 1.0, 0.5));
    middle.material.texture = Texture::pattern(
        Pattern::stripe(Stripe::new(
            Color::new(0.1, 1.0, 0.5),
            Color::new(0.5, 1.0, 1.0),
        ))
        .transformed(
            Matrix::scaling(0.20, 0.20, 0.20)
                * Matrix::rotation_z(consts::PI / 4.0)
                * Matrix::rotation_y(consts::PI / 4.0),
        ),
    );
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Geometry::default().with_form(Form::Sphere).transformed(
        *Matrix::identity()
            .scale(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5),
    );
    right.material.texture = Texture::pattern(Pattern::gradient(Gradient::new(
        Color::new(1.0, 0.0, 0.0),
        Color::new(0.0, 0.0, 1.0),
    )));
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Geometry::default().with_form(Form::Sphere).transformed(
        *Matrix::identity()
            .scale(0.33, 0.33, 0.33)
            .translate(-1.5, 0.33, -0.75),
    );
    left.material.texture = Texture::pattern(Pattern::solid(Color::new(1.0, 0.8, 0.1)));
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let sun = Light::point(light::Point::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let world = World::new(vec![floor, middle, right, left], vec![sun]);

    let mut camera = Camera::new(1000, 500, consts::PI / 3.0);
    camera.view = View::transformed(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    println!("{}", canvas.to_ppm());
}
