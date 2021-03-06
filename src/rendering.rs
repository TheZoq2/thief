use glium::{Program, VertexBuffer, Display};
use glium::framebuffer::SimpleFrameBuffer;
use glium;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use glium_types::Vertex;


pub const VERTEX_SHADER: &'static str = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

pub trait RenderTargets<T>
    where T: Clone + Eq + PartialEq + Hash
{
    fn get_render_target<'a>(&self, target: &T) -> SimpleFrameBuffer;
}

pub struct RenderProcess<T, U, F>
    where T: Eq + PartialEq + Hash + Clone,
          U: RenderTargets<T>,
          F: Fn(&mut glium::Frame, &U, &VertexBuffer<Vertex>, &Program)
{
    steps: HashSet<T>,
    uniforms: U,

    vertices: VertexBuffer<Vertex>,
    shader: Program,

    render_function: F
}

impl<T, U, F> RenderProcess<T, U, F>
    where T: Eq + PartialEq + Hash + Clone,
          U: RenderTargets<T>,
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
                    VERTEX_SHADER, 
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
        (self.render_function)(target, &self.uniforms, &self.vertices, &self.shader);
    }
}

