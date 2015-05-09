extern crate elmesque;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate glutin_window;
extern crate graphics;
extern crate num;
extern crate shader_version;
extern crate piston;
extern crate rand;
extern crate fps_counter;

use elmesque::Renderer;
use gfx::traits::*;
use gfx_graphics::{Gfx2d, GlyphCache};
use glutin_window::{GlutinWindow, OpenGL};
use piston::event::{Event, Events};
use piston::window::{Size, Window, WindowSettings};

mod renderer;
mod model { pub mod world; pub mod rover; pub mod resource;}

use model::world::*;

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

    let fps = 50;
    let fixed_dt = 1.0 / (fps as f64);

    let event_iter = window.events().ups(fps).max_fps(fps);
    let mut world = default_world(1000, &mut rand::thread_rng());
    let mut counter = fps_counter::FPSCounter::new();

    for event in event_iter
    {
        match event
        {
            Event::Render(args) =>
            {
                let (w, h) = (args.width as f64, args.height as f64);
                let element = renderer::render(w as i32, h as i32, &world, counter.tick());

                g2d.draw(&mut renderer, &output, args.viewport(), |_, graphics|
                {
                    let mut renderer = Renderer::new(w, h, graphics).character_cache(&mut glyph_cache);
                    element.draw(&mut renderer);
                });

                device.submit(renderer.as_buffer());
                renderer.reset();
                glyph_cache.update(&mut factory);
            },
            Event::Update(_) => world = update_world(world, fixed_dt),
            _ => (),
        }
    }
}
