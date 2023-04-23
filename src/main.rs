use spacetraders_sdk::apis::agents_api::get_my_agent;
use spacetraders_sdk::apis::configuration::Configuration;
use spacetraders_sdk::apis::contracts_api::{
    accept_contract, get_contracts, AcceptContractError, AcceptContractParams, GetContractsParams,
};
use spacetraders_sdk::apis::factions_api::{get_faction, GetFactionParams};
use spacetraders_sdk::apis::fleet_api::{get_my_ships, GetMyShipsParams};
use spacetraders_sdk::apis::systems_api::{get_waypoint, GetWaypointParams};
use spacetraders_sdk::apis::ResponseContent;

// const blove_account_id = "clgq3deon3qs5s60d7y09d7gg";
const BASE_URL: &str = "https://api.spacetraders.io";
const ACCOUNT_SYMBOL: &str = "BLOVE";
const FACTION: &str = "COSMIC";
const CONTRACT_ID: &str = "clgq3der73qs8s60dvfrf05yj";
const BLOVE_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiQkxPVkUiLCJpYXQiOjE2ODIwNTM3NDgsInN1YiI6ImFnZW50LXRva2VuIn0.NJcz9nRlLFkilnwRZQ4YR-LHQPFNhaqRmoYAgY1GYXuLhbux7rjVirFIj4jZlrugwn5yzLiNufXmBSQjKOmx8B5Mf0stOYuD9mYGdrZy_Gv9VsGBfX896_Jm2y33Nr35wzTGvkfDz32rnFReb1YDzI7AtbRpvlfbS7J6pLjESmR7lAwiS_4k_9LhLh2qOh5JVM1gWONzqN1z9domdICRVXxIOTaC8EwujtjOVlRJMPiCiD98hwlwar43ipQMQC1b5jOBTenZgKPpC1T6k2nMXmb0ABKl2PzTetC2m53t8qzahMOJaIYtZWBA3ljKpXM20EWUeylIj86dv4Lww4kuiARmS-AX5C6KM0iT9ER6uYK16MfUbZhtnzidH7DpAC0oHm-OZk1-SqLhX56Hf4eMEUAJRryZ_i-MoMGAE8g01W4iT1t6WrYtQlG7IkdiU0GgTPNYNDkBJwpE5bddhL2dOFiYigNaXVR6MXFUFyUlMIZZ37UWK_-R59Y1roaYA0JQ";

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

    let my_contracts = get_contracts(
        &conf,
        GetContractsParams {
            page: None,
            limit: None,
        },
    )
    .await?;
    println!("My contracts {:#?}", my_contracts);

    let my_faction = get_faction(
        &conf,
        GetFactionParams {
            faction_symbol: FACTION.to_string(),
        },
    )
    .await?;
    println!("My faction {:#?}", my_faction);

    let my_ships = get_my_ships(
        &conf,
        GetMyShipsParams {
            page: None,
            limit: None,
        },
    )
    .await?;
    println!("My ships {:#?}", my_ships);

    let starting_waypoint = get_waypoint(
        &conf,
        GetWaypointParams {
            system_symbol: "X1-DF55".to_string(),
            waypoint_symbol: "X1-DF55-20250Z".to_string(),
        },
    )
    .await?;
    println!("Starting waypoint: {:#?}", starting_waypoint);

    let accepted_contract = accept_contract(
        &conf,
        AcceptContractParams {
            contract_id: CONTRACT_ID.to_owned(),
        },
    )
    .await;

    match accepted_contract {
        Ok(ac) => println!("Accepted Contract: {:#?}", ac),
        Err(e) => match e {
            spacetraders_sdk::apis::Error::ResponseError::<AcceptContractError>(ace) => {
                let AcceptContractError::UnknownValue(v) = ace.entity.unwrap();
                println!(
                    "error status: {}, error entity: {}",
                    ace.status,
                    v.get("error").unwrap().get("message").unwrap()
                );
            }
            _ => println!("unknown error"),
        },
    }

    fn get_error_message<T>(e: spacetraders_sdk::apis::Error<T>) -> String {
        match e {
            spacetraders_sdk::apis::Error::ResponseError::<T>(ev) => {
                let <T>::UnknownValue(v) = ev.entity.unwrap();
                format!(
                    "error status: {}, error entity: {}",
                    v.status,
                    v.get("error").unwrap().get("message").unwrap()
                )
            }
            _ => format!("unknown error"),
        }
    }

    match accepted_contract {
        Ok(ac) => println!("Accepted Contract: {:#?}", ac),
        Err(e) => println!("Accept contract failed: {}", get_error_message<AcceptContractError>()),
    }

    // match accepted_contract {
    //     Ok(ac) => println!("Accepted Contract: {:#?}", ac),
    //     Err(e) => match e {
    //         spacetraders_sdk::apis::Error::ResponseError::<AcceptContractError>(ace) => {
    //             let AcceptContractError::UnknownValue(v) = ace.entity.unwrap();
    //             println!(
    //                 "error status: {}, error entity: {}",
    //                 ace.status,
    //                 v.get("error").unwrap().get("message").unwrap()
    //             );
    //         }
    //         _ => println!("unknown error"),
    //     },
    // }

    Ok(())
}
