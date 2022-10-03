use actix_web::{web, HttpResponse};

use crate::crypto_client::CryptoClient;

use super::OhlcResponseData;

#[derive(serde::Deserialize)]
pub struct FormData {
    coin: String,
}

pub async fn aroon_oscillator(
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

    let mut i: i8 = 0;
    let mut highest_high: f64 = 0.0;
    let mut hh_counter: i8 = 0;
    let mut lowest_low: f64 = f64::INFINITY;
    let mut ll_counter: i8 = 0;
    for ohlc in response.data.ohlc.iter() {
        let high = ohlc.high.parse::<f64>().unwrap();
        let low = ohlc.low.parse::<f64>().unwrap();
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

    Ok(HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: aroon_oscillator,
    }))
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: f32,
}
