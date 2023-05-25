mod api;

#[macro_use]
extern crate log;

use crate::api::{Api, ApiError};
use chrono::{DateTime, Local};
use dotenv::dotenv;
use env_logger::{Env, Target};
use reqwest::header::Values;
use serde_json::Value;
use spacetraders_sdk::apis::agents_api::get_my_agent;
use spacetraders_sdk::apis::contracts_api::{
    accept_contract, deliver_contract, fulfill_contract, get_contract, get_contracts,
    AcceptContractParams, DeliverContractParams, FulfillContractParams, GetContractParams,
    GetContractsParams,
};
use spacetraders_sdk::apis::factions_api::{get_faction, GetFactionParams};
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
use spacetraders_sdk::apis::{Error, ResponseContent};
use spacetraders_sdk::models::register_request::Faction;
use spacetraders_sdk::models::waypoint_trait::Symbol;
use spacetraders_sdk::models::{
    ContractDeliverGood, DeliverContractRequest, ExtractResourcesRequest, NavigateShipRequest,
    PurchaseShipRequest, RegisterRequest, SellCargoRequest, Ship, ShipCargoItem, ShipRole,
    ShipType, Survey, WaypointTrait, WaypointType,
};
use std::cmp::min;
use std::collections::HashMap;
use std::{env, process};

const BASE_URL: &str = "https://api.spacetraders.io";
const ACCOUNT_SYMBOL: &str = "BLOVE";
// const FACTION: &str = "COSMIC";
// const CONTRACT_ID: &str = "clgq3der73qs8s60dvfrf05yj";

struct ErrorInfo {
    code: u32,
    message: String,
    data: Option<Value>,
}

fn get_error_info(e: Error) -> ErrorInfo {
    match e {
        Error::ResponseError(re) => {
            let response_content_entity = re.entity.unwrap();

            ErrorInfo {
                code: response_content_entity.error.code,
                message: format!(
                    "error status: {}, error code: {} error message: {}",
                    re.status,
                    response_content_entity.error.code,
                    response_content_entity.error.message,
                ),
                data: Some(response_content_entity.error.data),
            }
        }
        _ => ErrorInfo {
            code: 0,
            message: format!("unknown error"),
            data: None,
        },
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let env = Env::default()
        .filter_or("RUST_LOG", "spacetraders_rs=trace")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::Builder::from_env(env)
        .target(Target::Stdout)
        .init();

    let access_token = match env::var("ACCESS_TOKEN") {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };

    let api = Api::new(access_token);

    // let register_response = Api::register(ACCOUNT_SYMBOL.to_string(), Faction::Cosmic).await?;
    // info!("Register response: {:?}", register_response);

    let my_agent = api.get_my_agent().await?;
    info!("My agent: {:?}", my_agent);
    let my_contracts = api.get_my_contracts().await?;
    info!("My contracts {:?}", my_contracts);

    let mut my_contract = my_contracts.get(0).unwrap().clone();

    if !my_contract.accepted {
        my_contract = api.accept_contract(my_contract.id).await?;
    }
    info!("My contract: {:?}", my_contract);

    let my_ships = api.get_my_ships().await?;
    info!("My ships {:?}", my_ships);

    let my_headquarters = my_agent.headquarters.clone();
    let last_hyphen = my_headquarters.rfind("-").unwrap();
    let my_system = &my_headquarters[..last_hyphen];

    info!("My headquarters: {}", my_headquarters);
    info!("My system: {}", my_system);

    let starting_waypoint = api
        .get_waypoint(my_system.to_string(), my_headquarters.to_string())
        .await?;
    info!("Starting waypoint: {:?}", starting_waypoint);

    let system_waypoints = api.get_system_waypoints(my_system.to_string()).await?;
    info!("System Waypoints: {:?}", system_waypoints);

    let shipyard = system_waypoints.clone().into_iter().find(|w| {
        w.traits
            .clone()
            .into_iter()
            .find(|t| t.symbol == Symbol::Shipyard)
            .is_some()
    });

    let shipyard = match shipyard {
        Some(s) => s,
        None => panic!("unable to find available shipyard"),
    };

    info!("First shipyard: {:?}", shipyard);

    let shipyard_waypoint_symbol = shipyard.symbol.clone();
    let shipyard_system_symbol = shipyard.system_symbol.clone();

    let shipyard_data = api
        .get_shipyard(shipyard_system_symbol, shipyard_waypoint_symbol.clone())
        .await?;
    info!("Shipyard data: {:?}", shipyard_data);

    let my_mining_ship = async {
        let my_mining_ship = my_ships
            .into_iter()
            .find(|s| s.registration.role != ShipRole::Command);

        if let Some(ship) = my_mining_ship {
            return ship;
        }

        let ship = api
            .purchase_ship(shipyard_waypoint_symbol.clone(), ShipType::MiningDrone)
            .await;

        if let Ok(ship) = ship {
            info!("Purchased ship: {:?}", ship);
            return ship;
        }

        panic!("Couldn't purchase a ship and didn't have one in inventory");
    }
    .await;

    let asteroid_field = system_waypoints
        .clone()
        .into_iter()
        .find(|w| w.r#type == WaypointType::AsteroidField);

    let asteroid_field = match asteroid_field {
        Some(s) => s,
        None => panic!("unable to find asteroid field"),
    };

    info!("Asteroid field: {:?}", asteroid_field);

    let asteroid_field_waypoint_symbol = asteroid_field.symbol.clone();
    let mining_ship_symbol = my_mining_ship.symbol;

    let dock_results = api.dock_ship(mining_ship_symbol.to_string()).await?;
    info!("Dock results: {:?}", dock_results);

    let refuel_results = api.refuel_ship(mining_ship_symbol.to_string()).await?;
    info!("Refuel results: {:?}", refuel_results);

    let orbit_results = api
        .put_ship_in_orbit(mining_ship_symbol.to_string())
        .await?;
    info!("Orbit results: {:?}", orbit_results);

    // DEBUG:
    // let delivery_waypoint_symbol = "X1-ZA40-15970B";
    // info!(
    //     "Navigating to delivery waypoint {}",
    //     delivery_waypoint_symbol
    // );
    // let navigation_results = navigate_ship(
    //     &conf,
    //     NavigateShipParams {
    //         ship_symbol: mining_ship_symbol.to_string(),
    //         navigate_ship_request: Some(NavigateShipRequest {
    //             waypoint_symbol: delivery_waypoint_symbol.to_string(),
    //         }),
    //     },
    // )
    // .await;
    // match navigation_results {
    //     Ok(nr) => {
    //         info!("Navigation results: {:?}", nr);
    //
    //         let arrival = DateTime::parse_from_rfc3339(&nr.data.nav.route.arrival).unwrap();
    //         let arrival_seconds = arrival.signed_duration_since(Local::now());
    //         info!(
    //             "Awaiting arrival of ship ({} seconds)",
    //             arrival_seconds.num_seconds()
    //         );
    //         tokio::time::sleep(tokio::time::Duration::from_secs(
    //             arrival_seconds.num_seconds() as u64,
    //         ))
    //         .await;
    //     }
    //     Err(nre) => {
    //         let ei = get_error_info(nre);
    //         // We can skip any 4204 errors since that just means we are already at the destination
    //         if ei.code != 4204 {
    //             error!(
    //                 "Navigation results error ({}), waiting 10 seconds and starting loop again: {}",
    //                 ei.code, ei.message
    //             );
    //             tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    //         }
    //     }
    // };
    // END DEBUG:

    println!("Navigating to asteroid field");
    let navigation_results = api
        .navigate_ship(
            mining_ship_symbol.to_string(),
            asteroid_field_waypoint_symbol.to_string(),
        )
        .await;

    match navigation_results {
        Ok(nr) => println!("Navigation results: {:?}", nr),
        Err(ApiError::SameDestination) => println!("Ship is already at destination"),
        _ => panic!("Unhandled error: {:?}", navigation_results),
    }

    'main: loop {
        let ship = api.get_my_ship(mining_ship_symbol.clone()).await;
        let ship = match ship {
            Ok(sr) => sr,
            Err(sre) => {
                error!(
                    "Error getting ship. Waiting for 10 seconds and trying again: {:?}",
                    sre
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            }
        };

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        info!(
            "Current ship capacity is {} and current cargo units is {}",
            ship.cargo.capacity, ship.cargo.units
        );
        info!("Current ship cargo is {:?}", ship.cargo.inventory);

        let mut contract_goods: HashMap<String, String> = HashMap::new();
        for good in (*my_contract.clone().terms)
            .deliver
            .expect("contract didn't provide delivery terms")
            .iter()
        {
            contract_goods
                .entry(good.trade_symbol.clone())
                .or_insert(good.destination_symbol.clone());
        }

        if ship.cargo.units == ship.cargo.capacity - 10 {
            for i in ship.cargo.inventory.clone() {
                if contract_goods.contains_key(&i.symbol) {
                    let delivery_waypoint_symbol = contract_goods.get(&i.symbol).unwrap();

                    info!("Ship is full of {}", i.symbol);

                    let dock_result = api.dock_ship(mining_ship_symbol.clone()).await;
                    match dock_result {
                        Ok(dr) => info!("Dock result: {:?}", dr),
                        Err(dre) => {
                            error!(
                                "Dock result error, waiting 10 seconds and starting loop again: {:?}",
                                dre
                            );
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                    info!("Refueling");
                    let refueling_result = api.refuel_ship(mining_ship_symbol.clone()).await;
                    match refueling_result {
                        Ok(rr) => info!("Refueling result: {:?}", rr),
                        Err(rre) => {
                            error!("Refueling result error, waiting 10 seconds and starting loop again: {:?}", rre);
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                    info!(
                        "Navigating to delivery waypoint {}",
                        delivery_waypoint_symbol
                    );
                    let navigation_results = api
                        .navigate_ship(
                            mining_ship_symbol.to_string(),
                            delivery_waypoint_symbol.to_string(),
                        )
                        .await;
                    match navigation_results {
                        Ok(nr) => println!("Navigation results: {:?}", nr),
                        Err(ApiError::SameDestination) => {
                            println!("Ship is already at destination")
                        }
                        _ => panic!("Unhandled error: {:?}", navigation_results),
                    }

                    info!("Docking");
                    let dock_result = api.dock_ship(mining_ship_symbol.to_string()).await;
                    match dock_result {
                        Ok(dr) => info!("Dock result: {:?}", dr),
                        Err(dre) => {
                            error!(
                                "Dock result error, waiting 10 seconds and starting loop again: {:?}",
                                dre
                            );
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                    info!("Refueling");
                    let refueling_result = api.refuel_ship(mining_ship_symbol.clone()).await;
                    match refueling_result {
                        Ok(rr) => info!("Refueling result: {:?}", rr),
                        Err(rre) => {
                            error!("Refueling result error, waiting 10 seconds and starting loop again: {:?}", rre);
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                    info!(
                        "Delivering {} of {} to {}",
                        i.units, i.symbol, delivery_waypoint_symbol
                    );
                    let delivery_result = api
                        .deliver_contract(
                            my_contract.id.clone(),
                            mining_ship_symbol.clone(),
                            i.symbol,
                            i.units,
                        )
                        .await;
                    match delivery_result {
                        Ok(dr) => info!("Delivery result: {:?}", dr),
                        Err(dre) => {
                            error!(
                                "Delivery result error, waiting 10 seconds and starting loop again: {}",
                                dre
                            );
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                    let contract_results = api.get_contract(my_contract.id.clone()).await;
                    match contract_results {
                        Ok(cr) => {
                            info!("Updated contract results: {:?}", cr);
                            if !cr.fulfilled
                                && cr
                                    .terms
                                    .deliver
                                    .unwrap()
                                    .iter()
                                    .all(|i| i.units_fulfilled == i.units_required)
                            {
                                let fulfill_contract_results =
                                    api.fulfill_contract(my_contract.id.clone()).await;

                                match fulfill_contract_results {
                                    Ok(fcr) => info!("Fulfill contract results: {:?}", fcr),
                                    Err(fcre) => {
                                        error!("Fulfill contract result error: {:?}", fcre);
                                    }
                                }

                                info!(
                                    "Technically this script is done since the contract is fulfilled"
                                );
                                process::exit(0);
                            }
                        }
                        Err(cre) => {
                            error!("Get contract results error: not waiting since this is a non-critical call: {:?}", cre);
                        }
                    }
                }

                info!("Going back into orbit");
                let orbit_result = api.put_ship_in_orbit(mining_ship_symbol.clone()).await;
                match orbit_result {
                    Ok(or) => info!("Orbit result: {:?}", or),
                    Err(ore) => {
                        error!(
                            "Orbit result error, waiting 10 seconds and starting loop again: {:?}",
                            ore
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Returning to asteroid field");
                let navigation_results = api
                    .navigate_ship(
                        mining_ship_symbol.to_string(),
                        asteroid_field_waypoint_symbol.to_string(),
                    )
                    .await;
                match navigation_results {
                    Ok(nr) => println!("Navigation results: {:?}", nr),
                    Err(ApiError::SameDestination) => println!("Ship is already at destination"),
                    _ => panic!("Unhandled error: {:?}", navigation_results),
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Docking ship to refuel");
                let docking_result = api.dock_ship(mining_ship_symbol.to_string()).await;
                match docking_result {
                    Ok(dr) => info!("Docking Result: {:?}", dr),
                    Err(dre) => {
                        error!(
                            "Docking results error, waiting 10 seconds and starting loop again: {:?}",
                            dre
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Refueling");
                let refueling_result = api.refuel_ship(mining_ship_symbol.to_string()).await;
                match refueling_result {
                    Ok(rr) => info!("Refueling result: {:?}", rr),
                    Err(rre) => {
                        error!(
                            "Refueling result error, waiting 10 seconds and starting loop again: {:?}",
                            rre
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Going back into orbit");
                let orbit_result = api.put_ship_in_orbit(mining_ship_symbol.clone()).await;
                match orbit_result {
                    Ok(or) => info!("Orbit result: {:?}", or),
                    Err(ore) => {
                        error!(
                            "Orbit result error, waiting 10 seconds and starting loop again: {}",
                            ore
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }

        if ship.cargo.units == ship.cargo.capacity {
            info!("Docking");
            let dock_results = api.dock_ship(mining_ship_symbol.to_string()).await?;
            info!("Dock results: {:?}", dock_results);

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            for i in ship.cargo.inventory.clone() {
                if !contract_goods.contains_key(&i.symbol) {
                    info!("Selling {} of {}", i.units, i.symbol);
                    let sell_results = api
                        .sell_cargo(mining_ship_symbol.clone(), i.symbol, i.units)
                        .await;
                    match sell_results {
                        Ok(sr) => info!("Sell results: {:?}", sr),
                        Err(sre) => {
                            error!(
                                "Sell results error: waiting for 10 seconds and restarting loop {}",
                                sre
                            );
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }

            info!("Going back into orbit");
            let orbit_result = api.put_ship_in_orbit(mining_ship_symbol.clone()).await;
            match orbit_result {
                Ok(or) => info!("Orbit result: {:?}", or),
                Err(ore) => {
                    error!(
                        "Orbit result error, waiting 10 seconds and starting loop again: {}",
                        ore
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    continue 'main;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        info!("Extracting");
        let extract_results = api.extract_resources(mining_ship_symbol.clone()).await;
        match extract_results {
            Ok(er) => {
                info!("Extract results: {:?}", er);
            }
            Err(ere) => {
                error!(
                    "Extract results error, waiting 10 seconds and starting loop again: {:?}",
                    ere
                );

                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue 'main;
            }
        }
    }

    Ok(())

    // 'main: loop {
    //
    //
    //
    // }
}
