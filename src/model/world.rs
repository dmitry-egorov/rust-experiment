extern crate rand;

use model::rover::*;
use model::resource::*;
use rand::Rng;

pub struct World {
    pub rover: Rover,
    pub resources: Vec<Resource>,
}

pub fn default_world<R: Rng>(count: i32, rng: &mut R) -> World
{
    World {
        rover : Rover
        {
            position : [0.0, 0.0],
            direction : [0.0, 1.0],
            speed: 0.0
        },
        resources :
        (0..count)
        .map(move |_| Resource {
            position:
            [
                rng.gen::<f64>() * 40.0 - 20.0,
                rng.gen::<f64>() * 40.0 - 20.0
            ]
        })
        .collect()
    }
}

pub fn update_world(world: World, dt: f64) -> World
{
    World { rover: update_rover(world.rover, &world.resources, dt), .. world }
}
