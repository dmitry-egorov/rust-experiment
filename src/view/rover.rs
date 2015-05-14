use elmesque::form::*;
use elmesque::color::*;
use model::rover::Rover;
use view::extensions::ViewForm;

pub trait RoversSliceExt
{
    fn render(&self) -> Form;
}

impl RoversSliceExt for [Rover]
{
    fn render(&self) -> Form
    {
        group(
            self
            .iter()
            .map(|r| r.render())
            .collect()
        )
    }
}

impl Rover
{
    pub fn render(&self) -> Form
    {
        group(vec![
          rect(0.5, 1.0).filled(black()),
          traced(solid(black()).width(0.05), segment((0.0, 0.0), (0.0, 0.75)))
        ])
        .rotate_to(self.direction())
        .shift_by(self.position())
    }
}
