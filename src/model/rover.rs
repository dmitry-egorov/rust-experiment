extern crate nalgebra;

use nalgebra::{Vec2, Norm};
use model::resource::Resource;
use std::f64;

pub struct Rover
{
    pub position: Vec2<f64>,
    pub direction: Vec2<f64>,
    pub speed: f64
}

pub fn update_rover(rover: Rover, resources: &[Resource], dt: f64) -> Rover
{
    let r = find_target(rover, resources);
    apply_velocity(r, dt)
}



fn find_target(rover: Rover, resources: &[Resource]) -> Rover
{
    let rover_speed = 2.0;
    let closest_resource = find_closest_resource(&rover, resources);

    match closest_resource {
        Some(resource) =>
            Rover {
              direction: (resource.position - rover.position).normalize(),
              speed    : rover_speed,
              ..rover
            },
        _ => rover
    }

}

fn apply_velocity(rover: Rover, dt: f64) -> Rover
{
  let delta = rover.direction * (rover.speed * dt);

  Rover {position: rover.position + delta, ..rover }
}

fn find_closest_resource<'a>(rover: &Rover, resources: &'a [Resource]) -> Option<&'a Resource>
{
    let (r, _) =
        resources
        .iter()
        .map(|resource| (Some(resource), (resource.position - rover.position).sqnorm()))
        .fold((None, f64::INFINITY), |(r1, d1), (r2, d2)| (if d1 < d2 {(r1, d1)} else {(r2, d2)}));

    r
}
