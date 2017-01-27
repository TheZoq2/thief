extern crate nalgebra as na;

use glium;

pub struct CameraState
{
    position: na::Vector2<f32>,
    zoom: f32
}

impl CameraState
{
    pub fn new() -> CameraState
    {
        CameraState {
            position: na::one(),
            zoom: 0.
        }
    }
}

pub trait Drawable
{
    fn draw(&self, display: &mut glium::Frame, camera_state: CameraState);
}



