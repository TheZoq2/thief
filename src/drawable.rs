extern crate nalgebra as na;

use glium;

use camera_state::CameraState;


pub trait Drawable
{
    fn draw(&self, display: &mut glium::Frame, camera_state: &CameraState);
}



