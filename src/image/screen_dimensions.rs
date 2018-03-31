use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct ScreenDimensions(pub u32, pub u32);

impl ScreenDimensions {
    pub fn as_tuple(&self) -> (u32, u32) {
        let ScreenDimensions(width, height) = *self;
        (width, height)
    }
}

impl FromStr for ScreenDimensions {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let mut splitted = s.split('x');

        let invalid_msg = "invalid dimension format";

        let width = splitted.next().ok_or(invalid_msg)?;
        let height = splitted.next().ok_or(invalid_msg)?;
        if splitted.next().is_some() {
            return Err(invalid_msg)
        }

        let width = width.parse().map_err(|_| "invalid width")?;
        let height = height.parse().map_err(|_| "invalid height")?;

        if width == 0 || height == 0 {
            return Err("dimensions cannot be zero")
        }

        Ok(ScreenDimensions(width, height))
    }
}
