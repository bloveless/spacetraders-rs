/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_jump_gate`]
#[derive(Clone, Debug, Default)]
pub struct GetJumpGateParams {
    /// The system symbol
    pub system_symbol: String,
    /// The waypoint symbol
    pub waypoint_symbol: String
}

/// struct for passing parameters to the method [`get_market`]
#[derive(Clone, Debug, Default)]
pub struct GetMarketParams {
    /// The system symbol
    pub system_symbol: String,
    /// The waypoint symbol
    pub waypoint_symbol: String
}

/// struct for passing parameters to the method [`get_shipyard`]
#[derive(Clone, Debug, Default)]
pub struct GetShipyardParams {
    /// The system symbol
    pub system_symbol: String,
    /// The waypoint symbol
    pub waypoint_symbol: String
}

/// struct for passing parameters to the method [`get_system`]
#[derive(Clone, Debug, Default)]
pub struct GetSystemParams {
    /// The system symbol
    pub system_symbol: String
}

/// struct for passing parameters to the method [`get_system_waypoints`]
#[derive(Clone, Debug, Default)]
pub struct GetSystemWaypointsParams {
    /// The system symbol
    pub system_symbol: String,
    /// What entry offset to request
    pub page: Option<i32>,
    /// How many entries to return per page
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`get_systems`]
#[derive(Clone, Debug, Default)]
pub struct GetSystemsParams {
    /// What entry offset to request
    pub page: Option<i32>,
    /// How many entries to return per page
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`get_waypoint`]
#[derive(Clone, Debug, Default)]
pub struct GetWaypointParams {
    /// The system symbol
    pub system_symbol: String,
    /// The waypoint symbol
    pub waypoint_symbol: String
}


/// struct for typed errors of method [`get_jump_gate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJumpGateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_market`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMarketError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_shipyard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetShipyardError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_system`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSystemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_system_waypoints`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSystemWaypointsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_systems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSystemsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_waypoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWaypointError {
    UnknownValue(serde_json::Value),
}


/// Get jump gate details for a waypoint.
pub async fn get_jump_gate(configuration: &configuration::Configuration, params: GetJumpGateParams) -> Result<crate::models::GetJumpGate200Response, Error<GetJumpGateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let system_symbol = params.system_symbol;
    let waypoint_symbol = params.waypoint_symbol;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/systems/{systemSymbol}/waypoints/{waypointSymbol}/jump-gate", local_var_configuration.base_path, systemSymbol=crate::apis::urlencode(system_symbol), waypointSymbol=crate::apis::urlencode(waypoint_symbol));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetJumpGateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve imports, exports and exchange data from a marketplace. Imports can be sold, exports can be purchased, and exchange goods can be purchased or sold. Send a ship to the waypoint to access trade good prices and recent transactions.
pub async fn get_market(configuration: &configuration::Configuration, params: GetMarketParams) -> Result<crate::models::GetMarket200Response, Error<GetMarketError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let system_symbol = params.system_symbol;
    let waypoint_symbol = params.waypoint_symbol;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/systems/{systemSymbol}/waypoints/{waypointSymbol}/market", local_var_configuration.base_path, systemSymbol=crate::apis::urlencode(system_symbol), waypointSymbol=crate::apis::urlencode(waypoint_symbol));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMarketError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the shipyard for a waypoint.
pub async fn get_shipyard(configuration: &configuration::Configuration, params: GetShipyardParams) -> Result<crate::models::GetShipyard200Response, Error<GetShipyardError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let system_symbol = params.system_symbol;
    let waypoint_symbol = params.waypoint_symbol;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/systems/{systemSymbol}/waypoints/{waypointSymbol}/shipyard", local_var_configuration.base_path, systemSymbol=crate::apis::urlencode(system_symbol), waypointSymbol=crate::apis::urlencode(waypoint_symbol));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetShipyardError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the details of a system.
pub async fn get_system(configuration: &configuration::Configuration, params: GetSystemParams) -> Result<crate::models::GetSystem200Response, Error<GetSystemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let system_symbol = params.system_symbol;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/systems/{systemSymbol}", local_var_configuration.base_path, systemSymbol=crate::apis::urlencode(system_symbol));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSystemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch all of the waypoints for a given system. System must be charted or a ship must be present to return waypoint details.
pub async fn get_system_waypoints(configuration: &configuration::Configuration, params: GetSystemWaypointsParams) -> Result<crate::models::GetSystemWaypoints200Response, Error<GetSystemWaypointsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let system_symbol = params.system_symbol;
    let page = params.page;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/systems/{systemSymbol}/waypoints", local_var_configuration.base_path, systemSymbol=crate::apis::urlencode(system_symbol));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSystemWaypointsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return a list of all systems.
pub async fn get_systems(configuration: &configuration::Configuration, params: GetSystemsParams) -> Result<crate::models::GetSystems200Response, Error<GetSystemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/systems", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSystemsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// View the details of a waypoint.
pub async fn get_waypoint(configuration: &configuration::Configuration, params: GetWaypointParams) -> Result<crate::models::GetWaypoint200Response, Error<GetWaypointError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let system_symbol = params.system_symbol;
    let waypoint_symbol = params.waypoint_symbol;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/systems/{systemSymbol}/waypoints/{waypointSymbol}", local_var_configuration.base_path, systemSymbol=crate::apis::urlencode(system_symbol), waypointSymbol=crate::apis::urlencode(waypoint_symbol));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWaypointError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

