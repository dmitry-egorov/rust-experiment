extern crate elmesque;
extern crate num;
extern crate viewport;
extern crate vecmath;


use model::world::*;
use model::rover::*;
use model::resource::*;
use elmesque::Element;
use elmesque::form::*;
use elmesque::color::*;
use elmesque::text::Text;
use num::Float;
use self::vecmath::*;

pub fn render(w: i32, h: i32, world: &World, fps: usize) -> Element
{
    let form = group(vec![
        render_background(w, h),
        render_resources(&world.resources, h as f64),
        render_rover(&world.rover, h as f64),

        render_fps(w, h, fps)
    ]);

    // Convert the form to an `Element` for rendering.
    elmesque::form::collage(w, h, vec![form])
        .clear(elmesque::color::black())
}

fn render_fps(w: i32, h: i32, fps: usize) -> Form
{
    let t = Text::from_string(format!("fps: {}", fps.to_string())).color(white());

    text(t)
}

/// Demo of grouping multiple forms into a new single form, transformable at any stage.
fn render_background(w: i32, h: i32) -> Form
{
    let (w, h) = (w as f64, h as f64);

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

fn rotation_of(orientation: Vector2<f64>) -> f64
{
    angle([0.0, 1.0], orientation)
}

fn transpose(position: Vector2<f64>, h: f64) -> Vector2<f64>
{
    vec2_scale(position, h / 2.0 / 10.0)
}

fn angle(v1: Vector2<f64>, v2: Vector2<f64>) -> f64
{
    v2[1].atan2(v2[0]) - v1[1].atan2(v1[0])
}
