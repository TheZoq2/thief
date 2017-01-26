extern crate nalgebra as na;

use glium;

pub struct CameraState
{
    position: na::Vector2<f32>,
    zoom: f32
}

pub trait Drawable
{
    fn draw(&self, display: &glium::Frame, camera_state: CameraState);
}
