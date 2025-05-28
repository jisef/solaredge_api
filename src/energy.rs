use reqwest::Error;
use crate::meters::{meters_to_string, Meters};
use crate::{fetch_and_parse, EnergyDetails};
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

#[derive(Debug, Deserialize)]
pub struct EnergyDetailsResponse {
    #[serde(rename = "energyDetails")]
    pub energy_details: EnergyDetails,
}
impl EnergyDetailsResponse {
    pub fn get_unit(&self) -> &str{ 
        &*self.energy_details.unit
    }
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

pub async fn get_site_energy(api_key: String, site_id: String, meters: Vec<Meters>, time_unit: TimeUnit, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<EnergyDetailsResponse, Error> {
    let from = from.format("%Y-%m-%d%%20%H:%M:%S").to_string();
    let to = to.format("%Y-%m-%d%%20%H:%M:%S").to_string();
    let url = format!(
        "https://monitoringapi.solaredge.com/site/{}/energyDetails?meters={}&timeUnit={}&startTime={}&endTime={}&api_key={}",
        site_id,
        meters_to_string(meters),
        time_unit.to_string(),
        from,
        to,
        api_key
    );

    fetch_and_parse::<EnergyDetailsResponse>(url).await
}

