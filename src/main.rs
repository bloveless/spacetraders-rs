use crate::api2::Api;
use crate::api_models::FactionSymbol;
use anyhow::Error;
use dotenv::dotenv;
use env_logger::{Env, Target};
use std::env;

mod api2;
mod api_models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let env = Env::default()
        .filter_or("RUST_LOG", "spacetraders_rs=trace")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::Builder::from_env(env)
        .target(Target::Stdout)
        .init();

    let mut api = Api::new();
    if let Ok(access_token) = env::var("ACCESS_TOKEN") {
        api.set_access_token(access_token);
    } else {
        if let Ok(username) = env::var("USERNAME") {
            let registration_response = api.register(FactionSymbol::Cosmic, username).await;

            match registration_response {
                Ok(rr) => println!("Registration Response {:#?}", rr),
                Err(e) => println!("Registration Error: {:#?}", e),
            }
        } else {
            return Err(Error::msg("ACCESS_KEY nor USERNAME was found in .env"));
        }

        return Ok(());
    }

    let my_ships_response = api.get_my_ships().await?;
    println!("My Ships: {:?}", my_ships_response.data);

    Ok(())
}
