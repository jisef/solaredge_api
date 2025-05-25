use std::ops::Sub;
use crate::reqwest::Error;
use chrono::{DateTime, Duration, Utc};
use crate::meters::meters_to_string;
use crate::{fetch_and_parse, PowerDetailsWrapper};

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