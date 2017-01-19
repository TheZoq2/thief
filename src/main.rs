#![feature(test)]

extern crate x11;
extern crate libc;

use x11::xlib;
use std::mem;

pub struct Screenshot
{
    width: u32,
    height: u32,
    pixels: Vec<(u8, u8, u8)>
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

    fn push_pixel(&mut self, pixel: (u8, u8, u8))
    {
        self.pixels.push(pixel)
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

        let bits_per_pixel = (*img).bits_per_pixel;

        let mut screenshot = Screenshot::new(width, height);

        //TODO: Handle propperly
        if bits_per_pixel < 3
        {
            panic!("Bits per pixel is less than 3, X might be weird");
        }

        for pixel_index in 0..width * height
        {
            let offsets = (pixel_index, (pixel_index + 1), (pixel_index + 2));
            let pixel = (
                    *((*img).data.offset(offsets.0 as isize)) as u8
                    , *((*img).data.offset(offsets.1 as isize)) as u8
                    , *((*img).data.offset(offsets.2 as isize)) as u8
                );

            screenshot.push_pixel(pixel);
        }

        xlib::XDestroyImage(img);
        xlib::XCloseDisplay(display);

        screenshot
    }
}

fn main() {
    println!("Hello, world!");

    let screenshot = capture_screenshot();
}




#[cfg(test)]
mod benchmarks
{
    use super::*;
    extern crate test;

    #[bench]
    fn screenshot_benchmark(b: &mut test::Bencher) {
        b.iter(|| {
            let screenshot = capture_screenshot();
        })
    }
}
