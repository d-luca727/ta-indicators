use reqwest::Client;
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
    ) -> Result<ParsedDataHistory, reqwest::Error> {
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

    pub async fn get_coin_uuid(&self, coin_symbol: &str) -> Result<String, CoinUuidErr> {
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

        Err(CoinUuidErr::CoinNotFound)
    }

    pub async fn get_coin_ohlc(&self, coin_symbol: &str) -> Result<ParsedOhlcData, CoinUuidErr> /* reqwest::Error */
    {
        let url = format!("{}/coin/{}/ohlc", self.base_url, coin_symbol);

        let response = self
            .http_client
            .get(&url)
            .header("X-RapidAPI-Key", self.authorization_token.expose_secret())
            .send()
            .await?
            .error_for_status()?;

        let response_json = response.json::<OhlcResponseData>().await?;

        let parsed_ohlc: Result<Vec<ParsedOhlc>, CoinUuidErr> = response_json
            .data
            .ohlc
            .iter()
            .take(30)
            .map(|ohlc| parse_ohlc(&ohlc))
            .collect();

        let parsed_data = ParsedOhlcData { ohlc: parsed_ohlc? };

        Ok(parsed_data)
    }
}

fn parse_ohlc(ohlc: &Ohlc) -> Result<ParsedOhlc, CoinUuidErr> {
    Ok(ParsedOhlc {
        starting_at: ohlc.starting_at,
        ending_at: ohlc.ending_at,
        open: parse_float(&ohlc.open)?,
        high: parse_float(&ohlc.high)?,
        low: parse_float(&ohlc.low)?,
        close: parse_float(&ohlc.close)?,
        avg: parse_float(&ohlc.avg)?,
    })
}

fn parse_float(s: &String) -> Result<f64, reqwest::StatusCode> {
    match s.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/*** Err enums ****/

#[derive(Debug)]
pub enum CoinUuidErr {
    CoinNotFound,
    RequestError(reqwest::Error),
    StatusError(reqwest::StatusCode),
}

impl From<reqwest::Error> for CoinUuidErr {
    fn from(err: reqwest::Error) -> Self {
        CoinUuidErr::RequestError(err)
    }
}

impl From<reqwest::StatusCode> for CoinUuidErr {
    fn from(err: reqwest::StatusCode) -> Self {
        CoinUuidErr::StatusError(err)
    }
}

///this struct is necessary for serde because the values in the
///request's fields are embedded in strings rather than being bare
///floats, so it is done in a manual step immediately after parsing the request

/*************** coinUuidResponseData ****************/
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

//you never know
/* |ohlc| ParsedOhlc {
    starting_at: ohlc.starting_at,
    ending_at: ohlc.ending_at,
    open: ohlc.open.parse::<f64>().unwrap(),
    high: ohlc.high.parse::<f64>().unwrap(),
    low: ohlc.low.parse::<f64>().unwrap(),
    close: ohlc.close.parse::<f64>().unwrap(),
    avg: ohlc.avg.parse::<f64>().unwrap(),
} */
