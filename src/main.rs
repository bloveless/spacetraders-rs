#[macro_use]
extern crate log;

use chrono::{DateTime, Local};
use dotenv::dotenv;
use env_logger::{Env, Target};
use reqwest::header::Values;
use serde_json::Value;
use spacetraders_sdk::apis::agents_api::get_my_agent;
use spacetraders_sdk::apis::configuration::Configuration;
use spacetraders_sdk::apis::contracts_api::{
    accept_contract, deliver_contract, fulfill_contract, get_contract, get_contracts,
    AcceptContractParams, DeliverContractParams, FulfillContractParams, GetContractParams,
    GetContractsParams,
};
use spacetraders_sdk::apis::default_api::{register, RegisterParams};
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
    DeliverContractRequest, ExtractResourcesRequest, NavigateShipRequest, PurchaseShipRequest,
    RegisterRequest, SellCargoRequest, Ship, ShipCargoItem, ShipRole, ShipType, Survey,
    WaypointTrait, WaypointType,
};
use std::{env, process};

const BASE_URL: &str = "https://api.spacetraders.io";
const ACCOUNT_SYMBOL: &str = "BLOVE-AGAIN";
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

    let conf = Configuration {
        bearer_access_token: Some(access_token),
        ..Default::default()
    };

    // let register_response = register(
    //     &conf,
    //     RegisterParams {
    //         register_request: Some(RegisterRequest {
    //             symbol: ACCOUNT_SYMBOL.to_string(),
    //             faction: Faction::Cosmic,
    //         }),
    //     },
    // )
    // .await?;
    //
    // info!("Register Response: {:?}", register_response);

    let my_agent = get_my_agent(&conf).await?;
    info!("My agent: {:?}", my_agent);

    let my_contracts = get_contracts(
        &conf,
        GetContractsParams {
            page: None,
            limit: None,
        },
    )
    .await?;

    info!("My contracts {:?}", my_contracts);

    let my_contract = my_contracts.data.get(0).unwrap().clone();
    let contract_id = my_contract.id;

    if !my_contract.accepted {
        let accepted_contract = accept_contract(
            &conf,
            AcceptContractParams {
                contract_id: contract_id.clone(),
            },
        )
        .await;

        match accepted_contract {
            Ok(ac) => info!("Accepted Contract: {:?}", ac),
            Err(e) => {
                let ei = get_error_info(e);
                panic!("Accept contract error ({}): {}", ei.code, ei.message);
            }
        }
    }

    // let my_faction = get_faction(
    //     &conf,
    //     GetFactionParams {
    //         faction_symbol: FACTION.to_string(),
    //     },
    // )
    // .await?;
    // info!("My faction {:?}", my_faction);

    let my_ships = get_my_ships(
        &conf,
        GetMyShipsParams {
            page: None,
            limit: None,
        },
    )
    .await?;
    info!("My ships {:?}", my_ships);

    let my_headquarters = my_agent.data.headquarters.clone();
    let last_hyphen = my_headquarters.rfind("-").unwrap();
    let my_system = &my_headquarters[..last_hyphen];

    let starting_waypoint = get_waypoint(
        &conf,
        GetWaypointParams {
            system_symbol: my_system.to_string(),
            waypoint_symbol: my_headquarters.clone(),
        },
    )
    .await?;
    info!("Starting waypoint: {:?}", starting_waypoint);

    let my_contract = get_contract(
        &conf,
        GetContractParams {
            contract_id: contract_id.clone(),
        },
    )
    .await?;
    info!("My Contract: {:?}", my_contract);

    // Find a shipyard
    let waypoints = get_system_waypoints(
        &conf,
        GetSystemWaypointsParams {
            system_symbol: my_system.to_string(),
            ..Default::default()
        },
    )
    .await?;
    info!("Waypoints: {:?}", waypoints);

    let shipyard = waypoints.data.clone().into_iter().find(|w| {
        w.traits
            .clone()
            .into_iter()
            .find(|t| t.symbol == Symbol::Shipyard)
            .is_some()
    });
    //
    let shipyard = match shipyard {
        Some(s) => s,
        None => panic!("unable to find available shipyard"),
    };

    info!("First shipyard: {:?}", shipyard);

    let shipyard_waypoint_symbol = shipyard.symbol.clone();
    let shipyard_system_symbol = shipyard.system_symbol.clone();

    let shipyard_data = get_shipyard(
        &conf,
        GetShipyardParams {
            system_symbol: shipyard_system_symbol.clone(),
            waypoint_symbol: shipyard_waypoint_symbol.clone(),
        },
    )
    .await;

    info!("Shipyard data: {:?}", shipyard_data);

    let my_mining_ship = async {
        let my_mining_ship = my_ships
            .data
            .into_iter()
            .find(|s| s.registration.role != ShipRole::Command);

        if my_mining_ship.is_some() {
            return my_mining_ship.unwrap();
        }

        // Purchase a ship
        let ship = purchase_ship(
            &conf,
            PurchaseShipParams {
                purchase_ship_request: Some(PurchaseShipRequest {
                    waypoint_symbol: shipyard_waypoint_symbol.clone(),
                    ship_type: ShipType::MiningDrone,
                }),
            },
        )
        .await;

        info!("Purchased ship: {:?}", ship);
        return *ship.unwrap().data.ship;
    }
    .await;

    let asteroid_field = waypoints
        .data
        .into_iter()
        .find(|w| w.r#type == WaypointType::AsteroidField);
    //

    let asteroid_field = match asteroid_field {
        Some(s) => s,
        None => panic!("unable to find asteroid field"),
    };

    info!("Asteroid field: {:?}", asteroid_field);

    let asteroid_field_waypoint_symbol = asteroid_field.symbol.clone();
    let mining_ship_symbol = my_mining_ship.symbol;

    // let dock_results = dock_ship(
    //     &conf,
    //     DockShipParams {
    //         ship_symbol: mining_ship_symbol.to_string(),
    //     },
    // )
    // .await?;
    // info!("Dock results: {:?}", dock_results);
    //
    // let refuel_results = refuel_ship(
    //     &conf,
    //     RefuelShipParams {
    //         ship_symbol: mining_ship_symbol.to_string(),
    //     },
    // )
    // .await?;
    // info!("Refuel results: {:?}", refuel_results);
    //
    // let orbit_results = orbit_ship(
    //     &conf,
    //     OrbitShipParams {
    //         ship_symbol: mining_ship_symbol.to_string(),
    //     },
    // )
    // .await?;
    // info!("Orbit results: {:?}", orbit_results);

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
    let nav_results = navigate_ship(
        &conf,
        NavigateShipParams {
            ship_symbol: mining_ship_symbol.to_string(),
            navigate_ship_request: Some(NavigateShipRequest {
                waypoint_symbol: asteroid_field_waypoint_symbol.to_string(),
            }),
        },
    )
    .await;

    match nav_results {
        Ok(nr) => {
            info!("Navigation results: {:?}", nr);

            let arrival = DateTime::parse_from_rfc3339(&nr.data.nav.route.arrival).unwrap();
            let arrival_seconds = arrival.signed_duration_since(Local::now());
            info!(
                "Awaiting arrival of ship ({} seconds)",
                arrival_seconds.num_seconds()
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(
                arrival_seconds.num_seconds() as u64,
            ))
            .await;
        }
        Err(e) => {
            let ei = get_error_info(e);
            // TODO: If error code is 4204 (navigateSameDestinationError) then continue
            if ei.code != 4204 && ei.code != 4214 {
                panic!(
                    "Navigate ship error ({}): {}: {:?}",
                    ei.code, ei.message, ei.data
                );
            }

            println!("{}: {:?}", ei.message, ei.data);

            // TOOD: If error code is 4214 (shipInTransitError) then wait and then continue
            if ei.code == 4214 {
                if let Value::Number(arrival_seconds) =
                    &ei.data.unwrap().get("secondsToArrival").unwrap()
                {
                    info!("Awaiting arrival of ship ({} seconds)", arrival_seconds);
                    tokio::time::sleep(tokio::time::Duration::from_secs(
                        arrival_seconds.as_u64().unwrap(),
                    ))
                    .await;
                }
            }
        }
    }

    'main: loop {
        let ship_result = get_my_ship(
            &conf,
            GetMyShipParams {
                ship_symbol: mining_ship_symbol.to_string(),
            },
        )
        .await;
        let ship = match ship_result {
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
            ship.data.cargo.capacity, ship.data.cargo.units
        );
        info!("Current ship cargo is {:?}", ship.data.cargo.inventory);

        for i in ship.data.cargo.inventory.clone() {
            if &i.symbol == "ALUMINUM_ORE" && i.units >= (ship.data.cargo.capacity - 10) {
                info!("Ship is full of {}", i.symbol);

                let deliver_goods = my_contract.data.terms.deliver.clone().unwrap();
                let deliver_good = deliver_goods.get(0).unwrap().clone();

                let dock_result = dock_ship(
                    &conf,
                    DockShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                    },
                )
                .await;
                match dock_result {
                    Ok(dr) => info!("Dock result: {:?}", dr),
                    Err(dre) => {
                        let ei = get_error_info(dre);
                        error!("Dock result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Refueling");
                let refueling_result = refuel_ship(
                    &conf,
                    RefuelShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                    },
                )
                .await;
                match refueling_result {
                    Ok(rr) => info!("Refueling result: {:?}", rr),
                    Err(rre) => {
                        let ei = get_error_info(rre);
                        error!("Refueling result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                let delivery_waypoint_symbol = deliver_good.destination_symbol;
                info!(
                    "Navigating to delivery waypoint {}",
                    delivery_waypoint_symbol
                );
                let navigation_results = navigate_ship(
                    &conf,
                    NavigateShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                        navigate_ship_request: Some(NavigateShipRequest {
                            waypoint_symbol: delivery_waypoint_symbol.to_string(),
                        }),
                    },
                )
                .await;
                match navigation_results {
                    Ok(nr) => {
                        info!("Navigation results: {:?}", nr);

                        let arrival =
                            DateTime::parse_from_rfc3339(&nr.data.nav.route.arrival).unwrap();
                        let arrival_seconds = arrival.signed_duration_since(Local::now());
                        info!(
                            "Awaiting arrival of ship ({} seconds)",
                            arrival_seconds.num_seconds()
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(
                            arrival_seconds.num_seconds() as u64,
                        ))
                        .await;
                    }
                    Err(nre) => {
                        let ei = get_error_info(nre);
                        // We can skip any 4204 errors since that just means we are already at the destination
                        if ei.code != 4204 {
                            error!("Navigation results error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }
                };

                let dock_result = dock_ship(
                    &conf,
                    DockShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                    },
                )
                .await;
                match dock_result {
                    Ok(dr) => info!("Dock result: {:?}", dr),
                    Err(dre) => {
                        let ei = get_error_info(dre);
                        error!("Dock result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Refueling");
                let refueling_result = refuel_ship(
                    &conf,
                    RefuelShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                    },
                )
                .await;
                match refueling_result {
                    Ok(rr) => info!("Refueling result: {:?}", rr),
                    Err(rre) => {
                        let ei = get_error_info(rre);
                        error!("Refueling result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!(
                    "Delivering {} of {} to {}",
                    i.units, i.symbol, delivery_waypoint_symbol
                );
                let delivery_result = deliver_contract(
                    &conf,
                    DeliverContractParams {
                        contract_id: contract_id.clone(),
                        deliver_contract_request: Some(DeliverContractRequest {
                            ship_symbol: mining_ship_symbol.to_string(),
                            trade_symbol: i.symbol,
                            units: i.units,
                        }),
                    },
                )
                .await;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                match delivery_result {
                    Ok(dr) => info!("Delivery result: {:?}", dr),
                    Err(dre) => {
                        let ei = get_error_info(dre);
                        error!("Delivery result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }

                let contract_results = get_contract(
                    &conf,
                    GetContractParams {
                        contract_id: contract_id.clone(),
                    },
                )
                .await;
                match contract_results {
                    Ok(cr) => {
                        info!("Updated contract results: {:?}", cr);
                        if !cr.data.fulfilled
                            && cr
                                .data
                                .terms
                                .deliver
                                .unwrap()
                                .iter()
                                .all(|i| i.units_fulfilled == i.units_required)
                        {
                            let fulfill_contract_results = fulfill_contract(
                                &conf,
                                FulfillContractParams {
                                    contract_id: contract_id.clone(),
                                },
                            )
                            .await;

                            match fulfill_contract_results {
                                Ok(fcr) => info!("Fulfill contract results: {:?}", fcr),
                                Err(fcre) => {
                                    let ei = get_error_info(fcre);
                                    error!(
                                        "Fulfill contract result error ({}): {}",
                                        ei.code, ei.message
                                    );
                                }
                            }

                            info!(
                                "Technically this script is done since the contract is fulfilled"
                            );
                            process::exit(0);
                        }
                    }
                    Err(cre) => {
                        let ei = get_error_info(cre);
                        error!("Get contract results error ({}): not waiting since this is a non-critical call: {}", ei.code, ei.message);
                    }
                }

                info!("Going back into orbit");
                let orbit_result = orbit_ship(
                    &conf,
                    OrbitShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                    },
                )
                .await;
                match orbit_result {
                    Ok(or) => info!("Orbit result: {:?}", or),
                    Err(ore) => {
                        let ei = get_error_info(ore);
                        error!("Orbit result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Returning to asteroid field");
                let navigation_results = navigate_ship(
                    &conf,
                    NavigateShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                        navigate_ship_request: Some(NavigateShipRequest {
                            waypoint_symbol: asteroid_field_waypoint_symbol.to_string(),
                        }),
                    },
                )
                .await;
                match navigation_results {
                    Ok(nr) => {
                        info!("Navigation results: {:?}", nr);

                        let arrival =
                            DateTime::parse_from_rfc3339(&nr.data.nav.route.arrival).unwrap();
                        let arrival_seconds = arrival.signed_duration_since(Local::now());
                        info!(
                            "Awaiting arrival of ship ({} seconds)",
                            arrival_seconds.num_seconds()
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(
                            arrival_seconds.num_seconds() as u64,
                        ))
                        .await;
                    }
                    Err(nre) => {
                        let ei = get_error_info(nre);
                        // We can skip any 4204 errors since that just means we are already at the destination
                        if ei.code != 4204 {
                            error!("Navigation results error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }
                };

                info!("Docking ship to refuel");
                let docking_result = dock_ship(
                    &conf,
                    DockShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                    },
                )
                .await;
                match docking_result {
                    Ok(dr) => info!("Docking Result: {:?}", dr),
                    Err(dre) => {
                        let ei = get_error_info(dre);
                        error!("Docking results error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Refueling");
                let refueling_result = refuel_ship(
                    &conf,
                    RefuelShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                    },
                )
                .await;
                match refueling_result {
                    Ok(rr) => info!("Refueling result: {:?}", rr),
                    Err(rre) => {
                        let ei = get_error_info(rre);
                        error!("Refueling result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("Going back into orbit");
                let orbit_result = orbit_ship(
                    &conf,
                    OrbitShipParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                    },
                )
                .await;
                match orbit_result {
                    Ok(or) => info!("Orbit result: {:?}", or),
                    Err(ore) => {
                        let ei = get_error_info(ore);
                        error!("Orbit result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue 'main;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }

        if ship.data.cargo.units == ship.data.cargo.capacity {
            info!("Docking");
            let dock_results = dock_ship(
                &conf,
                DockShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                },
            )
            .await?;
            info!("Dock results: {:?}", dock_results);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            for i in ship.data.cargo.inventory.clone() {
                if &i.symbol != "ALUMINUM_ORE" {
                    info!("Selling {} of {}", i.units, i.symbol);
                    let sell_results = sell_cargo(
                        &conf,
                        SellCargoParams {
                            ship_symbol: mining_ship_symbol.to_string(),
                            sell_cargo_request: Some(SellCargoRequest {
                                symbol: i.symbol,
                                units: i.units,
                            }),
                        },
                    )
                    .await;
                    match sell_results {
                        Ok(sr) => info!("Sell results: {:?}", sr),
                        Err(sre) => {
                            let ei = get_error_info(sre);
                            error!("Sell results error({}): waiting for 10 seconds and restarting loop {}", ei.code, ei.message);
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            continue 'main;
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }

            info!("Going into orbit");
            let orbit_results = orbit_ship(
                &conf,
                OrbitShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                },
            )
            .await;
            match orbit_results {
                Ok(or) => info!("Orbit results: {:?}", or),
                Err(ore) => {
                    let ei = get_error_info(ore);
                    error!(
                        "Orbit result error ({}), waiting 10 seconds and starting loop again: {}",
                        ei.code, ei.message
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    continue 'main;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        info!("Extracting");
        let extract_results = extract_resources(
            &conf,
            ExtractResourcesParams {
                ship_symbol: mining_ship_symbol.to_string(),
                extract_resources_request: None,
            },
        )
        .await;
        match extract_results {
            Ok(er) => {
                info!("Extract results: {:?}", er);
                info!(
                    "Cooling down for {} seconds",
                    er.data.cooldown.remaining_seconds
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    er.data.cooldown.remaining_seconds as u64,
                ))
                .await;
                info!("Cool down over");
            }
            Err(ere) => {
                let ei = get_error_info(ere);
                error!(
                    "Extract results error ({}), waiting 10 seconds and starting loop again: {}",
                    ei.code, ei.message
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue 'main;
            }
        }
    }
}
