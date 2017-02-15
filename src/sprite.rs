extern crate nalgebra as na;

use glium;
use glium::texture::SrgbTexture2d;
use glium::Surface;
use glium::framebuffer::SimpleFrameBuffer;
use glium::draw_parameters::DrawParameters;

use std::sync::Arc;

use drawable;
use constants::{DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER};
use glium_types::{Vertex};
use camera_state::CameraState;
use drawing_util;

use render_steps::RenderSteps;

use std::collections::HashMap;


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
    angle: f32,
    textures: HashMap<RenderSteps, Option<Arc<SrgbTexture2d>>>,
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
        let texture_x = texture.get_width();
        let texture_y = texture.get_height().unwrap();

        let aspect_ratio = (texture_x as f32) 
                / (texture_y as f32);

        let mut textures = HashMap::new();
        textures.insert(RenderSteps::Diffuse, Some(texture));

        Sprite
        {
            position: na::zero(),
            scale: na::one(),
            angle: 0.,
            textures: textures,
            aspect_ratio: aspect_ratio,
            depth: 0.,

            texture_size: (texture_x, texture_y),

            origin: na::zero(),

            vertices: vertex_buffer,
            shader: shader
        }
    }

    pub fn set_position(&mut self, position: na::Vector2<f32>)
    {
        self.position = position;
    }
    pub fn get_position(&self) -> na::Vector2<f32>
    {
        return self.position;
    }

    pub fn set_origin(&mut self, origin: na::Vector2<f32>)
    {
        self.origin = origin;
    }

    pub fn set_scale(&mut self, scale: na::Vector2<f32>)
    {
        self.scale = scale;
    }

    pub fn set_angle(&mut self, angle: f32)
    {
        self.angle = angle;
    }

    pub fn get_angle(&self) -> f32
    {
        return self.angle;
    }

    pub fn set_additional_texture(&mut self, step: RenderSteps, texture: Arc<SrgbTexture2d>)
    {
        self.textures.insert(step, Some(texture));
    }
}

impl drawable::Drawable for Sprite
{
    fn draw(&self, target: &mut SimpleFrameBuffer, step: &RenderSteps, camera_state: &CameraState)
    {
        match self.textures.get(step)
        {
            Some(&Some(ref texture)) => {
                let matrix = generate_default_matrix(
                        self.scale
                        , self.texture_size
                        , self.position
                        , self.origin
                        , self.angle
                        , target.get_dimensions()
                        , camera_state
                    );


                let texture = &**texture;
                let uniforms = uniform! {
                    matrix: *matrix.as_ref(),
                    tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                };

                let draw_parameters = DrawParameters{
                    blend: glium::draw_parameters::Blend::alpha_blending(),
                    .. Default::default()
                };

                let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
                target.draw(&*self.vertices, &indices, &*self.shader, &uniforms,
                            &draw_parameters).unwrap();
            },
            _ => {}
        }
    }
}


/**
  Calculates a transform matrix for positioning things based on pixel positions
  instead of floats

  ## Parameters:
  scale: 
  vector of 2 floats where (1.0, 1.0) means 1 pixel on the sprite
  should map to one pixel on the screen

  texture_size:
  The size of the sprites texture in pixels

  position:
  position of the sprite within the world in pixels

  origin:
  Vector between 0 and 1 which specifies what point on the sprite to make
  the center

  target_size:
  The size of the render target in pixels

  ## Returns
  A transformation matrix for rendering the sprite
*/
pub fn generate_default_matrix(
            scale: na::Vector2<f32>,
            texture_size: (u32, u32),
            position: na::Vector2<f32>,
            origin: na::Vector2<f32>,
            angle: f32,
            target_size: (u32, u32),
            camera_state: &CameraState
        ) -> na::Matrix4<f32>
{
    let scale_x = scale.x * texture_size.0 as f32;
    let scale_y = scale.y * texture_size.1 as f32;

    let local_translation_matrix = na::Matrix4::new(
            1., 0., 0., -origin.x,
            0., 1., 0., -origin.y,
            0., 0., 1., 0.,
            0., 0., 0., 1.
        );

    let local_matrix = na::Matrix4::new(
            scale_x * angle.cos(), -scale_y * angle.sin(), 0., 0.,
            scale_x * angle.sin(), scale_y * angle.cos() , 0., 0.,
            0.                   , 0.                    , 1., 0.,
            0.                   , 0.                    , 0., 1.
        ) * local_translation_matrix;

    let global_translation_matrix = na::Matrix4::new(
            1., 0., 0., position.x,
            0., 1., 0., position.y,
            0., 0., 1., 0.,
            0., 0., 0., 1.
        );

    let world_matrix = camera_state.get_matrix()
        * drawing_util::get_window_scaling_matrix(
                    (target_size.0 as f32, target_size.1 as f32)
                );

    let final_matrix = world_matrix * global_translation_matrix * local_matrix;

    //final_matrix
    final_matrix
}




#[cfg(test)]
mod tests
{
    use na;
    use super::*;

    #[test]
    fn test_default_matrix_scale()
    {
        let scale = na::Vector2::new(2., 2.);
        let texture_size = (100, 100);
        let position = na::zero();
        let origin = na::zero();
        let target_size = (100, 50);
        let camera_state = CameraState::new();
        let angle = 0.;

        let result = generate_default_matrix(
                scale,
                texture_size,
                position,
                origin,
                angle,
                target_size,
                &camera_state
            );

        let desired = na::Matrix4::new(
                2., 0., 0., 0.,
                0., 4., 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.
            );

        assert_eq!(desired, result);
    }

    #[test]
    fn test_default_matrix_position_and_scale()
    {
        let scale = na::Vector2::new(2., 2.);
        let texture_size = (100, 100);
        let position = na::Vector2::new(50., 50.);
        let origin = na::zero();
        let target_size = (100, 50);
        let camera_state = CameraState::new();
        let angle = 0.;

        let result = generate_default_matrix(
                scale,
                texture_size,
                position,
                origin,
                angle,
                target_size,
                &camera_state
            );

        let desired = na::Matrix4::new(
                2., 0., 0., 0.5,
                0., 4., 0., 1.,
                0., 0., 1., 0.,
                0., 0., 0., 1.
            );

        assert_eq!(desired, result);
    }

    fn test_default_matrix_position_scale_and_origin()
    {
        let scale = na::Vector2::new(2., 2.);
        let texture_size = (100, 100);
        let position = na::Vector2::new(50., 50.);
        let origin = na::Vector2::new(0.5, 0.5);
        let target_size = (100, 50);
        let camera_state = CameraState::new();
        let angle = 0.;

        let result = generate_default_matrix(
                scale,
                texture_size,
                position,
                origin,
                angle,
                target_size,
                &camera_state
            );

        let position = na::Vector2::new(
                (scale.x * texture_size.0 as f32 / target_size.0 as f32),
                (scale.y * texture_size.1 as f32 / target_size.1 as f32)
            );

        let desired = na::Matrix4::new(
                2., 0., 0., position.x,
                0., 4., 0., position.y,
                0., 0., 1., 0.,
                0., 0., 0., 1.
            );

        assert_eq!(desired, result);
    }
}
