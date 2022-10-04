use actix_web::{
    web::{self},
    HttpResponse,
};

use crate::crypto_client::{CoinUuidErr::*, CryptoClient};

pub async fn simple_moving_average(
    form: web::Form<FormData>,
    crypto_client: web::Data<CryptoClient>,
) -> HttpResponse {
    let uuid = match crypto_client.get_coin_uuid(&form.coin).await {
        Ok(uuid) => uuid,
        Err(CoinNotFound) => return HttpResponse::BadRequest().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let response = match crypto_client.get_history_prices(&uuid, &form.time).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut sum = 0.0;
    let mut n = 0.0;
    for entry in response.history {
        sum += entry.price;
        n += 1.0;
    }

    let sma = sum / n;

    HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: SimpleMovingAverageData(sma),
    })
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
