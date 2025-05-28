use reqwest::Error;
use crate::meters::{meters_to_string, Meters};
use crate::{EnergyDetails};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt;
use std::fmt::{Debug, Formatter};


#[derive(Debug)]
pub enum TimeUnit {
    QUARTER_OF_AN_HOUR,
    HOUR,
    DAY,
    WEEK,
    MONTH,
    YEAR
}
impl fmt::Display for TimeUnit{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
/// Represents the Response from the API
#[derive(Debug, Deserialize)]
pub struct EnergyDetailsResponse {
    #[serde(rename = "energyDetails")]
    pub energy_details: EnergyDetails,
}
impl EnergyDetailsResponse {
    /// gets the unit of the response
    pub fn get_unit(&self) -> &str{ 
        &*self.energy_details.unit
    }
    /// gets the average for a meter in the response
    pub fn get_average_for_meter(self, meter: Meters) -> f64 {
        let mut average: f64 = 0.0;
        let mut count =0;
        for x in self.energy_details.meters {
            if x.meter_type.eq_ignore_ascii_case(&*meter.to_string()) { 
                for value in x.values {
                    if value.value.is_some() {
                        average = average + value.value.unwrap();
                        count += 1;
                    }
                }
            }
        }
        average
    }
}
