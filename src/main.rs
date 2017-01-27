extern crate x11;
extern crate libc;
extern crate nalgebra as na;

#[macro_use]
extern crate glium;

mod drawable;
mod drawing_util;
mod camera_state;

use drawable::{Drawable};

use x11::xlib;
use std::mem;

use std::sync::Arc;

use glium::texture::{RawImage2d, Texture2d};
use glium::texture::srgb_texture2d::SrgbTexture2d;
use glium::{Surface};

use camera_state::CameraState;

const default_vertex_shader: &'static str = r#"
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
const default_fragment_shader: &'static str = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

type Pixel = (u8, u8, u8);

#[derive(Copy, Clone)]
struct Vertex {
    position: (f32, f32),
    tex_coords: (f32, f32),
}

implement_vertex!(Vertex, position, tex_coords);

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



pub struct Sprite
{
    position: na::Vector2<f32>,
    scale: na::Vector2<f32>,
    texture: Arc<SrgbTexture2d>,
    aspect_ratio: f32,
    depth: f32,

    origin: na::Vector2<f32>,

    vertices: Arc<glium::VertexBuffer<Vertex>>,
    shader: Arc<glium::Program>
}

impl Sprite
{
    pub fn new(display: &glium::Display, texture: Arc<SrgbTexture2d>) -> Sprite
    {
        let aspect_ratio = (texture.get_width() as f32) 
                / (texture.get_height().unwrap() as f32);

        let shape = vec!(
                //First triangle
                Vertex { position: (0., 0.), tex_coords: (0., 0.) },
                Vertex { position: (0., 1.), tex_coords: (0., 1.) },
                Vertex { position: (1., 0.), tex_coords: (1., 0.) },
                //Second triangle                                
                Vertex { position: (0., 1.), tex_coords: (0., 1.) },
                Vertex { position: (1., 1.), tex_coords: (1., 1.) },
                Vertex { position: (1., 0.), tex_coords: (1., 0.) },
            );


        let program = glium::Program::from_source(
                    display, 
                    default_vertex_shader, 
                    default_fragment_shader, 
                    None
                ).unwrap();
        Sprite
        {
            position: na::zero(),
            scale: na::one(),
            texture: texture,
            aspect_ratio: aspect_ratio,
            depth: 0.,

            origin: na::zero(),

            vertices: Arc::new(glium::VertexBuffer::new(display, &shape).unwrap()),
            shader: Arc::new(program)
        }
    }

    pub fn set_position(&mut self, position: na::Vector2<f32>)
    {
        self.position = position;
    }
}

impl drawable::Drawable for Sprite
{
    fn draw(&self, target: &mut glium::Frame, camera_state: &CameraState)
    {
        let (width, height) = target.get_dimensions();
        let window_aspect_ratio = drawing_util::calculate_aspect_ratio(
                width as f32,
                height as f32
            );

        let scale_x = self.scale.x * self.aspect_ratio;
        let scale_y = self.scale.y * window_aspect_ratio;

        let x_offset = -scale_x * self.origin.x + self.position.x;
        let y_offset = -scale_y * self.origin.y + self.position.y;

        let matrix = na::Matrix4::new(
                scale_x, 0.     , 0., x_offset,
                0.     , scale_y, 0., y_offset,
                0.     , 0.     , 1., 0.,
                0.     , 0.     , 0., 1.
            );

        let final_matrix = matrix * camera_state.get_matrix();


        let uniforms = uniform! {
            matrix: *final_matrix.as_ref(),
            tex: &*self.texture,
        };

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.draw(&*self.vertices, &indices, &*self.shader, &uniforms,
                    &Default::default()).unwrap();
    }
}

pub fn run_selector()
{
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let screenshot = capture_screenshot();
    let aspect_ratio = screenshot.get_aspect_ratio();

    let image = screenshot.get_glium_image();
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let mut sprite = Sprite::new(&display, Arc::new(texture));

    let mut camera_state = CameraState::new();

    //camera_state.set_position(na::Vector2::new(-0.5, -0.5))
    
    sprite.set_position(na::Vector2::new(0.25, 0.25));

    loop {
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
