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
    ) -> Result<Response, Box<dyn std::error::Error>> {
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

        Ok(response)
    }

    pub async fn get_coin_uuid(
        &self,
        coin_symbol: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
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

        Ok("Bad request".to_owned())
    }

    pub async fn get_coin_ohlc(
        &self,
        coin_symbol: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/coin/{}/ohlc", self.base_url, coin_symbol);

        let response = self
            .http_client
            .get(&url)
            .header("X-RapidAPI-Key", self.authorization_token.expose_secret())
            .send()
            .await?
            .error_for_status()?;

        Ok(response)
    }
}

/*************** body parsing for get_coin_uuid ****************/
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
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

/************************************************************/
