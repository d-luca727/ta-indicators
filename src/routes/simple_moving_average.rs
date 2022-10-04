use actix_web::{
    web::{self},
    HttpResponse,
};

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::crypto_client::{CryptoClient, ParsedDataHistory};

pub async fn simple_moving_average(
    form: web::Form<FormData>,
    crypto_client: web::Data<CryptoClient>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let uuid = crypto_client.get_coin_uuid(&form.coin).await?;

    if uuid.as_str().eq("Bad request") {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let response = crypto_client.get_history_prices(&uuid, &form.time).await?;

    let mut sum = 0.0;
    let mut n = 0.0;
    for entry in response.history {
        sum += entry.price;
        n += 1.0;
    }

    let sma = sum / n;

    Ok(HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: SimpleMovingAverageData(sma),
    }))
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
