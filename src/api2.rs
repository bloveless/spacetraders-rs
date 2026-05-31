extern crate serde;

use crate::api_models::{
    Error, FactionSymbol, GetMyShipsResponse, RegisterRequest, RegisterResponse, ResponseContent,
    ResponseContentEntity,
};
use serde::de::DeserializeOwned;

#[derive(Default)]
pub struct Api {
    access_token: Option<String>,
    client: reqwest::Client,
}

impl Api {
    pub fn new() -> Api {
        return Api::default();
    }

    pub fn set_access_token(&mut self, access_token: String) {
        self.access_token = Some(access_token);
    }

    async fn handle_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<T, Error> {
        let status_code = response.status();
        let content = response.text().await?;

        if !status_code.is_client_error() && !status_code.is_server_error() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ResponseContentEntity> =
                serde_json::from_str(&content).ok();
            let local_var_error = ResponseContent {
                status: status_code,
                content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn register(
        self,
        faction: FactionSymbol,
        symbol: String,
    ) -> Result<RegisterResponse, Error> {
        let register_request = RegisterRequest {
            faction,
            symbol,
            email: None,
        };

        let response = self
            .client
            .post("https://api.spacetraders.io/v2/register")
            .json(&register_request)
            .send()
            .await?;

        Api::handle_response(response).await
    }

    pub async fn get_my_ships(self) -> Result<GetMyShipsResponse, Error> {
        let response = self
            .client
            .get("https://api.spacetraders.io/v2/my/ships")
            .bearer_auth(
                self.access_token
                    .expect("Access token was not set before attempting authorized API endpoint"),
            )
            .send()
            .await?;

        Api::handle_response(response).await
    }
}
