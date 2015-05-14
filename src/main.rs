extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate glutin_window;
extern crate graphics;
extern crate shader_version;
extern crate piston;
extern crate rand;
extern crate elmesque;
extern crate num;
extern crate viewport;
extern crate nalgebra;
extern crate drag_controller;

use elmesque::Renderer;
use gfx::traits::*;
use gfx_graphics::{Gfx2d, GlyphCache};
use glutin_window::{GlutinWindow, OpenGL};
use piston::event::*;
use piston::window::{Size, Window, WindowSettings};
use drag_controller::{DragController, Drag};

mod model;
mod view;
mod ui;

use model::world::*;
use ui::ui::Ui;

fn main() {

    let window = GlutinWindow::new(
        OpenGL::_3_2,
        WindowSettings::new(
            "Elmesque".to_string(),
            Size { width: 1180, height: 580 },
        )
        .exit_on_esc(true)
        .samples(4)
    );
    let (mut device, mut factory) = gfx_device_gl::create(|s| window.window.get_proc_address(s));
    let mut g2d = Gfx2d::new(&mut device, &mut factory);
    let mut renderer = factory.create_renderer();
    let Size { width, height } = window.draw_size();
    let output = factory.make_fake_output(width as u16, height as u16);

    let font_path = ::std::path::Path::new("./assets/NotoSans/NotoSans-Regular.ttf");
    let mut glyph_cache = GlyphCache::new(&font_path, &mut factory).unwrap();

    let fps = 60;
    let fixed_dt = 1.0 / (fps as f64);

    let mut drag = DragController::new();

    let events = window.events().ups(fps).max_fps(fps);
    let mut world = World::default(10, 15, &mut rand::thread_rng());
    let mut ui = Ui::new();

    for event in events
    {
        if let Some(args) = event.render_args()
        {
            let (w, h) = (args.width as f64, args.height as f64);

            ui = ui.update_viewport(w, h);

            let world_form = view::render(h, &world);
            let ui_form = ui.render(&world);

            let element = elmesque::form::collage(w as i32, h as i32, vec![world_form, ui_form]);

            g2d.draw(&mut renderer, &output, args.viewport(), |_, graphics|
            {
                let mut renderer = Renderer::new(w, h, graphics).character_cache(&mut glyph_cache);
                element.draw(&mut renderer);
            });

            device.submit(renderer.as_buffer());
            renderer.reset();
            glyph_cache.update(&mut factory);
        }

        if let Some(_) = event.update_args()
        {
            world = world.update(fixed_dt);
        }

        let mut click = None;

        drag.event(&event, |action| {
            match action {
                Drag::End(x, y) => {click = Some((x, y)); false},
                Drag::Interrupt => false,
                _ => true
            }
        });

        if let Some((x, y)) = click
        {
             ui = ui.select_rover(x, y, &world)
        }
    }
}
