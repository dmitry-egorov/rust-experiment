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
    group(vec!
    [
      rect(10.0, 20.0).filled(black()),
      traced(solid(black()), segment((0.0, 0.0), (0.0, 15.0)))
    ])
    .rotate_by(rover.direction)
    .shift_by(rover.position, h)
}

fn render_resources(resources: &Vec<Resource>, h: f64) -> Form
{
    group(
        resources
        .iter()
        .map(|r| {
            square(20.0)
            .filled(yellow())
            .shift_by(r.position, h)
        })
        .collect()
    )
}

trait Transformable
{
    fn shift_by(self, position: Vec2<f64>, view_height: f64) -> Self;
    fn rotate_by(self, direction: Vec2<f64>) -> Self;
}

impl Transformable for Form
{
    fn shift_by(self, position: Vec2<f64>, view_height: f64) -> Self
    {
        let scaled = position * (view_height / 2.0 / 10.0);
        self.shift(scaled.x, scaled.y)
    }

    fn rotate_by(self, orientation: Vec2<f64>) -> Self
    {
        let rotation = angle(Vec2::y(), orientation);
        self.rotate(rotation)
    }
}

fn angle(v1: Vec2<f64>, v2: Vec2<f64>) -> f64
{
    v2.y.atan2(v2.x) - v1.y.atan2(v1.x)
}
