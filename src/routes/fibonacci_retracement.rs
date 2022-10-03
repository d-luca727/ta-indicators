use actix_web::web;
use actix_web::HttpResponse;

use crate::crypto_client::CryptoClient;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(serde::Deserialize)]
pub struct FormData {
    coin: String,
    market: String,
}

pub async fn fibonacci_retracement(
    form: web::Form<FormData>,
    crypto_client: web::Data<CryptoClient>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let uuid = crypto_client.get_coin_uuid(&form.coin).await?;

    if uuid.as_str().eq("Bad request") {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let response = crypto_client
        .get_coin_ohlc(&uuid)
        .await?
        .json::<OhlcResponseData>()
        .await?;

    //TODO remove unwrap
    let ohlc = response.data.ohlc.into_iter().nth(0).unwrap();

    let high = ohlc.high.parse::<f64>().unwrap();
    let low = ohlc.low.parse::<f64>().unwrap();

    //vec of %s
    let percentages: Vec<f64> = vec![0.0, 0.236, 0.382, 0.5, 0.618, 0.764, 1.0, 1.382];

    let mut vec: Vec<Percentage> = vec![];

    for percentage in percentages {
        let second_part = (high - low) * percentage;
        let first_part = match form.market.to_ascii_uppercase().as_str() {
            "U" | "UPTREND" => high - second_part,
            "D" | "DOWNTREND" => low + second_part,
            &_ => {
                return Ok(HttpResponse::Ok().json(Success {
                    status: "400 | Bad Request, try inserting the correct market data".to_owned(),
                    data: vec,
                }))
            }
        };

        vec.push(Percentage {
            percentage: format!("{}%", (percentage * 100.0).to_string()),
            value: first_part - second_part,
        });
    }

    Ok(HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: vec,
    }))
}

pub async fn fibonacci_extension(
    form: web::Form<FormData>,
    crypto_client: web::Data<CryptoClient>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let uuid = crypto_client.get_coin_uuid(&form.coin).await?;

    if uuid.as_str().eq("Bad request") {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let response = crypto_client
        .get_coin_ohlc(&uuid)
        .await?
        .json::<OhlcResponseData>()
        .await?;

    //TODO remove unwrap
    let ohlc = response.data.ohlc.into_iter().nth(0).unwrap();

    let high = ohlc.high.parse::<f64>().unwrap();
    let low = ohlc.low.parse::<f64>().unwrap();

    //vec of %s
    let percentages: Vec<f64> = vec![0.0, 0.236, 0.382, 0.5, 0.618, 0.764, 1.0, 1.382];

    let mut vec: Vec<Percentage> = vec![];

    for percentage in percentages {
        let second_part = (high - low) * percentage;
        let first_part = match form.market.to_ascii_uppercase().as_str() {
            "U" | "UPTREND" => high + second_part,
            "D" | "DOWNTREND" => low - second_part,
            &_ => {
                return Ok(HttpResponse::Ok().json(Success {
                    status: "400 | Bad Request, try inserting the correct market trend data"
                        .to_owned(),
                    data: vec,
                }))
            }
        };

        vec.push(Percentage {
            percentage: format!("{}%", (percentage * 100.0).to_string()),
            value: first_part - second_part,
        });
    }

    Ok(HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: vec,
    }))
}

//resposne data
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OhlcResponseData {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub ohlc: Vec<Ohlc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ohlc {
    pub starting_at: i64,
    pub ending_at: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub avg: String,
}

//success Response
#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: Vec<Percentage>,
}
#[derive(serde::Serialize)]
struct Percentage {
    percentage: String,
    value: f64,
}
