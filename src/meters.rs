use std::fmt;
use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::MeterValue;

#[derive(Debug, Clone)]
pub enum Meters {
    Production,
    Purchased,
    Consumption,
    SelfConsumption,
    FeedIn,
}

impl Display for Meters {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn meters_to_string(meters: Vec<Meters>) -> String {
    let mut to_return: String = String::from("");

    let mut i = 0;
    while i < meters.len() {
        let meter = meters.get(i).expect("Meters are not valid");
        to_return.push_str(&*meter.to_string().to_uppercase());
        if i < meters.len() - 1 {
            to_return.push_str(",")
        }

        i += 1;
    }

    to_return
}

#[derive(Debug, Deserialize)]
pub struct Meter {
    #[serde(rename = "type")]
    pub meter_type: String,
    pub values: Vec<MeterValue>,
}
