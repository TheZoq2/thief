extern crate nalgebra as na;

use camera_state::CameraState;

use glium;
use glium::Surface;

use glium_types::{Vertex};

use drawable::Drawable;
use drawing_util;


pub const LINE_VERTEX_SHADER: &'static str = r#"
        #version 140
        in vec2 position;
        uniform mat4 matrix;
        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
pub const LINE_FRAGMENT_SHADER: &'static str = r#"
        #version 140
        uniform vec4 line_color;
        out vec4 color;
        void main() {
            color = line_color;
        }
    "#;

pub struct Line
{
    start: na::Vector2<f32>,
    end: na::Vector2<f32>,
    color: (f32, f32, f32, f32),
    vertices: glium::VertexBuffer<Vertex>,
    shader: glium::Program,
}

impl Line
{
    pub fn new(display: &glium::Display, start: na::Vector2<f32>, end: na::Vector2<f32>) -> Line
    {
        let shape = vec!(
                //First triangle
                Vertex { position: (start.x, start.y), tex_coords: (0., 0.) },
                Vertex { position: (end.x, end.y), tex_coords: (0., 1.) },
                Vertex { position: (start.x, start.y), tex_coords: (1., 0.) },
            );

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let program = glium::Program::from_source(
                    display, 
                    LINE_VERTEX_SHADER, 
                    LINE_FRAGMENT_SHADER, 
                    None
                ).unwrap();

        Line {
            start: start,
            end: end,
            color: (1., 1., 1., 1.),
            vertices: vertex_buffer,
            shader: program
        }
    }

    pub fn with_color(mut self, color: (f32, f32, f32, f32)) -> Line
    {
        self.color = color;
        self
    }
}

impl Drawable for Line
{
    fn draw(&self, target: &mut glium::Frame, camera_state: &CameraState)
    {
        let (target_width, target_height) = target.get_dimensions();

        let world_matrix = camera_state.get_matrix()
            * drawing_util::get_window_scaling_matrix((target_width as f32, target_height as f32));

        let final_matrix = world_matrix;
        
        let uniforms = uniform! {
            matrix: *final_matrix.as_ref(),
            line_color: self.color
        };

        let params = glium::draw_parameters::DrawParameters{
            polygon_mode: glium::draw_parameters::PolygonMode::Line,
            .. Default::default()
        };

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        target.draw(&self.vertices, &indices, &self.shader, &uniforms,
                    &params).unwrap();
    }
}
