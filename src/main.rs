// const blove_account_id = "clgq3deon3qs5s60d7y09d7gg";
const ACCOUNT_SYMBOL: &str = "BLOVE";
const FACTION: &str = "COSMIC";
const CONTRACT_ID: &str = "clgq3der73qs8s60dvfrf05yj";
const BLOVE_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiQkxPVkUiLCJpYXQiOjE2ODIwNTM3NDgsInN1YiI6ImFnZW50LXRva2VuIn0.NJcz9nRlLFkilnwRZQ4YR-LHQPFNhaqRmoYAgY1GYXuLhbux7rjVirFIj4jZlrugwn5yzLiNufXmBSQjKOmx8B5Mf0stOYuD9mYGdrZy_Gv9VsGBfX896_Jm2y33Nr35wzTGvkfDz32rnFReb1YDzI7AtbRpvlfbS7J6pLjESmR7lAwiS_4k_9LhLh2qOh5JVM1gWONzqN1z9domdICRVXxIOTaC8EwujtjOVlRJMPiCiD98hwlwar43ipQMQC1b5jOBTenZgKPpC1T6k2nMXmb0ABKl2PzTetC2m53t8qzahMOJaIYtZWBA3ljKpXM20EWUeylIj86dv4Lww4kuiARmS-AX5C6KM0iT9ER6uYK16MfUbZhtnzidH7DpAC0oHm-OZk1-SqLhX56Hf4eMEUAJRryZ_i-MoMGAE8g01W4iT1t6WrYtQlG7IkdiU0GgTPNYNDkBJwpE5bddhL2dOFiYigNaXVR6MXFUFyUlMIZZ37UWK_-R59Y1roaYA0JQ";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut conf = Configuration::new();
    // conf.bearer_access_token = Some(BLOVE_TOKEN.to_string());

    // let register_response = register(
    //     &conf,
    //     Some(RegisterRequest::new(Faction::Cosmic, "blove".to_string()))
    // ).await?;

    // println!("Register Response: {:?}", register_response);

    // let my_agent = get_my_agent(&conf).await?;
    // println!("My Agent: {:#?}", my_agent);

    // let my_contracts = get_contracts(&conf, None, None).await?;
    // println!("My contracts {:#?}", my_contracts);

    // let my_faction = get_faction(&conf, BLOVE_FACTION).await?;
    // println!("My faction {:#?}", my_faction);

    // let my_ships = get_my_ships(&conf, None, None).await?;
    // println!("My ships {:#?}", my_ships);

    // let starting_waypoint = get_waypoint(&conf, "X1-DF55", "X1-DF55-20250Z").await?;
    // println!("Starting waypoint: {:#?}", starting_waypoint);

    // let accepted_contract = accept_contract(&conf, AcceptContractParams { contract_id: CONTRACT_ID.to_owned() }).await?;
    // println!("Accepted Contract: {:#?}", accepted_contract);

    Ok(())
}
