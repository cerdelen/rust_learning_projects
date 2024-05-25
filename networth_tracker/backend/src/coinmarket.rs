use crate::error::Error;
// use crate::DB;
// use crate::models::User;
// use actix_web::cookie::time::format_description::parse;
// use actix_web::web::Json;
// use actix_web::web::Query;
use actix_web::web::Path;
use actix_web::get;
use reqwest::Client;
use dotenv::dotenv;
use std::env;


use serde_json::Value;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct Coin {
    id: i64,
    rank: i64,
    name: String,
    symbol: String,
    slug: String,
    is_active: Value,
    first_historical_data: Value,
    last_historical_data: Value,
    platform: Value
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Data {
    Coins(Vec<Coin>),
    Prices(Vec<Value>),
}

impl Data {
    fn len(&self) -> usize {
        match self {
            Data::Coins(coins) => {coins.len()}
            Data::Prices(prices) => {prices.len()}
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct CoinMarketCapResponse {
    data: Data,
    status: Value,
}




#[get("/price_quote/{id}")]
async fn price_quote(id: Path<String>) -> Result<String, Error> {
    dotenv().ok();
    let client = Client::new();
    let api_key = env::var("coinmarketcap_api_key").expect("coinmarketcap_api_key must be set");
    println!("trying to get price_quote for id {:?}", id.as_ref());

    let response = match client
        .get("https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest")
        .query(&[("id", id.into_inner())])
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                resp.text().await.unwrap_or_else(|_| "Failed to read response text".to_string())
            } else {
                format!("Request failed with status: {}", resp.status())
            }
        }
        Err(err) => format!("Request error: {}", err),
    };
    match serde_json::from_str::<CoinMarketCapResponse>(&response) {
        Ok(parsed_response) => {
            println!("Success {:?}\nData_size: {:?}", parsed_response.status, parsed_response.data.len());
            if let Data::Prices(_prices) = &parsed_response.data {
                println!("IT WOOOORJKS!!!!!");
            }
            else {
                println!("Wrong Json got returned!");
            }
            // match parsed_response.data {
            //     Data::Coins(coin) => {
            //         println!("Coin: {:?}", coin);
            //     }
            //     _ => {
            //         println!("Wrong Type Returned ... did not receive Coins");
            //     }
            // };
        }
        Err(err) => {println!("Err: {}", err)}
    }

    Ok(response)
}

#[get("/currency_map")]
async fn coin_map() -> Result<String, Error> {
    dotenv().ok();
    let client = Client::new();
    let api_key = env::var("coinmarketcap_api_key").expect("coinmarketcap_api_key must be set");
    let limit: String = "10".to_string();

    let response = match client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/map")
        .query(&[("limit", limit)])
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                resp.text().await.unwrap_or_else(|_| "Failed to read response text".to_string())
            } else {
                format!("Request failed with status: {}", resp.status())
            }
        }
        Err(err) => format!("Request error: {}", err),
    };

    match serde_json::from_str::<CoinMarketCapResponse>(&response) {
        Ok(parsed_response) => {
            println!("Success {:?}\nData_size: {:?}", parsed_response.status, parsed_response.data.len());
            if let Data::Coins(_coin) = &parsed_response.data {
                println!("IT WOOOORJKS!!!!!");
            }
            else {
                println!("Wrong Json got returned!");
            }
            // match parsed_response.data {
            //     Data::Coins(coin) => {
            //         println!("Coin: {:?}", coin);
            //     }
            //     _ => {
            //         println!("Wrong Type Returned ... did not receive Coins");
            //     }
            // };
        }
        Err(err) => {println!("Err: {}", err)}
    }
    Ok(response)
}
