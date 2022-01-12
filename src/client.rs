//! The client module wraps the interactions between the client and the server
use crate::shared;
use crate::{requests, responses};

use crate::errors::SpaceTradersClientError;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Method, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;

/// HttpClient is a thread-safe rate-limited space traders client
pub type HttpClient = Arc<Mutex<SpaceTradersClient>>;

/// Allow the user to tie into the request lifecycle and do things with the request, responses, and/or error coming back
pub type PostRequestHook = fn(
    method: &str,
    url: &str,
    request_body: Option<&str>,
    response_status_code: Option<u16>,
    response_headers: Option<&HashMap<String, String>>,
    response_body: Option<&str>,
    error: Option<&SpaceTradersClientError>,
);

/// SpaceTradersClient wraps the actual reqwest client and adds rate-limiting support
#[derive(Clone)]
pub struct SpaceTradersClient {
    client: reqwest::Client,
    post_request_hook: Option<PostRequestHook>,
}

impl Debug for SpaceTradersClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpaceTradersClient")
            .field("client", &self.client)
            .finish()
    }
}

/// SpaceTradersClientRequest wraps all the parameters sent to the spacetraders client
#[derive(Serialize)]
pub struct SpaceTradersClientRequest {
    method: String,
    url: String,
    request_headers: HashMap<String, String>,
    request_text: String,
}

/// SpaceTradersClientRequest wraps all the parameters received from the spacetraders API
#[derive(Serialize)]
pub struct SpaceTradersClientResponse {
    status_code: u16,
    response_headers: HashMap<String, String>,
    response_text: String,
}

impl SpaceTradersClient {
    fn new(proxy: Option<String>) -> Self {
        let mut client_builder = reqwest::ClientBuilder::new();

        if let Some(proxy) = proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy).unwrap());
        }

        Self {
            client: client_builder.build().unwrap(),
            post_request_hook: None,
        }
    }

    fn set_post_request_hook(&mut self, hook: PostRequestHook) {
        self.post_request_hook = Some(hook);
    }

    async fn execute_request(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
        token: Option<&str>,
    ) -> Result<SpaceTradersClientResponse, SpaceTradersClientError> {
        let mut request_builder = self
            .client
            .request(Method::from_str(&method).unwrap(), Url::parse(url).unwrap());

        if let Some(token) = token {
            request_builder = request_builder.header(
                HeaderName::from_lowercase(b"authorization").unwrap(),
                HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap(),
            );
        }

        if let Some(body) = body {
            request_builder = request_builder.header(
                HeaderName::from_lowercase(b"content-type").unwrap(),
                HeaderValue::from_static("application/json"),
            );
            request_builder = request_builder.body(body.to_owned());
        }

        let mut attempts = 0;
        let request = request_builder.build().unwrap();
        loop {
            attempts += 1;
            if attempts > 3 {
                return Err(SpaceTradersClientError::TooManyRetries);
            }

            match self.client.execute(request.try_clone().unwrap()).await {
                Ok(response) => {
                    let response_headers =
                        response
                            .headers()
                            .iter()
                            .fold(HashMap::new(), |mut acc, (h, v)| {
                                acc.insert(h.to_string(), v.to_str().unwrap().to_string());
                                acc
                            });
                    let response_status = response.status();
                    let response_text = response.text().await?;

                    if let Some(post_request_hook) = self.post_request_hook {
                        post_request_hook(
                            method,
                            url,
                            body,
                            Some(response_status.as_u16()),
                            Some(&response_headers),
                            Some(&response_text),
                            None,
                        );
                    }

                    // Check if the response was a throttle exception (status 429 means we have been rate limited)
                    if response_status == 429 {
                        let retry_after: f64 = response_headers
                            .get("retry-after")
                            .unwrap_or(&"1.0".to_string())
                            .parse()
                            .unwrap_or(1.0);

                        // If it was a throttle then wait based on the retry-after response headers
                        log::warn!("Rate limited... waiting for {} seconds before trying again. Request: \"{} {}\"", retry_after, request.method(), request.url());
                        tokio::time::sleep(Duration::from_secs_f64(retry_after)).await;

                        continue;
                    } else if response_status == 401 {
                        return Err(SpaceTradersClientError::Unauthorized);
                    } else if response_status == 500 {
                        // If there was an internal server error then try the request again in 2 seconds
                        log::error!(
                            "Caught internal server error retrying in 2 seconds. {}",
                            response_text
                        );
                        tokio::time::sleep(Duration::from_secs(2)).await;

                        continue;
                    } else {
                        return Ok(SpaceTradersClientResponse {
                            status_code: response_status.as_u16(),
                            response_headers,
                            response_text,
                        });
                    }
                }
                Err(e) => {
                    let space_traders_client_error = SpaceTradersClientError::Http(e);

                    if let Some(post_request_hook) = self.post_request_hook {
                        post_request_hook(
                            method,
                            url,
                            body,
                            None,
                            None,
                            None,
                            Some(&space_traders_client_error),
                        );
                    }

                    return Err(space_traders_client_error);
                }
            }
        }
    }
}

/// Get a rate-limited http client that is safe to use across threads and won't break rate-limiting
pub fn get_http_client(proxy: Option<String>) -> HttpClient {
    Arc::new(Mutex::new(SpaceTradersClient::new(proxy)))
}

/// Get a rate-limited http client, with post receive hook, that is safe to use across threads and
/// won't break rate-limiting
pub fn get_http_client_with_hook(proxy: Option<String>, hook: PostRequestHook) -> HttpClient {
    let mut client = SpaceTradersClient::new(proxy);
    client.set_post_request_hook(hook);

    Arc::new(Mutex::new(client))
}

/// Parse a response string into the type represented by T
/// If the `response_text` cannot be parsed into type T then it is assumed that an error
/// occurred and an shared::ErrorMessage will be returned
///
/// # Arguments
///
/// * `response_text` - A string containing the JSON response to be parsed
fn parse_response<'a, T: Deserialize<'a>>(
    response_text: &'a str,
) -> Result<T, SpaceTradersClientError> {
    match serde_json::from_str::<T>(&response_text) {
        Ok(o) => Ok(o),
        Err(e) => {
            log::error!(
                "Error processing type {:?}: {}",
                std::any::type_name::<T>(),
                e
            );
            log::error!("Error response: {}", &response_text);

            match serde_json::from_str::<shared::ErrorMessage>(&response_text) {
                Ok(error_message) => Err(SpaceTradersClientError::ApiError(error_message)),
                Err(e) => Err(SpaceTradersClientError::JsonParse(e)),
            }
        }
    }
}

/// Claim a username and get a token
///
/// # Arguments
///
/// * `username` - A string containing the username to get a token for
pub async fn claim_username(
    http_client: HttpClient,
    username: String,
) -> Result<responses::ClaimUsername, SpaceTradersClientError> {
    let http_client = http_client.lock().await;
    let response = http_client
        .execute_request(
            "POST",
            &format!("https://api.spacetraders.io/users/{}/token", username),
            Some("{\"message\":\"this body doesn't actually matter\"}"),
            None,
        )
        .await?;

    parse_response::<responses::ClaimUsername>(&response.response_text)
}

/// Get the status of the game API.
pub async fn get_game_status(
    http_client: HttpClient,
) -> Result<responses::GameStatus, SpaceTradersClientError> {
    let http_client = http_client.lock().await;
    let response = http_client
        .execute_request("GET", "https://api.spacetraders.io/game/status", None, None)
        .await?;

    if response.status_code == StatusCode::SERVICE_UNAVAILABLE.as_u16() {
        return Err(SpaceTradersClientError::ServiceUnavailable);
    }

    parse_response::<responses::GameStatus>(&response.response_text)
}

/// Get the users current ip address
pub async fn get_my_ip_address(
    http_client: HttpClient,
) -> Result<responses::MyIpAddress, SpaceTradersClientError> {
    let http_client = http_client.lock().await;
    let response = http_client
        .execute_request("GET", "https://api.ipify.org?format=json", None, None)
        .await?;

    parse_response::<responses::MyIpAddress>(&response.response_text)
}

/// A SpaceTraders client that is associated to a specific username
#[derive(Debug, Clone)]
pub struct Client {
    http_client: HttpClient,
    /// The users username
    pub username: String,
    /// The uses access token
    pub token: String,
}

impl Client {
    /// Create a new game with a reqwest client that has the Authorization header set
    ///
    /// # Arguments
    ///
    /// * `username` - A string containing the username of the current player
    /// * `token` - A string containing the access token for the username provided
    pub fn new(http_client: HttpClient, username: String, token: String) -> Client {
        Client {
            http_client,
            username,
            token,
        }
    }

    //////////////////////////////////////////////
    ///// ACCOUNT
    //////////////////////////////////////////////

    /// Get all information about the current user
    pub async fn get_my_info(&self) -> Result<responses::UserInfo, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                "https://api.spacetraders.io/my/account",
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::UserInfo>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// FLIGHT PLANS
    //////////////////////////////////////////////

    /// Get the current details of a flight plan
    ///
    /// # Arguments
    ///
    /// * `flight_plan_id` - A string containing the flight plan id
    pub async fn get_flight_plan(
        &self,
        flight_plan_id: String,
    ) -> Result<responses::FlightPlan, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!(
                    "https://api.spacetraders.io/my/flight-plans/{}",
                    flight_plan_id
                ),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::FlightPlan>(&response.response_text)
    }

    /// Create a flight plan.
    ///
    /// # Arguments
    ///
    /// * `ship_id` - A string containing the ship_id to create the flight plan for
    /// * `destination` - A string containing the location to send the ship to
    pub async fn create_flight_plan(
        &self,
        ship_id: String,
        destination: String,
    ) -> Result<responses::FlightPlan, SpaceTradersClientError> {
        let flight_plan_request = requests::FlightPlanRequest {
            ship_id: ship_id.clone(),
            destination: destination.clone(),
        };

        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                "https://api.spacetraders.io/my/flight-plans",
                Some(&serde_json::to_string(&flight_plan_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::FlightPlan>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// LEADERBOARD
    //////////////////////////////////////////////

    // TODO: leaderboard/networth

    //////////////////////////////////////////////
    ///// LOANS
    //////////////////////////////////////////////

    /// Get any loans taken out by the current user
    pub async fn get_my_loans(&self) -> Result<responses::LoanInfo, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                "https://api.spacetraders.io/my/loans",
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::LoanInfo>(&response.response_text)
    }

    /// Pay off a loan completely
    ///
    /// # Arguments
    ///
    /// * `loan_id` - A string containing the loan_id of the loan to pay off
    pub async fn pay_off_loan(
        &self,
        loan_id: &str,
    ) -> Result<responses::PayLoanResponse, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "PUT",
                &format!("https://api.spacetraders.io/my/loans/{}", loan_id),
                Some("{\"message\":\"this body doesn't actually matter\"}"),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::PayLoanResponse>(&response.response_text)
    }

    /// Request a new loan
    ///
    /// # Arguments
    ///
    /// * `loan_type` - A LoanType with the type of loan being requested for the current user
    pub async fn request_new_loan(
        &self,
        loan_type: shared::LoanType,
    ) -> Result<responses::RequestLoan, SpaceTradersClientError> {
        let request_new_loan_request = requests::RequestNewLoanRequest { loan_type };

        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                "https://api.spacetraders.io/my/loans",
                Some(&serde_json::to_string(&request_new_loan_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::RequestLoan>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// LOCATIONS
    //////////////////////////////////////////////

    /// Get location info about a specific location
    ///
    /// # Arguments
    ///
    /// * `location_symbol` - A string containing the location name to get info about
    pub async fn get_location_info(
        &self,
        location_symbol: String,
    ) -> Result<responses::LocationInfo, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("https://api.spacetraders.io/locations/{}", location_symbol),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::LocationInfo>(&response.response_text)
    }

    // TODO:
    // /// Get all the locations in a particular system
    // ///
    // /// # Arguments
    // ///
    // /// * `system_symbol` - A string containing the system name to get the locations from
    // /// * `location_type` - An optional LocationType if you want to filter the locations by type
    // pub async fn get_locations_in_system(&self, system_symbol: String) -> Result<responses::AvailableLocations, SpaceTradersClientError> {
    //     let http_client = self.http_client.lock().await;
    //     let response = http_client.execute_request(
    //         "GET",
    //         &format!("https://api.spacetraders.io/game/systems/{}/locations", system_symbol),
    //         None,
    //         Some(&self.token),
    //     )
    //         .await?;
    //
    //     parse_response::<responses::AvailableLocations>(&response.response_text)
    // }

    /// Get the marketplace data about a location.
    ///
    /// # Note
    ///
    /// You must have a ship docked at the location in order to get it's marketplace data
    ///
    /// # Arguments
    ///
    /// * `location_symbol` - A string containing the name of the location to get marketplace data for
    pub async fn get_location_marketplace(
        &self,
        location_symbol: &str,
    ) -> Result<responses::LocationMarketplace, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!(
                    "https://api.spacetraders.io/locations/{}/marketplace",
                    location_symbol
                ),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::LocationMarketplace>(&response.response_text)
    }

    // TODO: Get Ships at a location

    //////////////////////////////////////////////
    ///// PURCHASE ORDERS
    //////////////////////////////////////////////

    /// Create a purchase order to transfer goods from a location to your ship
    ///
    /// # Arguments
    ///
    /// * `ship` - A Ship struct that you'd like to transfer the goods into
    /// * `good` - A Good enum containing the type of good you'd like to transfer
    /// * `quantity` - An i32 containing the quantity of good you'd like transferred
    pub async fn create_purchase_order(
        &self,
        ship_id: String,
        good: shared::Good,
        quantity: i32,
    ) -> Result<responses::PurchaseOrder, SpaceTradersClientError> {
        let purchase_order_request = requests::PurchaseOrderRequest {
            ship_id: ship_id.clone(),
            good,
            quantity,
        };

        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                "https://api.spacetraders.io/my/purchase-orders",
                Some(&serde_json::to_string(&purchase_order_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::PurchaseOrder>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// SELL ORDERS
    //////////////////////////////////////////////

    /// Create a sell order to transfer good from your ship to a location. Your ship will
    /// automatically sell the good to whatever location it is docked at
    ///
    /// # Arguments
    ///
    /// * `ship` - A Ship struct that you'd like to transfer the goods from
    /// * `good` - A Good enum containing the type of good you'd like to transfer
    /// * `quantity` - An i32 containing the quantity of good you'd like transferred
    pub async fn create_sell_order(
        &self,
        ship_id: String,
        good: shared::Good,
        quantity: i32,
    ) -> Result<responses::PurchaseOrder, SpaceTradersClientError> {
        let sell_order_request = requests::SellOrderRequest {
            ship_id: ship_id.clone(),
            good,
            quantity,
        };

        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                "https://api.spacetraders.io/my/sell-orders",
                Some(&serde_json::to_string(&sell_order_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::PurchaseOrder>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// SHIPS
    //////////////////////////////////////////////

    /// Add a ship to the users inventory by purchasing it
    ///
    /// # Arguments
    ///
    /// * `location_symbol` - A string containing the location you'd like to purchase the ship from
    /// * `ship_type` - A string containing the type of ship you'd like to purchase
    pub async fn purchase_ship(
        &self,
        location_symbol: String,
        ship_type: String,
    ) -> Result<responses::PurchaseShip, SpaceTradersClientError> {
        let purchase_ship_request = requests::PurchaseShipRequest {
            location: location_symbol.clone(),
            ship_type: ship_type.clone(),
        };

        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                "https://api.spacetraders.io/my/ships",
                Some(&serde_json::to_string(&purchase_ship_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::PurchaseShip>(&response.response_text)
    }

    /// Get info about a specific ship for the current user
    ///
    /// # Arguments
    ///
    /// * `ship_id` - A string id of the ship you'd like info about
    pub async fn get_my_ship(
        &self,
        ship_id: &str,
    ) -> Result<responses::MyShip, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!("https://api.spacetraders.io/my/ships/{}", ship_id),
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::MyShip>(&response.response_text)
    }

    /// Get all your ships
    pub async fn get_my_ships(&self) -> Result<responses::MyShips, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                "https://api.spacetraders.io/my/ships",
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::MyShips>(&response.response_text)
    }

    /// Jettison cargo from a ship
    pub async fn jettison_cargo(
        &self,
        ship_id: &str,
        good: shared::Good,
        quantity: i32,
    ) -> Result<responses::JettisonCargo, SpaceTradersClientError> {
        let jettison_cargo_request = requests::JettisonCargo { good, quantity };

        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                &format!("https://api.spacetraders.io/my/ships/{}/jettison", ship_id),
                Some(&serde_json::to_string(&jettison_cargo_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::JettisonCargo>(&response.response_text)
    }

    // TODO: Jettison cargo
    // TODO: Scrap your ship for credits
    // TODO: Transfer cargo between ships

    //////////////////////////////////////////////
    ///// STRUCTURES
    //////////////////////////////////////////////

    // TODO: Create a new structure
    // TODO: Deposit goods to a structure you own
    // TODO: Deposit goods to a structure
    // TODO: See specific structure
    // TODO: Transfer goods from your structure to a ship
    // TODO: Use to see a specific structure
    // TODO: Use to see all of your structures

    //////////////////////////////////////////////
    ///// SYSTEMS
    //////////////////////////////////////////////

    /// Get a list of all available ships in the system
    pub async fn get_ships_for_sale(
        &self,
        system_symbol: &str,
    ) -> Result<responses::ShipsForSale, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                &format!(
                    "https://api.spacetraders.io/systems/{}/ship-listings",
                    system_symbol
                ),
                None,
                Some(&self.token),
            )
            .await?;
        parse_response::<responses::ShipsForSale>(&response.response_text)
    }

    // TODO: Get all active flight plans in the system.
    // TODO: Get info on a system's docked ships
    // TODO: Get location info for a system
    // TODO: Get systems info

    // TODO: I'm not sure which endpoint this is supposed to be converted to
    /// Get information about all systems
    pub async fn get_systems_info(
        &self,
    ) -> Result<responses::SystemsInfo, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                "https://api.spacetraders.io/game/systems",
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::SystemsInfo>(&response.response_text)
    }

    //////////////////////////////////////////////
    ///// TYPES
    //////////////////////////////////////////////

    // TODO: Get available goods

    /// Get all available loans
    pub async fn get_available_loans(
        &self,
    ) -> Result<responses::AvailableLoans, SpaceTradersClientError> {
        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "GET",
                "https://api.spacetraders.io/types/loans",
                None,
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::AvailableLoans>(&response.response_text)
    }

    // TODO: Get available structures
    // TODO: Get info on available ships

    //////////////////////////////////////////////
    ///// WARP JUMP
    //////////////////////////////////////////////

    /// Attempt a warp jump
    pub async fn attempt_warp_jump(
        &self,
        ship_id: String,
    ) -> Result<responses::FlightPlan, SpaceTradersClientError> {
        let warp_jump_request = requests::WarpJump { ship_id };

        let http_client = self.http_client.lock().await;
        let response = http_client
            .execute_request(
                "POST",
                "https://api.spacetraders.io/my/warp-jumps",
                Some(&serde_json::to_string(&warp_jump_request).unwrap()),
                Some(&self.token),
            )
            .await?;

        parse_response::<responses::FlightPlan>(&response.response_text)
    }
}
