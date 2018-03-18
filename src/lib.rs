#![feature(slice_patterns)]

extern crate num_complex;

mod fractal;
mod camera;

pub use fractal::Fractal;
pub use fractal::Julia;
pub use camera::Camera;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
