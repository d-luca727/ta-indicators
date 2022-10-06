use actix_web::{web, HttpResponse};

use crate::crypto_client::{CoinUuidErr::*, CryptoClient};

#[derive(serde::Deserialize)]
pub struct PathData {
    coin: String,
}

pub async fn aroon_oscillator(
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
    let mut highest_high: f64 = 0.0;
    let mut hh_counter: i8 = 0;
    let mut lowest_low: f64 = f64::INFINITY;
    let mut ll_counter: i8 = 0;
    for ohlc in response.ohlc.iter() {
        let high = ohlc.high;
        let low = ohlc.low;
        if highest_high < high {
            hh_counter = i;
            highest_high = high;
        }

        if lowest_low > low {
            ll_counter = i;
            lowest_low = low;
        }

        i += 1;
        if i == 25 {
            break;
        }
    }

    let ap_second_part: f32 = (25. - hh_counter as f32) / 25.;
    let aroon_up: f32 = 100.0 * ap_second_part;

    let ad_second_part: f32 = (25. - ll_counter as f32) / 25.;
    let aroon_down: f32 = 100.0 * ad_second_part;

    let aroon_oscillator: f32 = aroon_up - aroon_down;

    HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: aroon_oscillator,
    })
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: f32,
}
