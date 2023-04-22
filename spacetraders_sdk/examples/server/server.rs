//! Main library entry point for spacetraders_sdk implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use spacetraders_sdk::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        spacetraders_sdk::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use spacetraders_sdk::{
    Api,
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
use spacetraders_sdk::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// My Agent Details
    async fn get_my_agent(
        &self,
        context: &C) -> Result<GetMyAgentResponse, ApiError>
    {
        let context = context.clone();
        info!("get_my_agent() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Accept Contract
    async fn accept_contract(
        &self,
        contract_id: String,
        context: &C) -> Result<AcceptContractResponse, ApiError>
    {
        let context = context.clone();
        info!("accept_contract(\"{}\") - X-Span-ID: {:?}", contract_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Deliver Contract
    async fn deliver_contract(
        &self,
        contract_id: String,
        deliver_contract_request: Option<models::DeliverContractRequest>,
        context: &C) -> Result<DeliverContractResponse, ApiError>
    {
        let context = context.clone();
        info!("deliver_contract(\"{}\", {:?}) - X-Span-ID: {:?}", contract_id, deliver_contract_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Fulfill Contract
    async fn fulfill_contract(
        &self,
        contract_id: String,
        context: &C) -> Result<FulfillContractResponse, ApiError>
    {
        let context = context.clone();
        info!("fulfill_contract(\"{}\") - X-Span-ID: {:?}", contract_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Contract
    async fn get_contract(
        &self,
        contract_id: String,
        context: &C) -> Result<GetContractResponse, ApiError>
    {
        let context = context.clone();
        info!("get_contract(\"{}\") - X-Span-ID: {:?}", contract_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List Contracts
    async fn get_contracts(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetContractsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_contracts({:?}, {:?}) - X-Span-ID: {:?}", page, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Register New Agent
    async fn register(
        &self,
        register_request: Option<models::RegisterRequest>,
        context: &C) -> Result<RegisterResponse, ApiError>
    {
        let context = context.clone();
        info!("register({:?}) - X-Span-ID: {:?}", register_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Faction
    async fn get_faction(
        &self,
        faction_symbol: String,
        context: &C) -> Result<GetFactionResponse, ApiError>
    {
        let context = context.clone();
        info!("get_faction(\"{}\") - X-Span-ID: {:?}", faction_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List Factions
    async fn get_factions(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetFactionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_factions({:?}, {:?}) - X-Span-ID: {:?}", page, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create Chart
    async fn create_chart(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateChartResponse, ApiError>
    {
        let context = context.clone();
        info!("create_chart(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Scan Ships
    async fn create_ship_ship_scan(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateShipShipScanResponse, ApiError>
    {
        let context = context.clone();
        info!("create_ship_ship_scan(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Scan Systems
    async fn create_ship_system_scan(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateShipSystemScanResponse, ApiError>
    {
        let context = context.clone();
        info!("create_ship_system_scan(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Scan Waypoints
    async fn create_ship_waypoint_scan(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateShipWaypointScanResponse, ApiError>
    {
        let context = context.clone();
        info!("create_ship_waypoint_scan(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create Survey
    async fn create_survey(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateSurveyResponse, ApiError>
    {
        let context = context.clone();
        info!("create_survey(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Dock Ship
    async fn dock_ship(
        &self,
        ship_symbol: String,
        context: &C) -> Result<DockShipResponse, ApiError>
    {
        let context = context.clone();
        info!("dock_ship(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Extract Resources
    async fn extract_resources(
        &self,
        ship_symbol: String,
        extract_resources_request: Option<models::ExtractResourcesRequest>,
        context: &C) -> Result<ExtractResourcesResponse, ApiError>
    {
        let context = context.clone();
        info!("extract_resources(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, extract_resources_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Ship
    async fn get_my_ship(
        &self,
        ship_symbol: String,
        context: &C) -> Result<GetMyShipResponse, ApiError>
    {
        let context = context.clone();
        info!("get_my_ship(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Ship Cargo
    async fn get_my_ship_cargo(
        &self,
        ship_symbol: String,
        context: &C) -> Result<GetMyShipCargoResponse, ApiError>
    {
        let context = context.clone();
        info!("get_my_ship_cargo(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List Ships
    async fn get_my_ships(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetMyShipsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_my_ships({:?}, {:?}) - X-Span-ID: {:?}", page, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Ship Cooldown
    async fn get_ship_cooldown(
        &self,
        ship_symbol: String,
        context: &C) -> Result<GetShipCooldownResponse, ApiError>
    {
        let context = context.clone();
        info!("get_ship_cooldown(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Ship Nav
    async fn get_ship_nav(
        &self,
        ship_symbol: String,
        context: &C) -> Result<GetShipNavResponse, ApiError>
    {
        let context = context.clone();
        info!("get_ship_nav(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Jettison Cargo
    async fn jettison(
        &self,
        ship_symbol: String,
        jettison_request: Option<models::JettisonRequest>,
        context: &C) -> Result<JettisonResponse, ApiError>
    {
        let context = context.clone();
        info!("jettison(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, jettison_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Jump Ship
    async fn jump_ship(
        &self,
        ship_symbol: String,
        jump_ship_request: Option<models::JumpShipRequest>,
        context: &C) -> Result<JumpShipResponse, ApiError>
    {
        let context = context.clone();
        info!("jump_ship(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, jump_ship_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Navigate Ship
    async fn navigate_ship(
        &self,
        ship_symbol: String,
        navigate_ship_request: Option<models::NavigateShipRequest>,
        context: &C) -> Result<NavigateShipResponse, ApiError>
    {
        let context = context.clone();
        info!("navigate_ship(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, navigate_ship_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Orbit Ship
    async fn orbit_ship(
        &self,
        ship_symbol: String,
        context: &C) -> Result<OrbitShipResponse, ApiError>
    {
        let context = context.clone();
        info!("orbit_ship(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Patch Ship Nav
    async fn patch_ship_nav(
        &self,
        ship_symbol: String,
        patch_ship_nav_request: Option<models::PatchShipNavRequest>,
        context: &C) -> Result<PatchShipNavResponse, ApiError>
    {
        let context = context.clone();
        info!("patch_ship_nav(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, patch_ship_nav_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Purchase Cargo
    async fn purchase_cargo(
        &self,
        ship_symbol: String,
        purchase_cargo_request: Option<models::PurchaseCargoRequest>,
        context: &C) -> Result<PurchaseCargoResponse, ApiError>
    {
        let context = context.clone();
        info!("purchase_cargo(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, purchase_cargo_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Purchase Ship
    async fn purchase_ship(
        &self,
        purchase_ship_request: Option<models::PurchaseShipRequest>,
        context: &C) -> Result<PurchaseShipResponse, ApiError>
    {
        let context = context.clone();
        info!("purchase_ship({:?}) - X-Span-ID: {:?}", purchase_ship_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Refuel Ship
    async fn refuel_ship(
        &self,
        ship_symbol: String,
        context: &C) -> Result<RefuelShipResponse, ApiError>
    {
        let context = context.clone();
        info!("refuel_ship(\"{}\") - X-Span-ID: {:?}", ship_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Sell Cargo
    async fn sell_cargo(
        &self,
        ship_symbol: String,
        sell_cargo_request: Option<models::SellCargoRequest>,
        context: &C) -> Result<SellCargoResponse, ApiError>
    {
        let context = context.clone();
        info!("sell_cargo(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, sell_cargo_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Ship Refine
    async fn ship_refine(
        &self,
        ship_symbol: String,
        ship_refine_request: Option<models::ShipRefineRequest>,
        context: &C) -> Result<ShipRefineResponse, ApiError>
    {
        let context = context.clone();
        info!("ship_refine(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, ship_refine_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Transfer Cargo
    async fn transfer_cargo(
        &self,
        ship_symbol: String,
        transfer_cargo_request: Option<models::TransferCargoRequest>,
        context: &C) -> Result<TransferCargoResponse, ApiError>
    {
        let context = context.clone();
        info!("transfer_cargo(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, transfer_cargo_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Warp Ship
    async fn warp_ship(
        &self,
        ship_symbol: String,
        navigate_ship_request: Option<models::NavigateShipRequest>,
        context: &C) -> Result<WarpShipResponse, ApiError>
    {
        let context = context.clone();
        info!("warp_ship(\"{}\", {:?}) - X-Span-ID: {:?}", ship_symbol, navigate_ship_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Jump Gate
    async fn get_jump_gate(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        context: &C) -> Result<GetJumpGateResponse, ApiError>
    {
        let context = context.clone();
        info!("get_jump_gate(\"{}\", \"{}\") - X-Span-ID: {:?}", system_symbol, waypoint_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Market
    async fn get_market(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        context: &C) -> Result<GetMarketResponse, ApiError>
    {
        let context = context.clone();
        info!("get_market(\"{}\", \"{}\") - X-Span-ID: {:?}", system_symbol, waypoint_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Shipyard
    async fn get_shipyard(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        context: &C) -> Result<GetShipyardResponse, ApiError>
    {
        let context = context.clone();
        info!("get_shipyard(\"{}\", \"{}\") - X-Span-ID: {:?}", system_symbol, waypoint_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get System
    async fn get_system(
        &self,
        system_symbol: String,
        context: &C) -> Result<GetSystemResponse, ApiError>
    {
        let context = context.clone();
        info!("get_system(\"{}\") - X-Span-ID: {:?}", system_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List Waypoints
    async fn get_system_waypoints(
        &self,
        system_symbol: String,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetSystemWaypointsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_system_waypoints(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", system_symbol, page, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List Systems
    async fn get_systems(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetSystemsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_systems({:?}, {:?}) - X-Span-ID: {:?}", page, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Waypoint
    async fn get_waypoint(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        context: &C) -> Result<GetWaypointResponse, ApiError>
    {
        let context = context.clone();
        info!("get_waypoint(\"{}\", \"{}\") - X-Span-ID: {:?}", system_symbol, waypoint_symbol, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
