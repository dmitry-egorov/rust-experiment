use elmesque::form::*;
use elmesque::color::*;
use model::world::World;
use view::resource::ResourcesSliceExt;
use view::rover::RoversSliceExt;

impl World
{
    pub fn render(&self) -> Form
    {
        fn render_terrain() -> Form
        {
            group(vec![
                rect(50.0, 50.0).filled(orange()),
                circle(15.0).filled(dark_orange()).shift(15.0, -10.0)
            ])
        }

        group(vec![
            render_terrain(),
            self.resources().render(),
            self.rovers().render()
        ])
    }
}
