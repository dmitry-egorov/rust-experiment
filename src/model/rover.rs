extern crate vecmath;

use self::vecmath::*;
use model::resource::*;
use std::f64;

pub struct Rover {
    pub position: Vector2<f64>,
    pub direction: Vector2<f64>,
    pub speed: f64
}

pub fn update_rover(rover: Rover, resources: &[Resource], dt: f64) -> Rover {
    let r = find_target(rover, resources);
    apply_velocity(r, dt)
}



fn find_target(rover: Rover, resources: &[Resource]) -> Rover {
    let rover_speed = 2.0;
    let closest_resource = find_closest_resource(&rover, resources);

    match closest_resource {
        Some(resource) =>
            Rover {
              direction: vec2_normalized_sub(resource.position, rover.position),
              speed    : rover_speed,
              ..rover
            },
        _ => rover
    }

}

fn apply_velocity(rover: Rover, dt: f64) -> Rover {
  let delta = vec2_scale(rover.direction, rover.speed * dt);

  Rover {position: vec2_add(rover.position, delta), ..rover }
}

fn find_closest_resource<'a>(rover: &Rover, resources: &'a [Resource]) -> Option<&'a Resource> {
    let (r, _) =
  resources
  .iter()
  .map(|r| (Some(r), vec2_square_len(vec2_sub(r.position, rover.position))))
  .fold((None, f64::INFINITY), |(r1, d1), (r2, d2)| (if d1 < d2 {(r1, d1)} else {(r2, d2)}));

  r
}
