use actix_web::{web, HttpResponse};

use crate::crypto_client::{CoinUuidErr::*, CryptoClient};

#[derive(serde::Deserialize)]
pub struct PathData {
    coin: String,
}

pub async fn rsi(
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

    let mut i: i8 = 0;
    let mut prev_close: f64 = 0.0;
    let mut sum_of_gains = 0.0;
    let mut sum_of_losses = 0.0;
    for ohlc in response.ohlc.iter() {
        if i != 0 {
            let x = prev_close - ohlc.close;
            match x {
                res if res > 0.0 => sum_of_gains += x,
                res if res < 0.0 => sum_of_losses += x.abs(),
                _ => {}
            };
        }

        i += 1;
        prev_close = ohlc.close;
        if i == 14 {
            break;
        }
    }

    let average_gain = sum_of_gains / 14.0;
    let average_losses = sum_of_losses / 14.0;

    let rs = average_gain / average_losses;

    let second_part = 100.0 / (1.0 + rs);
    let rsi = 100.0 - second_part;

    HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: rsi,
    })
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: f64,
}
