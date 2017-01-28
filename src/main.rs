#![allow(dead_code)]

extern crate x11;
extern crate libc;
extern crate nalgebra as na;

#[macro_use]
extern crate glium;

mod drawable;
mod drawing_util;
mod camera_state;
mod constants;
mod sprite;
mod glium_types;

use drawable::{Drawable};
use sprite::{SpriteFactory};

use x11::xlib;
use std::mem;

use std::sync::Arc;

use glium::texture::{RawImage2d};

use camera_state::CameraState;

use glium_types::{Pixel};


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



pub fn run_selector()
{
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let screenshot = capture_screenshot();

    let image = screenshot.get_glium_image();
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    //let mut sprite = Sprite::new(&display, Arc::new(texture));
    let sprite_factory = SpriteFactory::new(&display);

    let mut sprite = sprite_factory.create_sprite(Arc::new(texture));

    let mut camera_state = CameraState::new();

    //camera_state.set_position(na::Vector2::new(-0.5, -0.5))
    
    //sprite.set_position(na::Vector2::new(0.25, 0.25));
    sprite.set_position(na::Vector2::new(0., 0.5));
    sprite.set_origin(na::Vector2::new(0.5, 0.5));

    let t = 0.;
    loop {
        t += 0.05;
        sprite.set_position(na::Vector2::new(t.sin(), 0.));

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        sprite.draw(&mut target, &camera_state);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

fn main() {
    run_selector();
}




//#[cfg(test)]
//mod benchmarks
//{
//    use super::*;
//    extern crate test;
//
//    #[bench]
//    fn screenshot_benchmark(b: &mut test::Bencher) {
//        b.iter(|| {
//            let screenshot = capture_screenshot();
//        })
//    }
//}
