extern crate nalgebra;
extern crate elmesque;

use nalgebra::Vec2;
use elmesque::form::*;

pub trait ViewForm
{
    fn shift_by(self, position: Vec2<f64>) -> Self;
    fn rotate_to(self, orientation: Vec2<f64>) -> Self;
}

impl ViewForm for Form
{
    fn shift_by(self, position: Vec2<f64>) -> Self
    {
        self.shift(position.x, position.y)
    }

    fn rotate_to(self, orientation: Vec2<f64>) -> Self
    {
        let rotation_angle = angle(Vec2::y(), orientation);
        self.rotate(rotation_angle)
    }
}

fn angle(v1: Vec2<f64>, v2: Vec2<f64>) -> f64
{
    v2.y.atan2(v2.x) - v1.y.atan2(v1.x)
}
