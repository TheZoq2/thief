#![feature(test)]

extern crate x11;
extern crate libc;

#[macro_use]
extern crate glium;

use std::io::Cursor;

use x11::xlib;
use std::mem;

use glium::texture::RawImage2d;

type Pixel = (u8, u8, u8);

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
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let screenshot = capture_screenshot();

    //let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), dimensions);
    let image = screenshot.get_glium_image();
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let shape = vec![
            //First triangle
            Vertex { position: [0., 0.], tex_coords: [0., 0.] },
            Vertex { position: [0., 1.], tex_coords: [0., 1.] },
            Vertex { position: [1., 0.], tex_coords: [1., 0.] },
            //Second triangle                                
            Vertex { position: [0., 1.], tex_coords: [0., 1.] },
            Vertex { position: [1., 1.], tex_coords: [1., 1.] },
            Vertex { position: [1., 0.], tex_coords: [1., 0.] },
        ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform mat4 matrix;
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t = -0.5;

    loop {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0 , 0.0, 0.0, 0.0],
                [0.0 , 1.0, 0.0, 0.0],
                [0.0 , 0.0, 1.0, 0.0],
                [-0.5, -0.5, 0.0, 1.0f32],
            ],
            tex: &texture,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
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
