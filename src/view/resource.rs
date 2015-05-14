extern crate elmesque;

use elmesque::form::*;
use elmesque::color::*;
use model::resource::Resource;
use view::extensions::ViewForm;

pub trait ResourcesSliceExt
{
    fn render(&self) -> Form;
}

impl ResourcesSliceExt for [Resource]
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

impl Resource
{
    pub fn render(&self) -> Form
    {
        square(1.0)
        .filled(yellow())
        .shift_by(self.position())
    }
}
