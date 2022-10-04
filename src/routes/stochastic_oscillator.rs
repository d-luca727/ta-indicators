use actix_web::{web, HttpResponse};

use crate::crypto_client::{CoinUuidErr::*, CryptoClient};

#[derive(serde::Deserialize)]
pub struct FormData {
    coin: String,
}

pub async fn stochastic_oscillator(
    form: web::Form<FormData>,
    crypto_client: web::Data<CryptoClient>,
) -> HttpResponse {
    let uuid = match crypto_client.get_coin_uuid(&form.coin).await {
        Ok(uuid) => uuid,
        Err(CoinNotFound) => return HttpResponse::BadRequest().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let response = match crypto_client.get_coin_ohlc(&uuid).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut i: i8 = 0;
    let mut highest_high: f64 = 0.0;
    let mut lowest_low: f64 = f64::INFINITY;
    for ohlc in response.ohlc.iter() {
        let high = ohlc.high;
        let low = ohlc.low;
        if highest_high < high {
            highest_high = high;
        }

        if lowest_low > low {
            lowest_low = low;
        }

        i += 1;
        if i == 14 {
            break;
        }
    }

    let numerator = (response.ohlc.into_iter().nth(0).unwrap().close) - lowest_low;

    let denominator = highest_high - lowest_low;

    let stochastic_oscillator: f64 = (numerator / denominator) * 100.;

    HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: stochastic_oscillator,
    })
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: f64,
}
