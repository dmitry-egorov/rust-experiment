extern crate elmesque;
extern crate fps_counter;
extern crate nalgebra;

use ::nalgebra::Vec2;
use elmesque::form::*;
use elmesque::color::*;
use elmesque::text::*;
use self::fps_counter::FPSCounter;

use model::rover::RoverId;
use model::world::World;
use view::zoom;

pub struct Ui
{
    width: f64,
    height: f64,
    selected_rover_id: Option<RoverId>,

    fps_counter: FPSCounter
}

impl Ui
{
    pub fn new() -> Self
    {
        Ui
        {
            width: 0.0,
            height: 0.0,
            selected_rover_id: None,
            fps_counter: FPSCounter::new()
        }
    }

    pub fn render(&mut self, world: &World) -> Form
    {
        let fps = self.fps_counter.tick();

        let rover_info_form = self.render_rover_info(world);
        let mut forms = vec![self.render_fps(fps)];

        if rover_info_form.is_some() {forms.push(rover_info_form.unwrap())};

        group(forms)
        .shift(-self.width / 2.0 + 5.0, self.height / 2.0 -10.0)
    }

    pub fn update_viewport(self, w: f64, h: f64) -> Self
    {
        Ui {width: w, height: h, ..self}
    }

    pub fn select_rover(self, x: f64, y: f64, world: &World) -> Self
    {
        let rover = world.find_rover((Vec2::new(x, -y) + Vec2::new(-self.width / 2.0, self.height / 2.0)) * zoom() / self.height);

        Ui { selected_rover_id : rover, ..self }
    }

    pub fn render_fps(&self, fps: usize) -> Form
    {
        self.text(format!("fps: {}", fps))
    }

    pub fn render_rover_info(&self, world: &World) -> Option<Form>
    {
        self.selected_rover_id
        .and_then(|id| world.get_rover(id).map(|r| (id, r)))
        .map(|(id, rover)|
        {
            let pos = rover.position();
            let dir = rover.direction();
            group(vec![
                self.text(format!("id: {}", id))
                .shift_y(-15.0),
                self.text(format!("pos: {:.2}, {:.2}", pos.x, pos.y))
                .shift_y(-30.0),
                self.text(format!("dir: {:.2} {:.2}", dir.x, dir.y))
                .shift_y(-45.0),
                self.text(format!("speed:{:.2}", rover.speed()))
                .shift_y(-60.0),
                self.text(format!("hp:{:.2}", rover.hit_points()))
                .shift_y(-75.0),
            ])
        })
    }

    fn text(&self, txt: String) -> Form
    {
        text(Text::from_string(txt).color(white()).position(Position::ToRight))
    }
}
