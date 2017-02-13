extern crate nalgebra as na;

use glium::framebuffer::SimpleFrameBuffer;

use camera_state::CameraState;

use render_steps::RenderSteps;



pub trait Drawable
{
    fn draw(
            &self, 
            display: &mut SimpleFrameBuffer,
            render_step: RenderSteps,
            camera_state: &CameraState
        );
}



