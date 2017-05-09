#![allow(dead_code)]

#![feature(custom_attribute)]

extern crate x11;
extern crate libc;
extern crate nalgebra as na;
extern crate image;
extern crate time;
extern crate specs;

#[macro_use]
extern crate glium;

mod drawable;
mod drawing_util;
mod camera_state;
mod constants;
mod sprite;
mod glium_types;
mod line;
mod rendering;
mod render_steps;

use drawable::{Drawable};
use sprite::{SpriteFactory};

use x11::xlib;
use std::mem;

use line::Line;

use std::sync::Arc;

use glium::texture::{RawImage2d};

use camera_state::CameraState;

use glium_types::{Pixel};

use std::path::Path;

use rendering::RenderProcess;
use render_steps::{RenderSteps, RenderParameters};





pub fn load_texture<'a>(filename: &Path) -> RawImage2d<'a, u8>
{
    let image = image::open(filename).unwrap().to_rgba();
    let dimensions = image.dimensions();
    let raw_pixels = image.into_raw();

    RawImage2d::from_raw_rgba(raw_pixels, dimensions)
}


fn generate_grid(display: &glium::Display) -> Vec<Line>
{
    let mut result = vec!();

    let line_amount = 15;
    let step = 100.;
    
    let other_start = -line_amount as f32 * step;
    let other_end = -other_start as f32;
    for pos in -line_amount..line_amount
    {
        let pos_float = pos as f32 * step;
        let color = match pos
        {
            0 => (1., 1., 1., 1.),
            _ => (0.25, 0.25, 0.25, 1.)
        };

        {
            let start = na::Vector2::new(pos_float, other_start);
            let end = na::Vector2::new(pos_float, other_end);

            let line = line::Line::new(display, start, end)
                .with_color(color);
            result.push(line);
        }
        {
            let start = na::Vector2::new(other_start, pos_float);
            let end = na::Vector2::new(other_end, pos_float);

            let line = line::Line::new(display, start, end)
                .with_color(color);
            result.push(line);
        }
    }

    result
}



pub fn run_selector()
{
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();


    //let screenshot = capture_screenshot();

    //let image = screenshot.get_glium_image();
    //let image = RawImage2d::from_raw_rgba(image::open(&Path::new("media/fir1.png")).unwrap().raw_pixels());
    let texture_path = Path::new("media/lamp1.png");
    let texture = glium::texture::SrgbTexture2d::new(&display, load_texture(texture_path)).unwrap();
    let emissive_path = Path::new("media/lamp_emissive.png");
    let emissive = glium::texture::SrgbTexture2d::new(&display, load_texture(emissive_path)).unwrap();

    //let mut sprite = Sprite::new(&display, Arc::new(texture));
    let sprite_factory = SpriteFactory::new(&display);

    let mut sprite = sprite_factory.create_sprite(Arc::new(texture));
    sprite.set_additional_texture(RenderSteps::Emissive, Arc::new(emissive));

    let mut camera_state = CameraState::new();
    camera_state.set_position(na::Vector2::new(0., 0.));
    //camera_state.set_zoom(0.5);


    sprite.set_position(na::Vector2::new(100., 100.));
    //sprite.set_position(na::Vector2::new(0., 200.));
    sprite.set_origin(na::Vector2::new(0.5, 0.5));
    sprite.set_scale(na::Vector2::new(4., 4.));

    let grid = generate_grid(&display);

    let target_uniforms = RenderParameters::new(&display, display.get_framebuffer_dimensions());
    let render_process = RenderProcess::new(
            &display,
            RenderSteps::get_hash_set(),
            target_uniforms,
            render_steps::DEFAULT_FRAGMENT_SHADER,
            render_steps::default_render_function
        );

    let mut t: f32 = 0.;

    let mut mouse_pos = na::zero();

    let mut render_targets = render_process.get_targets();

    //let mut old_time = time::now();
    loop {
        //XXX Enable for FPS counter
        /*
        let now = time::now();
        let time_since_last = now - old_time;
        old_time = now;

        let frametime_nanos = time_since_last.num_nanoseconds().unwrap();
        let frametime_millis = time_since_last.num_milliseconds();
        let fps = if frametime_nanos != 0 
        {
            1_000_000_000 / frametime_nanos
        } else 
        {
            0
        };
        println!("Elapsed time: {} ms, FPS: {}", frametime_millis, fps);
        */

        for (_, target) in &mut render_targets
        {
            target.clear_color(0., 0., 0., 0.);
        }


        //sprite.set_position(na::Vector2::new((t * 0.01).sin(), 0.));
        sprite.set_angle(t * 0.05);

        t += 0.05;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);

        for (step, target) in &mut render_targets
        {
            for line in &grid
            {
                line.draw(target, step, &camera_state);
            }

            //sprite.draw(&mut target, &camera_state);
            sprite.draw(target, step, &camera_state);
        }

        render_process.draw_to_display(&mut target);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(x, y) => {
                    let new_mouse = na::Vector2::new(x as f32, y as f32);

                    //let new_pos = sprite.get_position() + moved;
                    //let new_pos = camera_state.get_position() + moved;

                    sprite.set_position(new_mouse);
                    //camera_state.set_position(new_mouse);

                    mouse_pos = new_mouse;
                },
                _ => ()
            }
        }
    }
}

fn main() {
    run_selector();
}

