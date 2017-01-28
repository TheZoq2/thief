
extern crate nalgebra as na;

use glium;
use glium::texture::SrgbTexture2d;
use glium::Surface;

use std::sync::Arc;

use drawable;
use constants::{DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER};
use glium_types::{Vertex};
use camera_state::CameraState;
use drawing_util;


pub struct SpriteFactory
{
    vertex_buffer: Arc<glium::VertexBuffer<Vertex>>,
    shader: Arc<glium::Program>
}

impl SpriteFactory
{
    pub fn new(display: &glium::Display) -> SpriteFactory
    {
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

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let program = glium::Program::from_source(
                    display, 
                    DEFAULT_VERTEX_SHADER, 
                    DEFAULT_FRAGMENT_SHADER, 
                    None
                ).unwrap();

        SpriteFactory {
            vertex_buffer: Arc::new(vertex_buffer),
            shader: Arc::new(program)
        }
    }

    pub fn create_sprite(&self, texture: Arc<SrgbTexture2d>) -> Sprite
    {
        Sprite::new(self.vertex_buffer.clone(), self.shader.clone(), texture)
    }
}

pub struct Sprite
{
    position: na::Vector2<f32>,
    scale: na::Vector2<f32>,
    texture: Arc<SrgbTexture2d>,
    aspect_ratio: f32,
    depth: f32,

    texture_size: (u32, u32),

    origin: na::Vector2<f32>,

    vertices: Arc<glium::VertexBuffer<Vertex>>,
    shader: Arc<glium::Program>
}

impl Sprite
{
    fn new(
            vertex_buffer: Arc<glium::VertexBuffer<Vertex>>,
            shader: Arc<glium::Program>,
            texture: Arc<SrgbTexture2d>
        ) -> Sprite
    {
        let aspect_ratio = (texture.get_width() as f32) 
                / (texture.get_height().unwrap() as f32);

        Sprite
        {
            position: na::zero(),
            scale: na::one(),
            texture: texture,
            aspect_ratio: aspect_ratio,
            depth: 0.,

            origin: na::zero(),

            vertices: vertex_buffer,
            shader: shader
        }
    }

    pub fn set_position(&mut self, position: na::Vector2<f32>)
    {
        self.position = position;
    }

    pub fn set_origin(&mut self, origin: na::Vector2<f32>)
    {
        self.origin = origin;
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
