use actix_web::{web, HttpResponse};

use crate::crypto_client::{CoinUuidErr::*, CryptoClient};

#[derive(serde::Deserialize)]
pub struct PathData {
    coin: String,
}

pub async fn exponential_moving_average(
    path: web::Path<PathData>,
    crypto_client: web::Data<CryptoClient>,
) -> HttpResponse {
    let uuid = match crypto_client.get_coin_uuid(&path.coin).await {
        Ok(uuid) => uuid,
        Err(CoinNotFound) => return HttpResponse::BadRequest().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let response = match crypto_client.get_coin_ohlc(&uuid).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let ema = response.ohlc.iter().take(20).fold(0., |acc: f64, x| {
        x.close * (2. / (1. + 20.)) + acc * (1. - (2. / (1. + 20.)))
    });

    HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: ema,
    })
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: f64,
}
