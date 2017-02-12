use glium::texture::texture2d::Texture2d;
use glium::backend::Facade;
use glium::uniforms::{Uniforms, AsUniformValue, UniformsStorage};
use glium::{Program, VertexBuffer, Display};
use glium::framebuffer::SimpleFrameBuffer;
use glium;
use glium::Surface;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::boxed::Box;

use glium_types::Vertex;

pub const DEFAULT_VERTEX_SHADER: &'static str = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

trait RenderTargets<T>
    where T: Clone + Eq + PartialEq + Hash
{
    fn get_render_target<'a>(&self, target: &T) -> SimpleFrameBuffer;
}


#[derive(Clone, Eq, PartialEq, Hash)]
pub enum DefaultRenderStep
{
    Diffuse,
    Emissive,
}

//TODO: Make a trait for this
impl DefaultRenderStep
{
    pub fn get_hash_set() -> HashSet<DefaultRenderStep>
    {
        let set = HashSet::new();

        set.insert(DefaultRenderStep::Diffuse);
        set.insert(DefaultRenderStep::Emissive);

        set
    }
}

pub struct DefaultUniforms
{
    diffuse_texture: Texture2d,
    emissive_texture: Texture2d,
    ambient: f32,
}

impl DefaultUniforms
{
    pub fn new(facade: &Facade, resolution: (u32, u32)) -> DefaultUniforms
    {
        DefaultUniforms {
            diffuse_texture: Texture2d::empty(facade, resolution.0, resolution.1)
                .unwrap(),
            emissive_texture: Texture2d::empty(facade, resolution.0, resolution.1)
                .unwrap(),
            ambient: 0.
        }
    }
}

impl RenderTargets<DefaultRenderStep> for DefaultUniforms
{
    fn get_render_target(&self, target: &DefaultRenderStep) -> SimpleFrameBuffer
    {
        match *target
        {
            DefaultRenderStep::Diffuse => self.diffuse_texture.as_surface(),
            DefaultRenderStep::Emissive => self.emissive_texture.as_surface()
        }
    }
}


fn default_render_function(
            target: &mut glium::Frame,
            uniforms: &DefaultUniforms,
            vertex_buffer: &VertexBuffer<Vertex>,
            shader: &Program
        )
{
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let uniform_object = uniform!{
        diffuse_texture: &uniforms.diffuse_texture,
        emissive_texture: &uniforms.emissive_texture,
        ambient: &uniforms.ambient
    };


    target.draw(vertex_buffer, &indices, shader, &uniform_object,
                &Default::default()).unwrap();
}



pub struct RenderProcess<T, U, F>
    where T: Eq + PartialEq + Hash + Clone,
          U: Uniforms + RenderTargets<T>,
          F: Fn(&glium::Frame, &U, &VertexBuffer<Vertex>, &Program)
{
    steps: HashSet<T>,
    uniforms: U,

    vertices: VertexBuffer<Vertex>,
    shader: Program,

    render_function: F
}

impl<T, U, F> RenderProcess<T, U, F>
    where T: Eq + PartialEq + Hash + Clone,
          U: Uniforms + RenderTargets<T>,
          F: Fn(&mut glium::Frame, &U, &VertexBuffer<Vertex>, &Program)
{
    pub fn new(
                display: &Display,
                steps: HashSet<T>,
                uniforms: U,
                fragment_source: &str,
                render_function: F
            )
            -> RenderProcess<T, U, F>
    {
        let shape = vec!(
                //First triangle
                Vertex { position: (-1., -1.), tex_coords: (0., 0.) },
                Vertex { position: (-1., 1.), tex_coords: (0., 1.) },
                Vertex { position: (1., -1.), tex_coords: (1., 0.) },
                //Second triangle                                
                Vertex { position: (-1., 1.), tex_coords: (0., 1.) },
                Vertex { position: (1., 1.), tex_coords: (1., 1.) },
                Vertex { position: (1., -1.), tex_coords: (1., 0.) },
            );

        let vertices = VertexBuffer::new(display, &shape).unwrap();

        let shader = Program::from_source(
                    display, 
                    DEFAULT_VERTEX_SHADER, 
                    fragment_source, 
                    None
                ).unwrap();

        RenderProcess {
            steps: steps,
            uniforms: uniforms,

            vertices: vertices,
            shader: shader,
            render_function: render_function
        }
    }

    pub fn get_targets(&self) -> HashMap<T, SimpleFrameBuffer>
    {
        let mut map = HashMap::new();
        for step in &self.steps
        {
            map.insert(step.clone(), self.uniforms.get_render_target(step));
        }
        map
    }

    pub fn draw_to_display(&self, target: &mut glium::Frame)
    {
        render_function(target, &self.uniforms, &self.vertices, &self.shader);
    }

}

