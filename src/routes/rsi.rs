use actix_web::{web, HttpResponse};

use crate::crypto_client::CryptoClient;

use super::OhlcResponseData;

#[derive(serde::Deserialize)]
pub struct FormData {
    coin: String,
}

pub async fn rsi(
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
    let mut prev_close: f64 = 0.0;
    let mut sum_of_gains = 0.0;
    let mut sum_of_losses = 0.0;
    for ohlc in response.data.ohlc.iter() {
        if i != 0 {
            let x = prev_close - ohlc.close.parse::<f64>().unwrap();
            match x {
                res if res > 0.0 => sum_of_gains += x,
                res if res < 0.0 => sum_of_losses += x.abs(),
                _ => {}
            };
        }

        i += 1;
        prev_close = ohlc.close.parse::<f64>().unwrap();
        if i == 14 {
            break;
        }
    }

    let average_gain = sum_of_gains / 14.0;
    let average_losses = sum_of_losses / 14.0;

    let rs = average_gain / average_losses;

    let second_part = 100.0 / (1.0 + rs);
    let rsi = 100.0 - second_part;

    Ok(HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: rsi,
    }))
}

#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: f64,
}
