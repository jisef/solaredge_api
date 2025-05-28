use serde::Deserialize;
use crate::meters::{Meter};


#[derive(Debug, Deserialize)]
pub struct PowerDetails {
    #[serde(rename = "timeUnit")]
    pub time_unit: String,
    pub unit: String,
    pub meters: Vec<Meter>,
}


#[derive(Debug, Deserialize)]
pub struct PowerDetailsWrapper {
    #[serde(rename = "powerDetails")]
    pub power_details: PowerDetails,
}


impl PowerDetailsWrapper {
    pub fn get_unit(&self) -> &str {
        self.power_details.unit.as_str()
    }
    
}
