use actix_web::{web, HttpResponse};

use crate::crypto_client::CryptoClient;

#[derive(serde::Deserialize)]
pub struct FormData {
    coin: String,
}

pub async fn exponential_moving_average(
    form: web::Form<FormData>,
    crypto_client: web::Data<CryptoClient>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let uuid = crypto_client.get_coin_uuid(&form.coin).await?;

    if uuid.as_str().eq("Bad request") {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let response = crypto_client.get_coin_ohlc(&uuid).await?;

    let ema = response.ohlc.iter().take(20).fold(0., |acc: f64, x| {
        x.close * (2. / (1. + 20.)) + acc * (1. - (2. / (1. + 20.)))
    });

    Ok(HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: ema,
    }))
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: f64,
}
