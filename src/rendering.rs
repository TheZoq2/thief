use glium::texture::texture2d::Texture2d;
use glium::framebuffer::SimpleFrameBuffer;
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


#[derive(Clone, Eq, PartialEq, Hash)]
enum DefaultRenderStep
{
    Diffuse,
    Emissive,
}

//fn default_uniform_generator_function(
//            targets: HashMap<DefaultRenderStep, Texture2d>,
//        )
//    -> Box<Uniforms>
//{
//        Box::new(uniform! {
//            diffuse_texture: targets[DefaultRenderStep::Diffuse].unwrap(),
//            emissive_texture: targets[DefaultRenderStep::Emissive].unwrap(),
//            ambient_light: 0.25 as f32
//        })
//}

struct RenderProcess<'n, T, F, UT, UR>
    where F: Fn(&HashMap<T, Texture2d>) -> UniformsStorage<'n, UT, UR>,
          T: Eq + PartialEq + Hash + Clone,
          UT: AsUniformValue,
          UR: Uniforms
{
    steps: HashSet<T>,
    uniform_function: F,

    vertices: VertexBuffer<Vertex>,
    shader: Program,
}

impl<'n, T, F, UT, UR> RenderProcess<'n, T, F, UT, UR>
    where F: Fn(&HashMap<T, Texture2d>) -> UniformsStorage<'n, UT, UR>,
          T: Eq + PartialEq + Hash + Clone,
          UT: AsUniformValue,
          UR: Uniforms
{
    pub fn new(
                display: &Display,
                steps: HashSet<T>,
                uniform_function: F,
                fragment_source: &str
            )
            -> RenderProcess<'n, T, F, UT, UR>
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
            uniform_function: uniform_function,

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


