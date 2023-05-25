use async_recursion::async_recursion;
use chrono::{DateTime, Local};
use serde_json::Value;
use spacetraders_sdk::apis::agents_api::get_my_agent;
use spacetraders_sdk::apis::configuration::Configuration;
use spacetraders_sdk::apis::contracts_api::{
    accept_contract, deliver_contract, fulfill_contract, get_contract, get_contracts,
    AcceptContractParams, DeliverContractParams, FulfillContractParams, GetContractParams,
    GetContractsParams,
};
use spacetraders_sdk::apis::default_api::{register, RegisterParams};
use spacetraders_sdk::apis::fleet_api::{
    dock_ship, extract_resources, get_my_ship, get_my_ships, navigate_ship, orbit_ship,
    purchase_ship, refuel_ship, sell_cargo, DockShipParams, ExtractResourcesParams,
    GetMyShipParams, GetMyShipsParams, NavigateShipParams, OrbitShipParams, PurchaseShipParams,
    RefuelShipParams, SellCargoParams,
};
use spacetraders_sdk::apis::systems_api::{
    get_shipyard, get_system_waypoints, get_waypoint, GetShipyardParams, GetSystemWaypointsParams,
    GetWaypointParams,
};
use spacetraders_sdk::apis::Error;
use spacetraders_sdk::models::register_request::Faction;
use spacetraders_sdk::models::{
    Agent, Contract, DeliverContractRequest, NavigateShipRequest, PurchaseShipRequest,
    Register201Response, Register201ResponseData, RegisterRequest, SellCargoRequest, Ship,
    ShipCargo, ShipFuel, ShipNav, ShipType, Shipyard, Waypoint,
};

// General error codes
const UNAUTHORIZED: u32 = 401;
const COOLDOWN_CONFLICT_ERROR: u32 = 4000;
const WAYPOINT_NO_ACCESS_ERROR: u32 = 4001;
const TOKEN_EMPTY_ERROR: u32 = 4100;
// Account error codes
const TOKEN_MISSING_SUBJECT_ERROR: u32 = 4101;
const TOKEN_INVALID_SUBJECT_ERROR: u32 = 4102;
const MISSING_TOKEN_REQUEST_ERROR: u32 = 4103;
const INVALID_TOKEN_REQUEST_ERROR: u32 = 4104;
const INVALID_TOKEN_SUBJECT_ERROR: u32 = 4105;
const ACCOUNT_NOT_EXISTS_ERROR: u32 = 4106;
const AGENT_NOT_EXISTS_ERROR: u32 = 4107;
const ACCOUNT_HAS_NO_AGENT_ERROR: u32 = 4108;
const REGISTER_AGENT_EXISTS_ERROR: u32 = 4109;
// Ship error codes
const NAVIGATE_IN_TRANSIT_ERROR: u32 = 4200;
const NAVIGATE_INVALID_DESTINATION_ERROR: u32 = 4201;
const NAVIGATE_OUTSIDE_SYSTEM_ERROR: u32 = 4202;
const NAVIGATE_INSUFFICIENT_FUEL_ERROR: u32 = 4203;
const NAVIGATE_SAME_DESTINATION_ERROR: u32 = 4204;
const SHIP_EXTRACT_INVALID_WAYPOINT_ERROR: u32 = 4205;
const SHIP_EXTRACT_PERMISSION_ERROR: u32 = 4206;
const SHIP_JUMP_NO_SYSTEM_ERROR: u32 = 4207;
const SHIP_JUMP_SAME_SYSTEM_ERROR: u32 = 4208;
const SHIP_JUMP_MISSING_MODULE_ERROR: u32 = 4210;
const SHIP_JUMP_NO_VALID_WAYPOINT_ERROR: u32 = 4211;
const SHIP_JUMP_MISSING_ANTIMATTER_ERROR: u32 = 4212;
const SHIP_IN_TRANSIT_ERROR: u32 = 4214;
const SHIP_MISSING_SENSOR_ARRAYS_ERROR: u32 = 4215;
const PURCHASE_SHIP_CREDITS_ERROR: u32 = 4216;
const SHIP_CARGO_EXCEEDS_LIMIT_ERROR: u32 = 4217;
const SHIP_CARGO_MISSING_ERROR: u32 = 4218;
const SHIP_CARGO_UNIT_COUNT_ERROR: u32 = 4219;
const SHIP_SURVEY_VERIFICATION_ERROR: u32 = 4220;
const SHIP_SURVEY_EXPIRATION_ERROR: u32 = 4221;
const SHIP_SURVEY_WAYPOINT_TYPE_ERROR: u32 = 4222;
const SHIP_SURVEY_ORBIT_ERROR: u32 = 4223;
const SHIP_SURVEY_EXHAUSTED_ERROR: u32 = 4224;
const SHIP_REFUEL_DOCKED_ERROR: u32 = 4225;
const SHIP_REFUEL_INVALID_WAYPOINT_ERROR: u32 = 4226;
const SHIP_MISSING_MOUNTS_ERROR: u32 = 4227;
const SHIP_CARGO_FULL_ERROR: u32 = 4228;
const SHIP_JUMP_FROM_GATE_TO_GATE_ERROR: u32 = 4229;
const WAYPOINT_CHARTED_ERROR: u32 = 4230;
const SHIP_TRANSFER_SHIP_NOT_FOUND: u32 = 4231;
const SHIP_TRANSFER_AGENT_CONFLICT: u32 = 4232;
const SHIP_TRANSFER_SAME_SHIP_CONFLICT: u32 = 4233;
const SHIP_TRANSFER_LOCATION_CONFLICT: u32 = 4234;
const WARP_INSIDE_SYSTEM_ERROR: u32 = 4235;
const SHIP_NOT_IN_ORBIT_ERROR: u32 = 4236;
const SHIP_INVALID_REFINERY_GOOD_ERROR: u32 = 4237;
const SHIP_INVALID_REFINERY_TYPE_ERROR: u32 = 4238;
const SHIP_MISSING_REFINERY_ERROR: u32 = 4239;
const SHIP_MISSING_SURVEYOR_ERROR: u32 = 4240;
// Contract error codes
const ACCEPT_CONTRACT_NOT_AUTHORIZED_ERROR: u32 = 4500;
const ACCEPT_CONTRACT_CONFLICT_ERROR: u32 = 4501;
const FULFILL_CONTRACT_DELIVERY_ERROR: u32 = 4502;
const CONTRACT_DEADLINE_ERROR: u32 = 4503;
const CONTRACT_FULFILLED_ERROR: u32 = 4504;
const CONTRACT_NOT_ACCEPTED_ERROR: u32 = 4505;
const CONTRACT_NOT_AUTHORIZED_ERROR: u32 = 4506;
const SHIP_DELIVER_TERMS_ERROR: u32 = 4508;
const SHIP_DELIVER_FULFILLED_ERROR: u32 = 4509;
const SHIP_DELIVER_INVALID_LOCATION_ERROR: u32 = 4510;
// Market error codes
const MARKET_TRADE_INSUFFICIENT_CREDITS_ERROR: u32 = 4600;
const MARKET_TRADE_NO_PURCHASE_ERROR: u32 = 4601;
const MARKET_TRADE_NOT_SOLD_ERROR: u32 = 4602;
const MARKET_NOT_FOUND_ERROR: u32 = 4603;
const MARKET_TRADE_UNIT_LIMIT_ERROR: u32 = 4604;

#[derive(Debug)]
struct ErrorInfo {
    code: i32,
    message: String,
    data: Option<Value>,
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("unauthorized: {0:?}")]
    Unauthorized(ErrorInfo),
    #[error("in transit error")]
    InTransit,
    #[error("same destination error")]
    SameDestination,
    #[error("retryable error: {0:?}")]
    Retryable(ErrorInfo),
    #[error("generic error: {0:?}")]
    Generic(ErrorInfo),
}

#[derive(Debug)]
pub struct Api {
    configuration: Configuration,
}

impl Api {
    pub fn new(access_token: String) -> Api {
        Api {
            configuration: Configuration {
                bearer_access_token: Some(access_token),
                ..Default::default()
            },
        }
    }

    async fn handle_error(error: Error) -> ApiError {
        match error {
            Error::Reqwest(re) => ApiError::Generic(ErrorInfo {
                code: -1,
                message: re.to_string(),
                data: None,
            }),
            Error::Serde(se) => ApiError::Generic(ErrorInfo {
                code: -1,
                message: se.to_string(),
                data: None,
            }),
            Error::Io(ioe) => ApiError::Generic(ErrorInfo {
                code: -1,
                message: ioe.to_string(),
                data: None,
            }),
            Error::ResponseError(re) => {
                let response_content_entity = re.entity.unwrap();

                println!(
                    "handle_error: Response Error: {:?}",
                    response_content_entity
                );

                // Attempting to navigate to a destination where a ship already is at should be a
                // NOOP not an error so we can skip the error
                if response_content_entity.error.code == NAVIGATE_SAME_DESTINATION_ERROR {
                    return ApiError::SameDestination;
                }

                // If a command couldn't be executed because the ship was in transit then wait
                // for the transit time automatically and return a retryable error
                if response_content_entity.error.code == SHIP_IN_TRANSIT_ERROR {
                    let error_data = response_content_entity.error.data.clone();
                    let seconds_to_arrival = error_data.get("secondsToArrival");

                    let seconds_to_arrival = match seconds_to_arrival {
                        Some(s) => {
                            // If we weren't able to convert the value to a number then default to
                            // waiting 10 seconds. This should be okay because we will return a
                            // retryable error and the calling function will call this over and
                            // over again waiting for the call to succeed while waiting for 10
                            // seconds in between
                            s.as_u64().unwrap_or(10)
                        }
                        // If the response didn't contain a value to wait for then just wait for 10
                        // and try again later
                        None => 10,
                    };

                    info!(
                        "handle_error: Awaiting arrival of ship ({} seconds)",
                        seconds_to_arrival
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(seconds_to_arrival)).await;

                    return ApiError::Retryable(ErrorInfo {
                        code: response_content_entity.error.code as i32,
                        message: "Successfully waited for ship to arrive so try again".to_string(),
                        data: Some(response_content_entity.error.data),
                    });
                }

                if response_content_entity.error.code == UNAUTHORIZED {
                    return ApiError::Unauthorized(ErrorInfo {
                        code: response_content_entity.error.code as i32,
                        message: format!(
                            "error status: {}, error code: {} error message: {}",
                            re.status,
                            response_content_entity.error.code,
                            response_content_entity.error.message,
                        ),
                        data: Some(response_content_entity.error.data),
                    });
                }

                // Any other error will just get bubbled up
                ApiError::Generic(ErrorInfo {
                    code: response_content_entity.error.code as i32,
                    message: format!(
                        "error status: {}, error code: {} error message: {}",
                        re.status,
                        response_content_entity.error.code,
                        response_content_entity.error.message,
                    ),
                    data: Some(response_content_entity.error.data),
                })
            }
        }
    }

    pub async fn register(
        symbol: String,
        faction: Faction,
    ) -> Result<Register201ResponseData, spacetraders_sdk::apis::Error> {
        let register_response = register(
            &Configuration::default(),
            RegisterParams {
                register_request: Some(RegisterRequest { symbol, faction }),
            },
        )
        .await?;

        Ok(*register_response.data)
    }

    #[async_recursion]
    pub async fn get_my_agent(&self) -> Result<Agent, ApiError> {
        let my_agent = get_my_agent(&self.configuration).await;

        match my_agent {
            Ok(my_agent) => Ok(*my_agent.data),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.get_my_agent().await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn get_my_contracts(&self) -> Result<Vec<Contract>, ApiError> {
        let my_contracts = get_contracts(
            &self.configuration,
            GetContractsParams {
                page: None,
                limit: None,
            },
        )
        .await;

        match my_contracts {
            Ok(my_contracts) => Ok((*my_contracts.data).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.get_my_contracts().await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn get_contract(&self, contract_id: String) -> Result<Contract, ApiError> {
        let contract = get_contract(
            &self.configuration,
            GetContractParams {
                contract_id: contract_id.clone(),
            },
        )
        .await;

        match contract {
            Ok(contract) => Ok((*contract.data).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.get_contract(contract_id).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn accept_contract(&self, contract_id: String) -> Result<Contract, ApiError> {
        let accepted_contract = accept_contract(
            &self.configuration,
            AcceptContractParams {
                contract_id: contract_id.clone(),
            },
        )
        .await;

        match accepted_contract {
            Ok(accepted_contract) => Ok(*accepted_contract.data.contract),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.accept_contract(contract_id).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn get_my_ships(&self) -> Result<Vec<Ship>, ApiError> {
        let my_ships = get_my_ships(
            &self.configuration,
            GetMyShipsParams {
                page: None,
                limit: None,
            },
        )
        .await;

        match my_ships {
            Ok(my_ships) => Ok((*my_ships.data).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.get_my_ships().await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn get_my_ship(&self, ship_symbol: String) -> Result<Ship, ApiError> {
        let ship_result = get_my_ship(
            &self.configuration,
            GetMyShipParams {
                ship_symbol: ship_symbol.to_string(),
            },
        )
        .await;

        match ship_result {
            Ok(ship_result) => Ok((*ship_result.data).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.get_my_ship(ship_symbol).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn get_waypoint(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
    ) -> Result<Waypoint, ApiError> {
        let waypoint = get_waypoint(
            &self.configuration,
            GetWaypointParams {
                system_symbol: system_symbol.clone(),
                waypoint_symbol: waypoint_symbol.clone(),
            },
        )
        .await;

        match waypoint {
            Ok(waypoint) => Ok((*waypoint.data).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.get_waypoint(system_symbol, waypoint_symbol).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn get_system_waypoints(
        &self,
        system_symbol: String,
    ) -> Result<Vec<Waypoint>, ApiError> {
        let waypoints = get_system_waypoints(
            &self.configuration,
            GetSystemWaypointsParams {
                system_symbol: system_symbol.clone(),
                ..Default::default()
            },
        )
        .await;

        match waypoints {
            Ok(waypoints) => Ok((*waypoints.data).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.get_system_waypoints(system_symbol).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn get_shipyard(
        &self,
        shipyard_system_symbol: String,
        shipyard_waypoint_symbol: String,
    ) -> Result<Shipyard, ApiError> {
        let shipyard_data = get_shipyard(
            &self.configuration,
            GetShipyardParams {
                system_symbol: shipyard_system_symbol.clone(),
                waypoint_symbol: shipyard_waypoint_symbol.clone(),
            },
        )
        .await;

        match shipyard_data {
            Ok(shipyard_data) => Ok((*shipyard_data.data).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self
                        .get_shipyard(shipyard_system_symbol, shipyard_waypoint_symbol)
                        .await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn purchase_ship(
        &self,
        shipyard_waypoint_symbol: String,
        ship_type: ShipType,
    ) -> Result<Ship, ApiError> {
        let ship = purchase_ship(
            &self.configuration,
            PurchaseShipParams {
                purchase_ship_request: Some(PurchaseShipRequest {
                    waypoint_symbol: shipyard_waypoint_symbol.clone(),
                    ship_type,
                }),
            },
        )
        .await;

        match ship {
            Ok(ship) => Ok((*ship.data.ship).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self
                        .purchase_ship(shipyard_waypoint_symbol, ship_type)
                        .await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn dock_ship(&self, ship_symbol: String) -> Result<ShipNav, ApiError> {
        let dock_results = dock_ship(
            &self.configuration,
            DockShipParams {
                ship_symbol: ship_symbol.clone(),
            },
        )
        .await;

        match dock_results {
            Ok(dock_results) => Ok((*dock_results.data.nav).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.dock_ship(ship_symbol).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn refuel_ship(&self, ship_symbol: String) -> Result<ShipFuel, ApiError> {
        let refuel_results = refuel_ship(
            &self.configuration,
            RefuelShipParams {
                ship_symbol: ship_symbol.to_string(),
            },
        )
        .await;

        match refuel_results {
            Ok(refuel_results) => Ok((*refuel_results.data.fuel).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.refuel_ship(ship_symbol).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn put_ship_in_orbit(&self, ship_symbol: String) -> Result<ShipNav, ApiError> {
        let orbit_results = orbit_ship(
            &self.configuration,
            OrbitShipParams {
                ship_symbol: ship_symbol.to_string(),
            },
        )
        .await;

        match orbit_results {
            Ok(orbit_results) => Ok((*orbit_results.data.nav).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.put_ship_in_orbit(ship_symbol).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn navigate_ship(
        &self,
        ship_symbol: String,
        waypoint_symbol: String,
    ) -> Result<ShipNav, ApiError> {
        let navigation_results = navigate_ship(
            &self.configuration,
            NavigateShipParams {
                ship_symbol: ship_symbol.to_string(),
                navigate_ship_request: Some(NavigateShipRequest {
                    waypoint_symbol: waypoint_symbol.to_string(),
                }),
            },
        )
        .await;

        match navigation_results {
            Ok(navigation_results) => {
                let arrival =
                    DateTime::parse_from_rfc3339(&navigation_results.data.nav.route.arrival)
                        .unwrap();
                let arrival_seconds = arrival.signed_duration_since(Local::now());
                info!(
                    "Awaiting arrival of ship ({} seconds)",
                    arrival_seconds.num_seconds()
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    arrival_seconds.num_seconds() as u64,
                ))
                .await;

                Ok((*navigation_results.data.nav).to_owned())
            }
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.navigate_ship(ship_symbol, waypoint_symbol).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn deliver_contract(
        &self,
        contract_id: String,
        ship_symbol: String,
        trade_symbol: String,
        units: i32,
    ) -> Result<Contract, ApiError> {
        let delivery_result = deliver_contract(
            &self.configuration,
            DeliverContractParams {
                contract_id: contract_id.clone(),
                deliver_contract_request: Some(DeliverContractRequest {
                    ship_symbol: ship_symbol.clone(),
                    trade_symbol: trade_symbol.clone(),
                    units: units.clone(),
                }),
            },
        )
        .await;

        match delivery_result {
            Ok(delivery_result) => Ok((*delivery_result.data.contract).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self
                        .deliver_contract(contract_id, ship_symbol, trade_symbol, units)
                        .await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn fulfill_contract(&self, contract_id: String) -> Result<Contract, ApiError> {
        let fulfillment_result = fulfill_contract(
            &self.configuration,
            FulfillContractParams {
                contract_id: contract_id.clone(),
            },
        )
        .await;

        match fulfillment_result {
            Ok(fulfillment_result) => Ok((*fulfillment_result.data.contract).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.fulfill_contract(contract_id).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn sell_cargo(
        &self,
        ship_symbol: String,
        good_symbol: String,
        units: i32,
    ) -> Result<ShipCargo, ApiError> {
        let sell_cargo_result = sell_cargo(
            &self.configuration,
            SellCargoParams {
                ship_symbol: ship_symbol.clone(),
                sell_cargo_request: Some(SellCargoRequest {
                    symbol: good_symbol.clone(),
                    units,
                }),
            },
        )
        .await;

        match sell_cargo_result {
            Ok(sell_cargo_result) => Ok((*sell_cargo_result.data.cargo).to_owned()),
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.sell_cargo(ship_symbol, good_symbol, units).await;
                }

                return Err(e);
            }
        }
    }

    #[async_recursion]
    pub async fn extract_resources(&self, ship_symbol: String) -> Result<ShipCargo, ApiError> {
        let extract_resources_result = extract_resources(
            &self.configuration,
            ExtractResourcesParams {
                ship_symbol: ship_symbol.clone(),
                extract_resources_request: None,
            },
        )
        .await;

        match extract_resources_result {
            Ok(extract_resources_result) => {
                info!(
                    "Cooling down for {} seconds after extraction",
                    extract_resources_result.data.cooldown.remaining_seconds
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    extract_resources_result.data.cooldown.remaining_seconds as u64,
                ))
                .await;
                info!("Cool down over");

                Ok((*extract_resources_result.data.cargo).to_owned())
            }
            Err(e) => {
                let e = Api::handle_error(e).await;

                if let ApiError::Retryable(_) = e {
                    return self.extract_resources(ship_symbol).await;
                }

                return Err(e);
            }
        }
    }
}
