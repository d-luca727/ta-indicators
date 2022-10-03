use actix_web::{web, HttpResponse};

use crate::crypto_client::CryptoClient;

use super::OhlcResponseData;

#[derive(serde::Deserialize)]
pub struct FormData {
    coin: String,
}

pub async fn stochastic_oscillator(
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
    let mut lowest_low: f64 = f64::INFINITY;
    for ohlc in response.data.ohlc.iter() {
        let high = ohlc.high.parse::<f64>().unwrap();
        let low = ohlc.low.parse::<f64>().unwrap();
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

    let numerator = response
        .data
        .ohlc
        .into_iter()
        .nth(0)
        .unwrap()
        .close
        .parse::<f64>()
        .unwrap()
        - lowest_low;

    let denominator = highest_high - lowest_low;

    let stochastic_oscillator: f64 = (numerator / denominator) * 100.;

    Ok(HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: stochastic_oscillator,
    }))
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: f64,
}
