use std::str::FromStr;

fn is_power_of_four(n: u32) -> bool {
    n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
}

#[derive(Debug, Copy, Clone)]
pub struct Antialiazing(pub u32);

impl FromStr for Antialiazing {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim().parse().map_err(|_| "invalid number")?;

        if !is_power_of_four(value) {
            return Err("number is not a power of 4")
        }

        Ok(Antialiazing(value))
    }
}
