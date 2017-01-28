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
        let scale = 1. / self.zoom;
        let offset = - self.position;

        na::Matrix4::new(
                scale   , 0.      , 0., offset.x,
                0.      , scale   , 0., offset.y,
                0.      , 0.      , 1., 0.      ,
                0.      , 0.      , 0., 1.      ,
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

        println!("{}", state.get_matrix());

        let translated = state.get_matrix() * vector;

        assert_eq!(translated, na::Vector4::new(-4., -3., 0., 1.));
    }
}
