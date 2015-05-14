extern crate nalgebra;

use ::nalgebra::Vec2;
use num::traits::Zero;
use model::rover::*;
use model::resource::*;
use rand::Rng;
use std::f64;

pub struct World
{
    rovers: Vec<Rover>,
    resources: Vec<Resource>,
}

impl World
{
    pub fn default<R: Rng>(rovers_count: u32, resources_count: u32, rng: &mut R) -> World
    {
        World {
            rovers :
            (0..rovers_count)
            .map(|i| Rover::new(i, random_vector(rng, 8.0)))
            .collect(),
            resources :
            (0..resources_count)
            .map(|_| Resource::new(random_vector(rng, 10.0)))
            .collect()
        }
    }

    pub fn rovers(&self) -> &[Rover]
    {
        &self.rovers
    }

    pub fn resources(&self) -> &[Resource]
    {
        &self.resources
    }

    pub fn update(self, dt: f64) -> Self
    {
        let new_rovers =
        {
            let resources = &self.resources;

            self.rovers
            .into_iter()
            .map(|r| r.update(resources, dt))
            .filter(|r| r.is_alive())
            .collect()
        };

        World { rovers: new_rovers, .. self }
    }

    pub fn find_rover(&self, position: Vec2<f64>) -> Option<RoverId>
    {
        //TODO: remove duplicate in rover
        let (r, d) =
            self.rovers
            .iter()
            .map(|rover| (Some(rover), rover.sq_distance_from(position)))
            .fold((None, f64::INFINITY), |(r1, d1), (r2, d2)| (if d1 < d2 {(r1, d1)} else {(r2, d2)}));

        if d < 2.0 { r.map(|r| r.id()) } else { None }
    }

    pub fn get_rover(&self, id: RoverId) -> Option<&Rover>
    {
        self.rovers.iter().find(|r| r.id() == id)
    }
}

fn random_vector<R: Rng>(rng: &mut R, square_size: f64) -> Vec2<f64>
{
     Vec2::new(rng.gen::<f64>(), rng.gen::<f64>()) * (2.0 * square_size) - Vec2::new(square_size, square_size)
}
