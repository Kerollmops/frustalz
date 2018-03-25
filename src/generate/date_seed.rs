use std::str::FromStr;
use chrono::{Utc, DateTime, Timelike};

fn floor_to_hour(datetime: DateTime<Utc>) -> Option<DateTime<Utc>> {
    datetime
        .with_minute(0)?
        .with_second(0)?
        .with_nanosecond(0)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DateSeed(pub DateTime<Utc>);

impl FromStr for DateSeed {
    type Err = <DateTime<Utc> as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let datetime = DateTime::from_str(s)?;
        let datetime = floor_to_hour(datetime).expect("not possible floor to hour");

        Ok(DateSeed(datetime))
    }
}

impl Default for DateSeed {
    fn default() -> Self {
        let datetime = floor_to_hour(Utc::now()).expect("not possible floor to hour");

        DateSeed(datetime)
    }
}
