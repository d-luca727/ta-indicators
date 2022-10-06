use actix_web::web;
use actix_web::HttpResponse;

use crate::crypto_client::CoinUuidErr::*;
use crate::crypto_client::CryptoClient;

#[derive(serde::Deserialize)]
pub struct PathData {
    coin: String,
    market: String,
}

pub async fn fibonacci_retracement(
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
    //TODO remove unwrap
    let ohlc = response.ohlc.into_iter().nth(0).unwrap();

    let high = ohlc.high;
    let low = ohlc.low;

    //vec of %s
    let percentages: Vec<f64> = vec![0.0, 0.236, 0.382, 0.5, 0.618, 0.764, 1.0, 1.382];

    let mut vec: Vec<Percentage> = vec![];

    for percentage in percentages {
        let second_part = (high - low) * percentage;
        let first_part = match path.market.to_ascii_uppercase().as_str() {
            "U" | "UPTREND" => high - second_part,
            "D" | "DOWNTREND" => low + second_part,
            &_ => {
                return HttpResponse::Ok().json(Success {
                    status: "400 | Bad Request, try inserting the correct market data".to_owned(),
                    data: vec,
                })
            }
        };

        vec.push(Percentage {
            percentage: format!("{}%", (percentage * 100.0).to_string()),
            value: first_part - second_part,
        });
    }

    HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: vec,
    })
}

pub async fn fibonacci_extension(
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

    //TODO remove unwrap
    let ohlc = response.ohlc.into_iter().nth(0).unwrap();

    let high = ohlc.high;
    let low = ohlc.low;

    //vec of %s
    let percentages: Vec<f64> = vec![0.0, 0.236, 0.382, 0.5, 0.618, 0.764, 1.0, 1.382];

    let mut vec: Vec<Percentage> = vec![];

    for percentage in percentages {
        let second_part = (high - low) * percentage;
        let first_part = match path.market.to_ascii_uppercase().as_str() {
            "U" | "UPTREND" => high + second_part,
            "D" | "DOWNTREND" => low - second_part,
            &_ => {
                return HttpResponse::Ok().json(Success {
                    status: "400 | Bad Request, try inserting the correct market trend data"
                        .to_owned(),
                    data: vec,
                })
            }
        };

        vec.push(Percentage {
            percentage: format!("{}%", (percentage * 100.0).to_string()),
            value: first_part - second_part,
        });
    }

    HttpResponse::Ok().json(Success {
        status: "success".to_owned(),
        data: vec,
    })
}

//success Response
#[derive(serde::Serialize)]
struct Success {
    status: String,
    data: Vec<Percentage>,
}
#[derive(serde::Serialize)]
struct Percentage {
    percentage: String,
    value: f64,
}
