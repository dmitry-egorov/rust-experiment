extern crate elmesque;
extern crate num;

use nalgebra::Vec2;
use elmesque::form::*;
use elmesque::color::*;
use num::Float;

use model::world::*;
use model::rover::*;
use model::resource::*;

pub fn render(w: f64, h: f64, world: &World) -> Form
{
    group(vec![
        render_background(w, h),
        render_resources(&world.resources, h),
        render_rover(&world.rover, h),
    ])
}

/// Demo of grouping multiple forms into a new single form, transformable at any stage.
fn render_background(w: f64, h: f64) -> Form
{
    group(vec![
        rect(w, h).filled(orange()),
        circle(h).filled(dark_orange())
            .shift(w / 2.0, -h)
        ])
}

fn render_rover(rover: &Rover, h: f64) -> Form
{
    let transposed = transpose(rover.position, h);

    group(vec!
    [
      rect(10.0, 20.0).filled(black()),
      traced(solid(black()), segment((0.0, 0.0), (0.0, 15.0)))
    ])
    .rotate(rotation_of(rover.direction))
    .shift(transposed[0], transposed[1])
}

fn render_resources(resources: &Vec<Resource>, h: f64) -> Form
{
    group(
        resources
        .iter()
        .map(|r| {
            let transposed = transpose(r.position, h);

            square(20.0)
            .filled(yellow())
            .shift(transposed[0], transposed[1])
        })
        .collect()
    )
}

fn rotation_of(orientation: Vec2<f64>) -> f64
{
    angle(Vec2::y(), orientation)
}

fn transpose(position: Vec2<f64>, h: f64) -> Vec2<f64>
{
    position * (h / 2.0 / 10.0)
}

fn angle(v1: Vec2<f64>, v2: Vec2<f64>) -> f64
{
    v2.y.atan2(v2.x) - v1.y.atan2(v1.x)
}
