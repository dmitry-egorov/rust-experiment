extern crate elmesque;

use elmesque::form::*;
use elmesque::color::*;
use elmesque::text::*;

pub fn render(w: f64, h: f64, fps: usize) -> Form
{
    text(Text::from_string(format!("FPS: {}", fps)).color(white()))
    .shift(-w / 2.0 + 30.0, h / 2.0 - 10.0)
}
