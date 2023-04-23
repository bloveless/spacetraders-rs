use spacetraders_sdk::apis::agents_api::get_my_agent;
use spacetraders_sdk::apis::configuration::Configuration;
use spacetraders_sdk::apis::contracts_api::{accept_contract, get_contracts, AcceptContractParams, GetContractsParams, get_contract, GetContractParams};
use spacetraders_sdk::apis::factions_api::{get_faction, GetFactionParams};
use spacetraders_sdk::apis::fleet_api::{dock_ship, DockShipParams, extract_resources, ExtractResourcesParams, get_my_ships, GetMyShipsParams, navigate_ship, NavigateShipParams, orbit_ship, OrbitShipParams, purchase_ship, PurchaseShipParams, refuel_ship, RefuelShipParams};
use spacetraders_sdk::apis::systems_api::{get_shipyard, get_system_waypoints, get_waypoint, GetShipyardParams, GetSystemWaypointsParams, GetWaypointParams};
use spacetraders_sdk::apis::{Error, ResponseContent};
use spacetraders_sdk::models::waypoint_trait::Symbol;
use spacetraders_sdk::models::{ExtractResourcesRequest, NavigateShipRequest, PurchaseShipRequest, ShipType, Survey, WaypointTrait};

// const blove_account_id = "clgq3deon3qs5s60d7y09d7gg";
const BASE_URL: &str = "https://api.spacetraders.io";
const ACCOUNT_SYMBOL: &str = "BLOVE";
const FACTION: &str = "COSMIC";
const CONTRACT_ID: &str = "clgq3der73qs8s60dvfrf05yj";
const BLOVE_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiQkxPVkUiLCJpYXQiOjE2ODIwNTM3NDgsInN1YiI6ImFnZW50LXRva2VuIn0.NJcz9nRlLFkilnwRZQ4YR-LHQPFNhaqRmoYAgY1GYXuLhbux7rjVirFIj4jZlrugwn5yzLiNufXmBSQjKOmx8B5Mf0stOYuD9mYGdrZy_Gv9VsGBfX896_Jm2y33Nr35wzTGvkfDz32rnFReb1YDzI7AtbRpvlfbS7J6pLjESmR7lAwiS_4k_9LhLh2qOh5JVM1gWONzqN1z9domdICRVXxIOTaC8EwujtjOVlRJMPiCiD98hwlwar43ipQMQC1b5jOBTenZgKPpC1T6k2nMXmb0ABKl2PzTetC2m53t8qzahMOJaIYtZWBA3ljKpXM20EWUeylIj86dv4Lww4kuiARmS-AX5C6KM0iT9ER6uYK16MfUbZhtnzidH7DpAC0oHm-OZk1-SqLhX56Hf4eMEUAJRryZ_i-MoMGAE8g01W4iT1t6WrYtQlG7IkdiU0GgTPNYNDkBJwpE5bddhL2dOFiYigNaXVR6MXFUFyUlMIZZ37UWK_-R59Y1roaYA0JQ";

fn get_error_message(e: Error) -> String {
    match e {
        Error::ResponseError(re) => {
            format!(
                "error status: {}, error message: {}",
                re.status,
                re.entity.unwrap().error.message,
            )
        },
        _ => format!("unknown error"),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conf = Configuration {
        bearer_access_token: Some(BLOVE_TOKEN.to_string()),
        ..Default::default()
    };
    // let register_response = register(
    //     &conf,
    //     Some(RegisterRequest::new(Faction::Cosmic, "blove".to_string()))
    // ).await?;

    // println!("Register Response: {:?}", register_response);

    let my_agent = get_my_agent(&conf).await?;
    println!("My agent: {:#?}", my_agent);

    // let my_contracts = get_contracts(
    //     &conf,
    //     GetContractsParams {
    //         page: None,
    //         limit: None,
    //     },
    // )
    // .await?;
    // println!("My contracts {:#?}", my_contracts);

    // let my_faction = get_faction(
    //     &conf,
    //     GetFactionParams {
    //         faction_symbol: FACTION.to_string(),
    //     },
    // )
    // .await?;
    // println!("My faction {:#?}", my_faction);

    // let my_ships = get_my_ships(
    //     &conf,
    //     GetMyShipsParams {
    //         page: None,
    //         limit: None,
    //     },
    // )
    // .await?;
    // println!("My ships {:#?}", my_ships);

    // let starting_waypoint = get_waypoint(
    //     &conf,
    //     GetWaypointParams {
    //         system_symbol: "X1-DF55".to_string(),
    //         waypoint_symbol: "X1-DF55-20250Z".to_string(),
    //     },
    // )
    // .await?;
    // println!("Starting waypoint: {:#?}", starting_waypoint);

    // let accepted_contract = accept_contract(
    //     &conf,
    //     AcceptContractParams {
    //         contract_id: CONTRACT_ID.to_owned(),
    //     },
    // )
    // .await;
    //
    // match accepted_contract {
    //     Ok(ac) => println!("Accepted Contract: {:#?}", ac),
    //     Err(e) => println!("Accept contract failed: {}", get_error_message(e)),
    // }

    let my_contract = get_contract(&conf, GetContractParams { contract_id: CONTRACT_ID.to_string() }).await?;
    println!("My Contract: {:#?}", my_contract);

    // Find a shipyard
    // let waypoints = get_system_waypoints(&conf, GetSystemWaypointsParams {
    //     system_symbol: "X1-DF55".to_string(),
    //     ..Default::default()
    // }).await?;
    // println!("Waypoints: {:#?}", waypoints);

    // let shipyard = waypoints.data.into_iter().find(|w| {
    //     w.traits.clone().into_iter().find(|t| t.symbol == Symbol::Shipyard).is_some()
    // });
    //
    // let shipyard = match shipyard {
    //     Some(s) => s,
    //     None => panic!("unable to find available shipyard")
    // };
    //
    // println!("First shipyard: {:?}", shipyard);

    let shipyard_waypoint_symbol = "X1-DF55-69207D";
    let shipyard_system_symbol = "X1-DF55";

    // let shipyard_data = get_shipyard(&conf, GetShipyardParams {
    //     system_symbol: shipyard_system_symbol.to_string(),
    //     waypoint_symbol: shipyard_waypoint_symbol.to_string(),
    // }).await;
    //
    // println!("Shipyard data: {:?}", shipyard_data);

    // Purchase a ship
    // let ship = purchase_ship(&conf, PurchaseShipParams {
    //     purchase_ship_request: Some(PurchaseShipRequest {
    //         waypoint_symbol: shipyard_waypoint_symbol.to_string(),
    //         ship_type: ShipType::MiningDrone,
    //     })
    // }).await?;
    // println!("Purchased ship: {:?}", ship);

    let asteroid_field_waypoint_symbol = "X1-DF55-69207D";
    let mining_ship_symbol = "BLOVE-2";

    // let nav_results = navigate_ship(&conf, NavigateShipParams {
    //     ship_symbol: mining_ship_symbol.to_string(),
    //     navigate_ship_request: Some(NavigateShipRequest {
    //         waypoint_symbol: asteroid_field_waypoint_symbol.to_string(),
    //     }),
    // }).await;
    //
    // match nav_results {
    //     Ok(nav) => println!("Nav results: {:?}", nav),
    //     Err(e) => println!("Nav results error: {:?}", get_error_message(e)),
    // }

    // let dock_results = dock_ship(&conf, DockShipParams {
    //     ship_symbol: mining_ship_symbol.to_string(),
    // }).await?;
    // println!("Dock results: {:?}", dock_results);

    // let refuel_results = refuel_ship(&conf, RefuelShipParams {
    //     ship_symbol: mining_ship_symbol.to_string(),
    // }).await?;
    // println!("Refuel results: {:?}", refuel_results);

    // Orbit ship
    // let orbit_results = orbit_ship(&conf, OrbitShipParams {
    //     ship_symbol: mining_ship_symbol.to_string(),
    // }).await?;
    // println!("Orbit results: {:?}", orbit_results);

    let extract_results = extract_resources(&conf, ExtractResourcesParams {
        ship_symbol: mining_ship_symbol.to_string(),
        extract_resources_request: None,
    }).await;
    match extract_results {
        Ok(er) => println!("Extract results: {:?}", er),
        Err(e) => println!("Extract error: {:?}", get_error_message(e)),
    }

    Ok(())
}
