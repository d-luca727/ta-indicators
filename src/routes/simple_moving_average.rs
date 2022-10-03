use actix_web::{
    web::{self},
    HttpResponse,
};

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::crypto_client::CryptoClient;

pub async fn simple_moving_average(
    form: web::Form<FormData>,
    crypto_client: web::Data<CryptoClient>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let uuid = crypto_client.get_coin_uuid(&form.coin).await?;

    if uuid.as_str().eq("Bad request") {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let response = crypto_client
        .get_history_prices(&uuid, &form.time)
        .await?
        .json::<HistoryResponseData>()
        .await?;

    let mut sum = 0.0;
    let mut n = 0.0;
    for entry in response.data.history {
        sum += entry.price.parse::<f64>().unwrap();
        n += 1.0;
    }

    let sma = sum / n;

    Ok(HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: SimpleMovingAverageData(sma),
    }))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryResponseData {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub change: String,
    pub history: Vec<History>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub price: String,
    pub timestamp: i64,
}

#[derive(serde::Deserialize)]
pub struct FormData {
    coin: String,
    time: String,
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: SimpleMovingAverageData,
}
#[derive(serde::Serialize)]
struct SimpleMovingAverageData(f64);
