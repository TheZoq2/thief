use glium::texture::texture2d::Texture2d;
use glium::framebuffer::SimpleFrameBuffer;
use glium::backend::Facade;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq, Hash)]
enum DefaultRenderStep
{
    Diffuse,
    Emissive,
}

struct RenderProcess<T, F> 
    where F: Fn(&HashMap<T, SimpleFrameBuffer>),
          T: Eq + PartialEq + Hash + Clone
{
    steps: HashSet<T>,
    combine_function: F
}

impl<T, F> RenderProcess<T, F>
    where F: Fn(&HashMap<T, SimpleFrameBuffer>),
          T: Eq + PartialEq + Hash + Clone
{
    pub fn new(steps: HashSet<T>, combine_function: F) -> RenderProcess<T, F>
    {
        RenderProcess {
            steps: steps,
            combine_function: combine_function
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


