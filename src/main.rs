mod meters;
mod power;
mod energy;

use crate::energy::TimeUnit;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use http_adapter_reqwest::reqwest;
use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use crate::meters::Meters;

/*pub async fn fetch_and_parse<T>(url: String) -> Result<T, Error> where T: DeserializeOwned {
    let response = reqwest::get(url.clone())
        .await?;

    match response.json::<T>().await {
        Ok(data) => {Ok(data)}
        Err(err) => {Err(err)}
    }
}*/

/// Gets all PowerDetails


/// Returns the average of PowerDetails and the Unit
/// Hashmap<String, f64>, String
async fn get_average_power(api_key: String, site_id: String) -> (HashMap<String, f64>, String) {
    let url = format!(
        "https://monitoringapi.solaredge.com/site/{}/powerDetails?meters=PRODUCTION,CONSUMPTION&startTime=2025-04-1%2011:00:00&endTime=2025-04-30%2013:00:00&api_key={}",
        site_id, api_key
    );
    let pd = reqwest::get(url)
        .await
        .unwrap()
        .json::<PowerDetailsWrapper>()
        .await
        .unwrap();
    let unit = pd.power_details.unit;
    let mut map: HashMap<String, f64> = HashMap::new();
    for meter in pd.power_details.meters {
        let mut avg = 0.0;
        let mut avg_count = 0.0;
        for x in meter.values {
            if x.value.is_some() {
                avg += x.value.unwrap();
                avg_count += 1.0;
            }
        }
        let avg = avg / avg_count;
        map.insert(meter.meter_type, avg);
    }

    (map, unit)
}

#[derive(Debug, Deserialize)]
pub struct PowerDetailsWrapper {
    #[serde(rename = "powerDetails")]
    pub power_details: PowerDetails,
}

#[derive(Debug, Deserialize)]
pub struct PowerDetails {
    #[serde(rename = "timeUnit")]
    pub time_unit: String,
    pub unit: String,
    pub meters: Vec<Meter>,
}

#[derive(Debug, Deserialize)]
pub struct Meter {
    #[serde(rename = "type")]
    pub meter_type: String,
    pub values: Vec<MeterValue>,
}

#[derive(Debug, Deserialize)]
pub struct MeterValue {
    pub date: String,
    pub value: Option<f64>,
}



#[derive(Debug, Deserialize)]
pub struct EnergyDetails {
    #[serde(rename = "timeUnit")]
    pub time_unit: String,
    pub unit: String,
    pub meters: Vec<Meter>,
}
#[derive(Deserialize, Debug)]
struct SiteCurrentPowerFlow {
    #[serde(rename = "updateRefreshRate")]
    update_refresh_rate: Option<u32>,
    unit: String,
    connections: Vec<Connection>,
    #[serde(rename = "GRID")]
    grid: Component,
    #[serde(rename = "LOAD")]
    load: Component,
    #[serde(rename = "PV")]
    pv: Component,
    #[serde(rename = "STORAGE")]
    storage: Option<Storage>,
}

#[derive(Deserialize, Debug)]
struct Connection {
    from: String,
    to: String,
}

#[derive(Deserialize, Debug)]
struct Component {
    status: String,
    #[serde(rename = "currentPower")]
    current_power: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct Storage {
    status: String,
    #[serde(rename = "currentPower")]
    current_power: Option<f64>,
    #[serde(rename = "chargeLevel")]
    charge_level: Option<u32>,
    critical: Option<bool>,
}

fn main() {}