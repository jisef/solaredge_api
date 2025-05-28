use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use crate::energy::TimeUnit;
use crate::meters::{Meter, Meters};

mod test;
pub mod energy;
pub mod meters;
pub mod power;
#[tokio::test]
async fn main() {
    let api_key = dotenvy::var("SOLAR_EDGE_API_KEY").unwrap();
    let site_id = dotenvy::var("SOLAR_EDGE_SITE_ID").unwrap();
    let meters: Vec<meters::Meters> =
        vec![meters::Meters::Production, meters::Meters::Consumption];

    let date = NaiveDate::from_ymd_opt(2025, 5, 1).unwrap();
    let time = NaiveTime::from_hms_opt(11, 0, 0).unwrap();
    let naive_datetime = NaiveDateTime::new(date, time);
    let from: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);


    power::get_power_details_in_range(api_key.clone(), site_id.clone(), meters.clone(), from, Utc::now()).await.unwrap();
    power::get_power_details(api_key.clone(), site_id.clone(), meters.clone()).await.unwrap();

    let energy = energy::get_site_energy(api_key.clone(), site_id.clone(), meters.clone(), TimeUnit::HOUR, from, Utc::now()).await.unwrap();

    println!("{}", energy.get_unit());
    println!("{}", energy.get_average_for_meter(Meters::Production));
}
    

async fn fetch_and_parse<T>(url: String) -> Result<T, Error> where T: DeserializeOwned {
    let response = reqwest::get(url.clone())
        .await?;

    match response.json::<T>().await {
        Ok(data) => {Ok(data)}
        Err(err) => {Err(err)}
    }
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
