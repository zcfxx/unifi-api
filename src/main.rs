mod curl;
mod response;

use anyhow::Result;
use reqwest::{header::HeaderMap, Body, RequestBuilder};
use response::ResponseGetIspMetric;
use std::{collections::HashMap, env::var};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();

    // let url = var("URL_LIST_HOSTS").unwrap();
    // let url = var("URL_Q_METRICS").unwrap();
    // let url = var("URL_BY_ID").unwrap();
    // let url = var("URL_LIST_SITES").unwrap();
    let url = var("URL_G_METRICS").unwrap();
    log::info!("url: {}", url);

    let api_key = var("API").unwrap();
    log::info!("api: {}", api_key);

    let host_id = var("HOST_ID").unwrap();
    let site_id = var("SITE_ID").unwrap();

    // let mut sites = HashMap::new();
    // sites.insert("hostId", host_id);
    // sites.insert("siteId", site_id);

    // let mut body_map = HashMap::new();
    // body_map.insert("sites", sites);
    //
    let mut body_map = curl::Body::new();
    body_map.with_host(host_id).with_site(site_id);
    let body_str = serde_json::to_string::<curl::Body>(&body_map);
    println!("{:?}", body_str);

    let body = Body::wrap(body_str.unwrap());

    // let data = r#"
    //     {
    //       "sites": [
    //         {
    //           "hostId": "9C05D65983C90000000007F4456E00000000085FAB500000000065E5DF43:913329714",
    //           "siteId": "660ca3ada0dda36af5c219c9"
    //         }
    //       ]
    //     }
    //     "#;

    // let body = Body::wrap(data.to_string());

    let mut headers = HeaderMap::new();

    // headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "applicatioin/json".parse().unwrap());
    headers.insert("X-API-KEY", api_key.parse().unwrap());

    let client = reqwest::Client::new();

    let request = client.get(url).headers(headers).body(body);
    let response = request.send().await?.text().await?;

    // println!("{:?}", response);
    let value: ResponseGetIspMetric = serde_json::from_str(response.as_str()).unwrap();
    // println!("{:?}", value);

    println!("http_status: {:?}", value.get_http_status_code());

    // println!("ave_latency: {:?}", value.get_avg_latency());

    for ave in value.get_avg_latency() {
        println!("{}\t\t{}", ave.0, ave.1);
    }

    println!("count: {}", value.get_avg_latency().len());
    Ok(())
}
