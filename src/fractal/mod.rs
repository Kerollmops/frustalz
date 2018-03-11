mod julia;

pub use self::julia::Julia;

pub trait Fractal {
    fn iterations(&self, x: f64, y: f64) -> u8;
}
