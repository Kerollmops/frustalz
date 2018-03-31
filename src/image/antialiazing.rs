use std::str::FromStr;

fn is_power_of_four(n: u32) -> bool {
    n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
}

#[derive(Debug, Copy, Clone)]
pub struct Antialiazing(u32);

impl Antialiazing {
    pub fn new(value: u32) -> Option<Self> {
        if is_power_of_four(value) {
            Some(Antialiazing(value))
        } else {
            None
        }
    }
}

impl From<Antialiazing> for u32 {
    fn from(antialiazing: Antialiazing) -> Self {
        antialiazing.0
    }
}

impl FromStr for Antialiazing {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim().parse().map_err(|_| "invalid number")?;

        Antialiazing::new(value).ok_or("number is not a power of 4")
    }
}
