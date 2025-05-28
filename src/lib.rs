use std::ops::Sub;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use crate::energy::{EnergyDetailsResponse, TimeUnit};
use crate::meters::{meters_to_string, Meter, Meters};
use crate::power::PowerDetailsWrapper;

mod test;
pub mod energy;
pub mod meters;
pub mod power;
#[tokio::test]
async fn main() {
    let api_key = dotenvy::var("SOLAR_EDGE_API_KEY").unwrap();
    let site_id = dotenvy::var("SOLAR_EDGE_SITE_ID").unwrap();
    let meters: Vec<Meters> =
        vec![Meters::Production, Meters::Consumption];

    let date = NaiveDate::from_ymd_opt(2025, 5, 1).unwrap();
    let time = NaiveTime::from_hms_opt(11, 0, 0).unwrap();
    let naive_datetime = NaiveDateTime::new(date, time);
    let from: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);


    get_power_details_in_range(api_key.clone(), site_id.clone(), meters.clone(), from, Utc::now()).await.unwrap();
    get_power_details(api_key.clone(), site_id.clone(), meters.clone()).await.unwrap();

    let energy = get_site_energy(api_key.clone(), site_id.clone(), meters.clone(), TimeUnit::HOUR, from, Utc::now()).await.unwrap();

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

/// The Api only allows data in range of one month
pub async fn get_power_details_in_range(api_key: String, site_id: String, meters: Vec<crate::meters::Meters>, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<PowerDetailsWrapper, Error> {
    let from = from.format("%Y-%m-%d%%20%H:%M:%S").to_string();
    let to = to.format("%Y-%m-%d%%20%H:%M:%S").to_string();
    let url = format!(
        "https://monitoringapi.solaredge.com/site/{}/powerDetails?meters={}&startTime={}&endTime={}&api_key={}",
        site_id, meters_to_string(meters), from, to, api_key
    );
    fetch_and_parse::<PowerDetailsWrapper>(url).await
}

/// Gets Power Details from now to 28days before
pub async fn get_power_details(api_key: String, site_id: String, meters: Vec<crate::meters::Meters>) -> Result<PowerDetailsWrapper, Error> {
    let to: DateTime<Utc> = Utc::now();
    let from: DateTime<Utc> = to.sub(Duration::days(28));

    get_power_details_in_range(api_key, site_id, meters,from,to).await
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
