use reqwest::{Client, Response};
use secrecy::{ExposeSecret, Secret};
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub struct CryptoClient {
    http_client: Client,
    base_url: String,
    authorization_token: Secret<String>,
}

impl CryptoClient {
    pub fn new(base_url: String, authorization_token: Secret<String>) -> Self {
        let http_client = Client::new();

        Self {
            http_client,
            base_url,
            authorization_token,
        }
    }

    pub async fn get_history_prices(
        &self,
        coin_uuid: &str,
        time: &str,
    ) -> Result<ParsedDataHistory, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/coin/{}/history?timePeriod={}",
            self.base_url, coin_uuid, time
        );

        let response = self
            .http_client
            .get(&url)
            .header("X-RapidAPI-Key", self.authorization_token.expose_secret())
            .send()
            .await?
            .error_for_status()?;

        let parsed_response = response.json::<HistoryResponseData>().await?;

        let parsed_history: Vec<ParsedHistory> = parsed_response
            .data
            .history
            .iter()
            .map(|history| ParsedHistory {
                price: history.price.parse::<f64>().unwrap(),
                timestamp: history.timestamp,
            })
            .collect();

        let parsed_data = ParsedDataHistory {
            history: parsed_history,
        };
        Ok(parsed_data)
    }

    pub async fn get_coin_uuid(&self, coin_symbol: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/search-suggestions?query={}", self.base_url, coin_symbol);

        let response = self
            .http_client
            .get(&url)
            .header("X-RapidAPI-Key", self.authorization_token.expose_secret())
            .send()
            .await?
            .error_for_status()?;

        let body = response.json::<Body>().await?;

        for entry in body.data.coins {
            if entry.symbol.as_str().eq(&coin_symbol.to_ascii_uppercase()) {
                return Ok(entry.uuid);
            }
        }

        Ok("coin not found".to_owned())
    }

    pub async fn get_coin_ohlc(&self, coin_symbol: &str) -> Result<ParsedOhlcData, reqwest::Error> {
        let url = format!("{}/coin/{}/ohlc", self.base_url, coin_symbol);

        let response = self
            .http_client
            .get(&url)
            .header("X-RapidAPI-Key", self.authorization_token.expose_secret())
            .send()
            .await?
            .error_for_status()?;

        let response_json = response.json::<OhlcResponseData>().await?;
        //todo: CHECK STATUS

        let parsed_ohlc: Vec<ParsedOhlc> = response_json
            .data
            .ohlc
            .iter()
            .take(30)
            .map(|ohlc| ParsedOhlc {
                starting_at: ohlc.starting_at,
                ending_at: ohlc.ending_at,
                open: ohlc.open.parse::<f64>().unwrap(),
                high: ohlc.high.parse::<f64>().unwrap(),
                low: ohlc.low.parse::<f64>().unwrap(),
                close: ohlc.close.parse::<f64>().unwrap(),
                avg: ohlc.avg.parse::<f64>().unwrap(),
            })
            .collect();

        let parsed_data = ParsedOhlcData { ohlc: parsed_ohlc };

        Ok(parsed_data)
    }
}

/*************** body parsing for get_coin_uuid ****************/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub status: String,
    pub data: DataUuid,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataUuid {
    pub coins: Vec<Coin>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub uuid: String,
    pub icon_url: String,
    pub name: String,
    pub symbol: String,
    pub price: String,
}

/*************************** historyResponseData ******************************/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryResponseData {
    pub status: String,
    pub data: DataHistory,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataHistory {
    pub change: String,
    pub history: Vec<History>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub price: String,
    pub timestamp: i64,
}
/********* PARSED DATA ******/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedDataHistory {
    //pub change: String,
    pub history: Vec<ParsedHistory>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedHistory {
    pub price: f64,
    pub timestamp: i64,
}

/******************** ohlc RESPONSE DATA *****************/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OhlcResponseData {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub ohlc: Vec<Ohlc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ohlc {
    pub starting_at: i64,
    pub ending_at: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub avg: String,
}
/********* PARSED DATA ******/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedOhlcData {
    pub ohlc: Vec<ParsedOhlc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedOhlc {
    pub starting_at: i64,
    pub ending_at: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub avg: f64,
}
