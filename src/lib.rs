extern crate num_complex;

mod fractal;

pub use fractal::Fractal;
pub use fractal::Julia;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
