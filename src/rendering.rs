use glium::texture::texture2d::Texture2d;
use glium::backend::Facade;
use glium::uniforms::{Uniforms, AsUniformValue, UniformsStorage};
use glium::{Program, VertexBuffer, Display};

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
    fn get_render_target<'a>(&self, target: T) -> &Texture2d;
}


#[derive(Clone, Eq, PartialEq, Hash)]
enum DefaultRenderStep
{
    Diffuse,
    Emissive,
}

#[uniform]
struct DefaultUniforms
{
    diffuse_texture: Texture2d,
    emissive_texture: Texture2d,
    ambient: f32,
}

impl DefaultUniforms
{
    pub fn new(&self, facade: &Facade, resolution: (u32, u32)) -> DefaultUniforms
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
    fn get_render_target(&self, target: DefaultRenderStep) -> &Texture2d
    {
        match target
        {
            DefaultRenderStep::Diffuse => &self.diffuse_texture,
            DefaultRenderStep::Emissive => &self.emissive_texture
        }
    }
}




struct RenderProcess<T, U>
    where T: Eq + PartialEq + Hash + Clone,
          U: Uniforms + RenderTargets<T>
{
    steps: HashSet<T>,
    uniforms: U,

    vertices: VertexBuffer<Vertex>,
    shader: Program,
}

impl<T, U> RenderProcess<T, U>
    where T: Eq + PartialEq + Hash + Clone,
          U: Uniforms + RenderTargets<T>
{
    pub fn new(
                display: &Display,
                steps: HashSet<T>,
                uniforms: U,
                fragment_source: &str
            )
            -> RenderProcess<T, U>
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
            shader: shader
        }
    }

    pub fn generate_target_textures(&self, facade: &Facade, resolution: (u32, u32)) 
            -> HashMap<T, Texture2d>
    {
        let mut result = HashMap::new();

        for step in &self.steps
        {
            let texture = Texture2d::empty(facade, resolution.0, resolution.1)
                    .unwrap();

            result.insert(step.clone(), texture);
        }

        result
    }
}

