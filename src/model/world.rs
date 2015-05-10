extern crate nalgebra;

use nalgebra::Vec2;
use num::traits::Zero;
use model::rover::*;
use model::resource::*;
use rand::Rng;

pub struct World
{
    pub rover: Rover,
    pub resources: Vec<Resource>,
}

pub fn default_world<R: Rng>(count: i32, rng: &mut R) -> World
{
    World {
        rover : Rover
        {
            position : Vec2::zero(),
            direction : Vec2::y(),
            speed: 0.0
        },
        resources :
        (0..count)
        .map(move |_| Resource {
            position:
            Vec2::new(
                rng.gen::<f64>() * 20.0 - 10.0,
                rng.gen::<f64>() * 20.0 - 10.0
            )
        })
        .collect()
    }
}

pub fn update_world(world: World, dt: f64) -> World
{
    World { rover: update_rover(world.rover, &world.resources, dt), .. world }
}
