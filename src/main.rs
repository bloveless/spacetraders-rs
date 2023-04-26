#[macro_use]
extern crate log;

use std::process;
use chrono::{DateTime, Local};
use env_logger::{Env, Target};
use spacetraders_sdk::apis::agents_api::get_my_agent;
use spacetraders_sdk::apis::configuration::Configuration;
use spacetraders_sdk::apis::contracts_api::{accept_contract, get_contracts, AcceptContractParams, GetContractsParams, get_contract, GetContractParams, deliver_contract, DeliverContractParams, fulfill_contract, FulfillContractParams};
use spacetraders_sdk::apis::factions_api::{get_faction, GetFactionParams};
use spacetraders_sdk::apis::fleet_api::{dock_ship, DockShipParams, extract_resources, ExtractResourcesParams, get_my_ship, get_my_ships, GetMyShipParams, GetMyShipsParams, navigate_ship, NavigateShipParams, orbit_ship, OrbitShipParams, purchase_ship, PurchaseShipParams, refuel_ship, RefuelShipParams, sell_cargo, SellCargoParams};
use spacetraders_sdk::apis::systems_api::{get_shipyard, get_system_waypoints, get_waypoint, GetShipyardParams, GetSystemWaypointsParams, GetWaypointParams};
use spacetraders_sdk::apis::{Error, ResponseContent};
use spacetraders_sdk::models::waypoint_trait::Symbol;
use spacetraders_sdk::models::{DeliverContractRequest, ExtractResourcesRequest, NavigateShipRequest, PurchaseShipRequest, SellCargoRequest, ShipCargoItem, ShipType, Survey, WaypointTrait};

// const blove_account_id = "clgq3deon3qs5s60d7y09d7gg";
const BASE_URL: &str = "https://api.spacetraders.io";
const ACCOUNT_SYMBOL: &str = "BLOVE";
const FACTION: &str = "COSMIC";
const CONTRACT_ID: &str = "clgq3der73qs8s60dvfrf05yj";
const BLOVE_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiQkxPVkUiLCJpYXQiOjE2ODIwNTM3NDgsInN1YiI6ImFnZW50LXRva2VuIn0.NJcz9nRlLFkilnwRZQ4YR-LHQPFNhaqRmoYAgY1GYXuLhbux7rjVirFIj4jZlrugwn5yzLiNufXmBSQjKOmx8B5Mf0stOYuD9mYGdrZy_Gv9VsGBfX896_Jm2y33Nr35wzTGvkfDz32rnFReb1YDzI7AtbRpvlfbS7J6pLjESmR7lAwiS_4k_9LhLh2qOh5JVM1gWONzqN1z9domdICRVXxIOTaC8EwujtjOVlRJMPiCiD98hwlwar43ipQMQC1b5jOBTenZgKPpC1T6k2nMXmb0ABKl2PzTetC2m53t8qzahMOJaIYtZWBA3ljKpXM20EWUeylIj86dv4Lww4kuiARmS-AX5C6KM0iT9ER6uYK16MfUbZhtnzidH7DpAC0oHm-OZk1-SqLhX56Hf4eMEUAJRryZ_i-MoMGAE8g01W4iT1t6WrYtQlG7IkdiU0GgTPNYNDkBJwpE5bddhL2dOFiYigNaXVR6MXFUFyUlMIZZ37UWK_-R59Y1roaYA0JQ";

struct ErrorInfo {
    code: u32,
    message: String,
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
            }
        }
        _ => ErrorInfo { code: 0, message: format!("unknown error") },
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::default()
        .filter_or("RUST_LOG", "spacetraders_rs=trace")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::Builder::from_env(env).target(Target::Stdout).init();

    let conf = Configuration {
        bearer_access_token: Some(BLOVE_TOKEN.to_string()),
        ..Default::default()
    };

    // let register_response = register(
    //     &conf,
    //     Some(RegisterRequest::new(Faction::Cosmic, "blove".to_string()))
    // ).await?;

    // info!("Register Response: {:?}", register_response);

    let my_agent = get_my_agent(&conf).await?;
    info!("My agent: {:?}", my_agent);

    // let my_contracts = get_contracts(
    //     &conf,
    //     GetContractsParams {
    //         page: None,
    //         limit: None,
    //     },
    // )
    // .await?;
    // info!("My contracts {:?}", my_contracts);

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

    // let starting_waypoint = get_waypoint(
    //     &conf,
    //     GetWaypointParams {
    //         system_symbol: "X1-DF55".to_string(),
    //         waypoint_symbol: "X1-DF55-20250Z".to_string(),
    //     },
    // )
    // .await?;
    // info!("Starting waypoint: {:?}", starting_waypoint);

    // let accepted_contract = accept_contract(
    //     &conf,
    //     AcceptContractParams {
    //         contract_id: CONTRACT_ID.to_owned(),
    //     },
    // )
    // .await;
    //
    // match accepted_contract {
    //     Ok(ac) => info!("Accepted Contract: {:?}", ac),
    //     Err(e) => error!("Accept contract failed: {}", get_error_message(e)),
    // }

    let my_contract = get_contract(&conf, GetContractParams { contract_id: CONTRACT_ID.to_string() }).await?;
    info!("My Contract: {:?}", my_contract);

    // Find a shipyard
    // let waypoints = get_system_waypoints(&conf, GetSystemWaypointsParams {
    //     system_symbol: "X1-DF55".to_string(),
    //     ..Default::default()
    // }).await?;
    // info!("Waypoints: {:?}", waypoints);

    // let shipyard = waypoints.data.into_iter().find(|w| {
    //     w.traits.clone().into_iter().find(|t| t.symbol == Symbol::Shipyard).is_some()
    // });
    //
    // let shipyard = match shipyard {
    //     Some(s) => s,
    //     None => error!("unable to find available shipyard")
    // };
    //
    // info!("First shipyard: {:?}", shipyard);

    let shipyard_waypoint_symbol = "X1-DF55-69207D";
    let shipyard_system_symbol = "X1-DF55";

    // let shipyard_data = get_shipyard(&conf, GetShipyardParams {
    //     system_symbol: shipyard_system_symbol.to_string(),
    //     waypoint_symbol: shipyard_waypoint_symbol.to_string(),
    // }).await;
    //
    // info!("Shipyard data: {:?}", shipyard_data);

    // Purchase a ship
    // let ship = purchase_ship(&conf, PurchaseShipParams {
    //     purchase_ship_request: Some(PurchaseShipRequest {
    //         waypoint_symbol: shipyard_waypoint_symbol.to_string(),
    //         ship_type: ShipType::MiningDrone,
    //     })
    // }).await?;
    // info!("Purchased ship: {:?}", ship);

    let asteroid_field_waypoint_symbol = "X1-DF55-17335A";
    let mining_ship_symbol = "BLOVE-2";

    // let nav_results = navigate_ship(&conf, NavigateShipParams {
    //     ship_symbol: mining_ship_symbol.to_string(),
    //     navigate_ship_request: Some(NavigateShipRequest {
    //         waypoint_symbol: asteroid_field_waypoint_symbol.to_string(),
    //     }),
    // }).await;
    //
    // match nav_results {
    //     Ok(nav) => info!("Nav results: {:?}", nav),
    //     Err(e) => error!("Nav results error: {:?}", get_error_message(e)),
    // }

    // let dock_results = dock_ship(&conf, DockShipParams {
    //     ship_symbol: mining_ship_symbol.to_string(),
    // }).await?;
    // info!("Dock results: {:?}", dock_results);

    // let refuel_results = refuel_ship(&conf, RefuelShipParams {
    //     ship_symbol: mining_ship_symbol.to_string(),
    // }).await?;
    // info!("Refuel results: {:?}", refuel_results);

    // Orbit ship
    // let orbit_results = orbit_ship(&conf, OrbitShipParams {
    //     ship_symbol: mining_ship_symbol.to_string(),
    // }).await?;
    // info!("Orbit results: {:?}", orbit_results);

    'main: loop {
        let ship_result = get_my_ship(&conf, GetMyShipParams {
            ship_symbol: mining_ship_symbol.to_string(),
        }).await;
        let ship = match ship_result {
            Ok(sr) => sr,
            Err(sre) => {
                error!("Error getting ship. Waiting for 10 seconds and trying again: {:?}", sre);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            }
        };

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        info!("Current ship capacity is {} and current cargo units is {}", ship.data.cargo.capacity, ship.data.cargo.units);
        info!("Current ship cargo is {:?}", ship.data.cargo.inventory);

        for i in ship.data.cargo.inventory.clone() {
            if &i.symbol == "ALUMINUM_ORE" && i.units >= (ship.data.cargo.capacity - 10) {
                info!("Ship is full of {}", i.symbol);

                let deliver_goods = my_contract.data.terms.deliver.clone().unwrap();
                let deliver_good = deliver_goods.get(0).unwrap().clone();

                let dock_result = dock_ship(&conf, DockShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                }).await;
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
                let refueling_result = refuel_ship(&conf, RefuelShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                }).await;
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
                info!("Navigating to delivery waypoint {}", delivery_waypoint_symbol);
                let navigation_results = navigate_ship(&conf, NavigateShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                    navigate_ship_request: Some(NavigateShipRequest {
                        waypoint_symbol: delivery_waypoint_symbol.to_string()
                    }),
                }).await;
                match navigation_results {
                    Ok(nr) => {
                        info!("Navigation results: {:?}", nr);

                        let arrival = DateTime::parse_from_rfc3339(&nr.data.nav.route.arrival).unwrap();
                        let arrival_seconds = arrival.signed_duration_since(Local::now());
                        info!("Awaiting arrival of ship ({} seconds)", arrival_seconds.num_seconds());
                        tokio::time::sleep(tokio::time::Duration::from_secs(arrival_seconds.num_seconds() as u64)).await;
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

                let dock_result = dock_ship(&conf, DockShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                }).await;
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
                let refueling_result = refuel_ship(&conf, RefuelShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                }).await;
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

                info!("Delivering {} of {} to {}", i.units, i.symbol, delivery_waypoint_symbol);
                let delivery_result = deliver_contract(&conf, DeliverContractParams {
                    contract_id: CONTRACT_ID.to_string(),
                    deliver_contract_request: Some(DeliverContractRequest {
                        ship_symbol: mining_ship_symbol.to_string(),
                        trade_symbol: i.symbol,
                        units: i.units,
                    }),
                }).await;
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

                let contract_results = get_contract(&conf, GetContractParams { contract_id: CONTRACT_ID.to_string() }).await;
                match contract_results {
                    Ok(cr) => {
                        info!("Updated contract results: {:?}", cr);
                        if !cr.data.fulfilled && cr.data.terms.deliver.unwrap().iter().all(|i| i.units_fulfilled == i.units_required) {
                            let fulfill_contract_results = fulfill_contract(&conf, FulfillContractParams {
                                contract_id: CONTRACT_ID.to_string(),
                            }).await;

                            match fulfill_contract_results {
                                Ok(fcr) => info!("Fulfill contract results: {:?}", fcr),
                                Err(fcre) => {
                                    let ei = get_error_info(fcre);
                                    error!("Fulfill contract result error ({}): {}", ei.code, ei.message);
                                }
                            }

                            info!("Technically this script is done since the contract is fulfilled");
                            process::exit(0);
                        }
                    },
                    Err(cre) => {
                        let ei = get_error_info(cre);
                        error!("Get contract results error ({}): not waiting since this is a non-critical call: {}", ei.code, ei.message);
                    }
                }

                info!("Going back into orbit");
                let orbit_result = orbit_ship(&conf, OrbitShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                }).await;
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
                let navigation_results = navigate_ship(&conf, NavigateShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                    navigate_ship_request: Some(NavigateShipRequest {
                        waypoint_symbol: asteroid_field_waypoint_symbol.to_string(),
                    }),
                }).await;
                match navigation_results {
                    Ok(nr) => {
                        info!("Navigation results: {:?}", nr);

                        let arrival = DateTime::parse_from_rfc3339(&nr.data.nav.route.arrival).unwrap();
                        let arrival_seconds = arrival.signed_duration_since(Local::now());
                        info!("Awaiting arrival of ship ({} seconds)", arrival_seconds.num_seconds());
                        tokio::time::sleep(tokio::time::Duration::from_secs(arrival_seconds.num_seconds() as u64)).await;
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
                let docking_result = dock_ship(&conf, DockShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                }).await;
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
                let refueling_result = refuel_ship(&conf, RefuelShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                }).await;
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
                let orbit_result = orbit_ship(&conf, OrbitShipParams {
                    ship_symbol: mining_ship_symbol.to_string(),
                }).await;
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
            let dock_results = dock_ship(&conf, DockShipParams {
                ship_symbol: mining_ship_symbol.to_string(),
            }).await?;
            info!("Dock results: {:?}", dock_results);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            for i in ship.data.cargo.inventory.clone() {
                if &i.symbol != "ALUMINUM_ORE" {
                    info!("Selling {} of {}", i.units, i.symbol);
                    let sell_results = sell_cargo(&conf, SellCargoParams {
                        ship_symbol: mining_ship_symbol.to_string(),
                        sell_cargo_request: Some(SellCargoRequest {
                            symbol: i.symbol,
                            units: i.units,
                        }),
                    }).await;
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
            let orbit_results = orbit_ship(&conf, OrbitShipParams {
                ship_symbol: mining_ship_symbol.to_string(),
            }).await;
            match orbit_results {
                Ok(or) => info!("Orbit results: {:?}", or),
                Err(ore) => {
                    let ei = get_error_info(ore);
                    error!("Orbit result error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    continue 'main;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        info!("Extracting");
        let extract_results = extract_resources(&conf, ExtractResourcesParams {
            ship_symbol: mining_ship_symbol.to_string(),
            extract_resources_request: None,
        }).await;
        match extract_results {
            Ok(er) => {
                info!("Extract results: {:?}", er);
                info!("Cooling down for {} seconds", er.data.cooldown.remaining_seconds);
                tokio::time::sleep(tokio::time::Duration::from_secs(er.data.cooldown.remaining_seconds as u64)).await;
                info!("Cool down over");
            }
            Err(ere) => {
                let ei = get_error_info(ere);
                error!("Extract results error ({}), waiting 10 seconds and starting loop again: {}", ei.code, ei.message);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue 'main;
            }
        }
    }
}
