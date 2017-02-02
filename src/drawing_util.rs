extern crate nalgebra as na;

/**
  Calculates the aspect ratio between x and y
 */
pub fn calculate_aspect_ratio(x: f32, y: f32) -> f32
{
    x / y
}


pub fn get_window_scaling_matrix(window_size: (f32, f32)) -> na::Matrix4<f32>
{
    let (x, y) = window_size;

    na::Matrix4::new(
            1./x, 0.,   0., 0.,
            0.,   1./y, 0., 0.,
            0.,   0.,   1., 0.,
            0.,   0.,   0., 1.
        )
}
