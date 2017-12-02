//#![allow(dead_code)]

#![feature(custom_attribute)]

extern crate nalgebra as na;
extern crate image;
extern crate time;
extern crate specs;
extern crate rand;

#[macro_use]
extern crate glium;

#[macro_use]
extern crate lazy_static;

mod drawable;
mod drawing_util;
mod camera_state;
mod constants;
mod sprite;
mod glium_types;
mod line;
mod rendering;
mod render_steps;
mod components;
mod grid;

use drawable::{Drawable};
use sprite::{SpriteFactory};

use line::Line;

use std::sync::Arc;

use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::Surface;

use camera_state::CameraState;

use glium_types::{Pixel};

use std::path::Path;

use rendering::RenderProcess;
use render_steps::{RenderSteps, RenderParameters};

use std::collections::HashMap;


#[derive(Clone, Debug)]
struct Orientation
{
    position: na::Vector2<f32>,
    angle: f32
}
impl specs::Component for Orientation
{
    type Storage = specs::VecStorage<Orientation>;
}

#[derive(Clone, Debug)]
struct Name
{
    name: String
}
impl specs::Component for Name
{
    type Storage = specs::HashMapStorage<Name>;
}



pub fn load_texture<'a>(filename: &Path) -> RawImage2d<'a, u8>
{
    let image = image::open(filename).unwrap().to_rgba();
    let dimensions = image.dimensions();
    let raw_pixels = image.into_raw();

    RawImage2d::from_raw_rgba(raw_pixels, dimensions)
}



fn load_textures(display: &glium::Display)
    -> HashMap<grid::BlockType, Vec<Arc<SrgbTexture2d>>>
{
    let mut result = HashMap::new();

    fn add_texture(display: &glium::Display, map: &mut HashMap<grid::BlockType, Vec<Arc<SrgbTexture2d>>>, path: &str) {
        map.insert(
            grid::BlockType::Stone,
            vec!(Arc::new(SrgbTexture2d::new(display, load_texture(Path::new(path))).unwrap()))
        );
    };

    add_texture(display, &mut result, "media/stone.png");
    add_texture(display, &mut result, "media/StoneLadder.png");
    result
}



pub fn run_selector()
{
    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glium::glutin::EventsLoop::new();
    // 2. Parameters for building the Window.
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let context = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(window, context, &events_loop).unwrap();


    //let mut sprite = Sprite::new(&display, Arc::new(texture));
    let sprite_factory = SpriteFactory::new(&display);

    let mut camera_state = CameraState::new();
    camera_state.set_position(na::Vector2::new(0., 0.));



    let target_uniforms = RenderParameters::new(&display, display.get_framebuffer_dimensions());
    let render_process = RenderProcess::new(
            &display,
            RenderSteps::get_hash_set(),
            target_uniforms,
            render_steps::DEFAULT_FRAGMENT_SHADER,
            render_steps::default_render_function
        );

    let mut t: f32 = 0.;


    let mut render_targets = render_process.get_targets();



    //let mut old_time = time::now();
    loop {
        for (_, target) in &mut render_targets
        {
            target.clear_color(0., 0., 0., 0.);
        }


        //sprite.set_position(na::Vector2::new((t * 0.01).sin(), 0.));
        t += 0.05;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);

        for (step, target) in &mut render_targets
        {
        }

        render_process.draw_to_display(&mut target);

        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glium::glutin::Event::WindowEvent{window_id, event} => {
                    match event {
                        glium::glutin::WindowEvent::Closed => return,
                        _ => ()
                    }
                }
                _ => {}
            }
        });
    }
}

fn main() {
    run_selector();
}

