#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{future, Stream, stream};
#[allow(unused_imports)]
use spacetraders_sdk::{Api, ApiNoContext, Client, ContextWrapperExt, models,
                      GetMyAgentResponse,
                      AcceptContractResponse,
                      DeliverContractResponse,
                      FulfillContractResponse,
                      GetContractResponse,
                      GetContractsResponse,
                      RegisterResponse,
                      GetFactionResponse,
                      GetFactionsResponse,
                      CreateChartResponse,
                      CreateShipShipScanResponse,
                      CreateShipSystemScanResponse,
                      CreateShipWaypointScanResponse,
                      CreateSurveyResponse,
                      DockShipResponse,
                      ExtractResourcesResponse,
                      GetMyShipResponse,
                      GetMyShipCargoResponse,
                      GetMyShipsResponse,
                      GetShipCooldownResponse,
                      GetShipNavResponse,
                      JettisonResponse,
                      JumpShipResponse,
                      NavigateShipResponse,
                      OrbitShipResponse,
                      PatchShipNavResponse,
                      PurchaseCargoResponse,
                      PurchaseShipResponse,
                      RefuelShipResponse,
                      SellCargoResponse,
                      ShipRefineResponse,
                      TransferCargoResponse,
                      WarpShipResponse,
                      GetJumpGateResponse,
                      GetMarketResponse,
                      GetShipyardResponse,
                      GetSystemResponse,
                      GetSystemWaypointsResponse,
                      GetSystemsResponse,
                      GetWaypointResponse,
                     };
use clap::{App, Arg};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "GetMyAgent",
                "AcceptContract",
                "DeliverContract",
                "FulfillContract",
                "GetContract",
                "GetContracts",
                "Register",
                "GetFaction",
                "GetFactions",
                "CreateChart",
                "CreateShipShipScan",
                "CreateShipSystemScan",
                "CreateShipWaypointScan",
                "CreateSurvey",
                "DockShip",
                "ExtractResources",
                "GetMyShip",
                "GetMyShipCargo",
                "GetMyShips",
                "GetShipCooldown",
                "GetShipNav",
                "Jettison",
                "JumpShip",
                "NavigateShip",
                "OrbitShip",
                "PatchShipNav",
                "PurchaseCargo",
                "PurchaseShip",
                "RefuelShip",
                "SellCargo",
                "ShipRefine",
                "TransferCargo",
                "WarpShip",
                "GetJumpGate",
                "GetMarket",
                "GetShipyard",
                "GetSystem",
                "GetSystemWaypoints",
                "GetSystems",
                "GetWaypoint",
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("api.spacetraders.io")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let context: ClientContext =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let mut client : Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client = Box::new(Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client = Box::new(Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("GetMyAgent") => {
            let result = rt.block_on(client.get_my_agent(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AcceptContract") => {
            let result = rt.block_on(client.accept_contract(
                  "contract_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeliverContract") => {
            let result = rt.block_on(client.deliver_contract(
                  "contract_id_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("FulfillContract") => {
            let result = rt.block_on(client.fulfill_contract(
                  "contract_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetContract") => {
            let result = rt.block_on(client.get_contract(
                  "contract_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetContracts") => {
            let result = rt.block_on(client.get_contracts(
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("Register") => {
            let result = rt.block_on(client.register(
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetFaction") => {
            let result = rt.block_on(client.get_faction(
                  "faction_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetFactions") => {
            let result = rt.block_on(client.get_factions(
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateChart") => {
            let result = rt.block_on(client.create_chart(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateShipShipScan") => {
            let result = rt.block_on(client.create_ship_ship_scan(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateShipSystemScan") => {
            let result = rt.block_on(client.create_ship_system_scan(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateShipWaypointScan") => {
            let result = rt.block_on(client.create_ship_waypoint_scan(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateSurvey") => {
            let result = rt.block_on(client.create_survey(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DockShip") => {
            let result = rt.block_on(client.dock_ship(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ExtractResources") => {
            let result = rt.block_on(client.extract_resources(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetMyShip") => {
            let result = rt.block_on(client.get_my_ship(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetMyShipCargo") => {
            let result = rt.block_on(client.get_my_ship_cargo(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetMyShips") => {
            let result = rt.block_on(client.get_my_ships(
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetShipCooldown") => {
            let result = rt.block_on(client.get_ship_cooldown(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetShipNav") => {
            let result = rt.block_on(client.get_ship_nav(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("Jettison") => {
            let result = rt.block_on(client.jettison(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("JumpShip") => {
            let result = rt.block_on(client.jump_ship(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("NavigateShip") => {
            let result = rt.block_on(client.navigate_ship(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("OrbitShip") => {
            let result = rt.block_on(client.orbit_ship(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("PatchShipNav") => {
            let result = rt.block_on(client.patch_ship_nav(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("PurchaseCargo") => {
            let result = rt.block_on(client.purchase_cargo(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("PurchaseShip") => {
            let result = rt.block_on(client.purchase_ship(
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("RefuelShip") => {
            let result = rt.block_on(client.refuel_ship(
                  "ship_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SellCargo") => {
            let result = rt.block_on(client.sell_cargo(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ShipRefine") => {
            let result = rt.block_on(client.ship_refine(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("TransferCargo") => {
            let result = rt.block_on(client.transfer_cargo(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("WarpShip") => {
            let result = rt.block_on(client.warp_ship(
                  "ship_symbol_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetJumpGate") => {
            let result = rt.block_on(client.get_jump_gate(
                  "system_symbol_example".to_string(),
                  "waypoint_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetMarket") => {
            let result = rt.block_on(client.get_market(
                  "system_symbol_example".to_string(),
                  "waypoint_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetShipyard") => {
            let result = rt.block_on(client.get_shipyard(
                  "system_symbol_example".to_string(),
                  "waypoint_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSystem") => {
            let result = rt.block_on(client.get_system(
                  "system_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSystemWaypoints") => {
            let result = rt.block_on(client.get_system_waypoints(
                  "system_symbol_example".to_string(),
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSystems") => {
            let result = rt.block_on(client.get_systems(
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWaypoint") => {
            let result = rt.block_on(client.get_waypoint(
                  "system_symbol_example".to_string(),
                  "waypoint_symbol_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
