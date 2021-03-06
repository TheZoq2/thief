extern crate nalgebra as na;

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
            position: na::zero(),
            zoom: 1.
        }
    }

    pub fn get_matrix(&self) -> na::Matrix4<f32>
    {
        let offset = -self.position * self.zoom;

        na::Matrix4::new(
                self.zoom, 0.       , 0., offset.x,
                0.       , self.zoom, 0., offset.y,
                0.       , 0.       , 1., 0.,
                0.       , 0.       , 0., 1.,
            )
    }

    pub fn set_position(&mut self, new_pos: na::Vector2<f32>)
    {
        self.position = new_pos;
    }

    pub fn set_zoom(&mut self, zoom: f32)
    {
        self.zoom = zoom;
    }

    pub fn get_position(&self) -> na::Vector2<f32>
    {
        return self.position;
    }
}


#[cfg(test)]
mod tests
{
    extern crate nalgebra as na;
    use super::CameraState;

    #[test]
    fn matrix_translation_test()
    {
        let vector = na::Vector4::new(
                1.,
                2.,
                0.,
                1.
            );

        let mut state = CameraState::new();
        state.set_position(na::Vector2::new(5., 5.));

        let scaling_matrix = na::one::<na::Matrix4<f32>>();
        let translated = (scaling_matrix + state.get_position_matrix((1,1))) * vector;

        assert_eq!(translated, na::Vector4::new(-4., -3., 0., 1.));
    }

    fn matrix_translation_with_window_test()
    {
        let mut state = CameraState::new();
        state.set_position(na::Vector2::new(100., 50.));

        let desired_matrix = na::Matrix4::new(
                0., 0., 0., -1. ,
                0., 0., 0., -0.5,
                0., 0., 0., 0.  ,
                0., 0., 0., 0.
            );

        assert_eq!(desired_matrix, state.get_position_matrix((100, 100)));
    }
}
