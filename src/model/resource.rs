extern crate nalgebra;

use nalgebra::{Vec2, Norm};

pub struct Resource
{
    position: Vec2<f64>,
}

impl Resource
{
    pub fn new(position: Vec2<f64>) -> Self
    {
        Resource
        {
            position: position
        }
    }

    pub fn position(&self) -> Vec2<f64>
    {
        self.position
    }

    pub fn direction_from(&self, position: Vec2<f64>) -> Vec2<f64>
    {
        (self.position - position).normalize()
    }

    pub fn sq_distance_from(&self, position: Vec2<f64>) -> f64
    {
        (self.position - position).sqnorm()
    }
}
