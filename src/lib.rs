extern crate num_complex;

mod fractal;
mod camera_zoom;

pub use fractal::Fractal;
pub use fractal::Julia;
pub use camera_zoom::CameraZoom;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
