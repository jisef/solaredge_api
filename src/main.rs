mod Meters;

use crate::Meters::meters_to_string;
use dotenvy::dotenv;
use http_adapter_reqwest::{ReqwestAdapter, reqwest};
use serde::Deserialize;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let api_key = dotenvy::var("SOLAR_EDGE_API_KEY").unwrap();
    let site_id = dotenvy::var("SOLAR_EDGE_SITE_ID").unwrap();
    let meters: Vec<crate::Meters::Meters> =
        vec![Meters::Meters::Production, Meters::Meters::Consumption];

    /*
        let avg = get_average_power(api_key.clone(), site_id.clone()).await;
        println!("{:?}", avg);
    */

    /*
            let energy = get_energy(api_key.clone(), site_id.clone(), meters).await;
            println!("{:?}", energy);
    */
    let powerflow = get_powerflow(api_key.clone(), site_id.clone()).await;
}

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

/// meters: Production, Consumption, SelfConsumption, FeedIn, Purchased
///
///
async fn get_energy(api_key: String, site_id: String, meters: Vec<crate::Meters::Meters>) {
    //todo!("does not make sense s");
    let url = format!(
        "https://monitoringapi.solaredge.com/site/{}/energyDetails?meters={}&timeUnit=DAY&startTime=2013-05-15%2011:00:00&endTime=2013-05-25%2013:00:00&api_key={}",
        site_id,
        meters_to_string(meters.clone()),
        api_key
    );
    let edr = reqwest::get(url)
        .await
        .expect("Error whilye making request")
        .json::<EnergyDetailsResponse>()
        .await
        .expect("error parsing");
    println!("{:?}", edr);
    let unit = edr.energy_details.unit;
    let mut map: HashMap<String, f64> = HashMap::new();

    for meter in edr.energy_details.meters {
        let mut sum = 0.0;
        for meter_value in meter.values {
            if meter_value.value.is_some() {
                sum += meter_value.value.unwrap()
            }
        }
    }
}

async fn get_powerflow(api_key: String, site_id: String) -> Result<SiteCurrentPowerFlow, String> {
    let url = format!(
        "https://monitoringapi.solaredge.com/site/{}/currentPowerFlow?api_key={}",
        site_id, api_key
    );

    match reqwest::get(url).await {
        Ok(x) => match x.json::<SiteCurrentPowerFlow>().await {
            Ok(supi_dupi) => {
                return Ok(supi_dupi);
            }
            Err(err) => {
                println!("{}", err.to_string());
                println!("Could not parse answer to struct");
                return Err(String::from("Could not parse answer to struct"));
            }
        },
        Err(r) => {
            println!("Request failed: {}", r.to_string());
            return Err(format!("Request failed: {}", r.to_string()));
        }
    }
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
pub struct EnergyDetailsResponse {
    #[serde(rename = "energyDetails")]
    pub energy_details: EnergyDetails,
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
    updateRefreshRate: Option<u32>,
    unit: String,
    connections: Vec<Connection>,
    GRID: Component,
    LOAD: Component,
    PV: Component,
    STORAGE: Option<Storage>,
}

#[derive(Deserialize, Debug)]
struct Connection {
    from: String,
    to: String,
}

#[derive(Deserialize, Debug)]
struct Component {
    status: String,
    currentPower: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct Storage {
    status: String,
    currentPower: Option<f64>,
    chargeLevel: Option<u32>,
    critical: Option<bool>,
}
