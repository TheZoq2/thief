extern crate nalgebra as na;

use glium::framebuffer::SimpleFrameBuffer;

use camera_state::CameraState;


pub trait Drawable
{
    fn draw(&self, display: &mut SimpleFrameBuffer, camera_state: &CameraState);
}



