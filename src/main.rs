#![allow(dead_code)]

#![feature(custom_attribute)]

extern crate x11;
extern crate libc;
extern crate nalgebra as na;
extern crate image;
extern crate time;

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


pub struct Screenshot
{
    width: u32,
    height: u32,
    pixels: Vec<Pixel>
}

impl Screenshot
{
    fn new(width: u32, height: u32) -> Screenshot
    {
        let mut pixels = vec!();

        pixels.reserve_exact((width * height) as usize);

        Screenshot
        {
            width: width,
            height: height,
            pixels: pixels
        }
    }

    fn push_pixel(&mut self, pixel: Pixel)
    {
        self.pixels.push(pixel)
    }

    fn get_glium_image<'a>(self) -> RawImage2d<'a, Pixel>
    {
        let dimensions = (self.width, self.height);
        RawImage2d::from_raw_rgb(self.pixels, dimensions)
    }

    fn get_aspect_ratio(&self) -> f32
    {
        self.height as f32 / self.width as f32
    }
}

pub fn capture_screenshot() -> Screenshot
{
    //http://stackoverflow.com/questions/24988164/c-fast-screenshots-in-linux-for-use-with-opencv
    unsafe
    {
        let display = xlib::XOpenDisplay(0 as *const i8);
        let root_window = xlib::XDefaultRootWindow(display);

        let attributes = libc::malloc(mem::size_of::<xlib::XWindowAttributes>())
                     as *mut xlib::XWindowAttributes;

        xlib::XGetWindowAttributes(display
                                   , root_window
                                   , attributes);

        let width = (*attributes).width as u32;
        let height = (*attributes).height as u32;

        let img = xlib::XGetImage(display
                                  , root_window
                                  , 0
                                  , 0
                                  , width
                                  , height
                                  , xlib::XAllPlanes()
                                  , xlib::ZPixmap);

        let bytes_per_pixel = (*img).bits_per_pixel / 8;

        let mut screenshot = Screenshot::new(width, height);

        //TODO: Handle propperly
        if bytes_per_pixel < 3
        {
            panic!("Bits per pixel is less than 3, X might be weird");
        }

        for pixel_index in 0..width * height
        {
            let pixel_address = pixel_index * bytes_per_pixel as u32;

            let offsets = (pixel_address, (pixel_address + 1), (pixel_address + 2));
            let pixel = (
                    *((*img).data.offset(offsets.2 as isize)) as u8
                    , *((*img).data.offset(offsets.1 as isize)) as u8
                    , *((*img).data.offset(offsets.0 as isize)) as u8
                );

            screenshot.push_pixel(pixel);
        }

        xlib::XDestroyImage(img);
        xlib::XCloseDisplay(display);

        screenshot
    }
}


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
    let texture_path = Path::new("media/fir1.png");
    let texture = glium::texture::SrgbTexture2d::new(&display, load_texture(texture_path)).unwrap();

    //let mut sprite = Sprite::new(&display, Arc::new(texture));
    let sprite_factory = SpriteFactory::new(&display);

    let mut sprite = sprite_factory.create_sprite(Arc::new(texture));

    let mut camera_state = CameraState::new();
    camera_state.set_position(na::Vector2::new(0., 0.));
    //camera_state.set_zoom(0.5);


    sprite.set_position(na::Vector2::new(100., 100.));
    //sprite.set_position(na::Vector2::new(0., 200.));
    sprite.set_origin(na::Vector2::new(0.5, 0.5));
    sprite.set_scale(na::Vector2::new(1., 1.));

    let grid = generate_grid(&display);


    let mut t: f32 = 0.;

    let mut mouse_pos = na::zero();

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

        //sprite.set_position(na::Vector2::new((t * 0.01).sin(), 0.));
        sprite.set_angle(t * 0.05);

        t += 0.05;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for line in &grid
        {
            line.draw(&mut target, &camera_state);
        }

        sprite.draw(&mut target, &camera_state);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(x, y) => {
                    let new_mouse = na::Vector2::new(x as f32, y as f32);
                    let moved = new_mouse - mouse_pos;

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

