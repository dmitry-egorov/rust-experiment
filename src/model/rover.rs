extern crate nalgebra;

use nalgebra::{Vec2, Norm};
use model::resource::Resource;
use std::f64;

pub type RoverId = u32;

pub struct Rover
{
    id: RoverId,

    position: Vec2<f64>,
    direction: Vec2<f64>,
    speed: f64,

    hit_points: f64
}

impl Rover
{
    pub fn new(id: u32, position: Vec2<f64>) -> Self
    {
        Rover
        {
            id: id,

            position: position,
            direction: Vec2::y(),
            speed: 0.0,

            hit_points: 100.0
        }
    }

    pub fn id(&self) -> RoverId
    {
        self.id
    }

    pub fn position(&self) -> Vec2<f64>
    {
        self.position
    }

    pub fn direction(&self) -> Vec2<f64>
    {
        self.direction
    }

    pub fn speed(&self) -> f64
    {
        self.speed
    }

    pub fn hit_points(&self) -> f64
    {
        self.hit_points
    }

    pub fn update(self, resources: &[Resource], dt: f64) -> Self
    {
        self
        .age(dt)
        .find_target(resources)
        .apply_velocity(dt)
    }

    pub fn is_alive(&self) -> bool
    {
        self.hit_points > 0.0
    }

    pub fn sq_distance_from(&self, position: Vec2<f64>) -> f64
    {
        (self.position - position).sqnorm()
    }

    fn age(self, dt: f64) -> Self
    {
        let hp_per_second = 2.0;

        Rover {hit_points: self.hit_points - hp_per_second * dt, ..self}
    }

    fn find_target(self, resources: &[Resource]) -> Self
    {
        let rover_speed = 2.0;

        self
        .find_closest_resource(resources)
        .map(|resource|
        {
            Rover {
                direction: resource.direction_from(self.position),
                speed    : rover_speed,
                ..self
            }
        })
        .unwrap_or_else(|| {self})
    }

    fn apply_velocity(self, dt: f64) -> Self
    {
      let delta = self.direction * (self.speed * dt);

      Rover {position: self.position + delta, ..self }
    }

    fn find_closest_resource<'a>(&self, resources: &'a [Resource]) -> Option<&'a Resource>
    {
        //TODO: remove duplicate in world
        let (r, _) =
            resources
            .iter()
            .map(|resource| (Some(resource), resource.sq_distance_from(self.position)))
            .fold((None, f64::INFINITY), |(r1, d1), (r2, d2)| (if d1 < d2 {(r1, d1)} else {(r2, d2)}));

        r
    }
}
