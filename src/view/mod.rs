extern crate elmesque;

#[macro_use]
mod extensions;
mod rover;
mod resource;
mod world;

use elmesque::form::*;
use model::world::*;

pub fn zoom() -> f64
{
    20.0
}

pub fn render(h: f64, world: &World) -> Form
{
    world.render().scale(h / zoom())
}
