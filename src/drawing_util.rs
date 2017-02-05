extern crate nalgebra as na;


pub fn get_window_scaling_matrix(window_size: (f32, f32)) -> na::Matrix4<f32>
{
    let (x, y) = window_size;

    let (x, y) = (x/2., y/2.);

    na::Matrix4::new(
            1./x, 0.,   0., 0.,
            0.,   1./y, 0., 0.,
            0.,   0.,   1., 0.,
            0.,   0.,   0., 1.
        )
}

