#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/v2";
pub const API_VERSION: &str = "2.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetMyAgentResponse {
    /// OK
    OK
    (models::GetMyAgent200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AcceptContractResponse {
    /// OK
    OK
    (models::AcceptContract200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeliverContractResponse {
    /// OK
    OK
    (models::DeliverContract200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FulfillContractResponse {
    /// OK
    OK
    (models::FulfillContract200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetContractResponse {
    /// OK
    OK
    (models::GetContract200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetContractsResponse {
    /// OK
    OK
    (models::GetContracts200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RegisterResponse {
    /// OK
    OK
    (models::Register201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetFactionResponse {
    /// OK
    OK
    (models::GetFaction200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetFactionsResponse {
    /// 
    Status200
    (models::GetFactions200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChartResponse {
    /// Created
    Created
    (models::CreateChart201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateShipShipScanResponse {
    /// Created
    Created
    (models::CreateShipShipScan201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateShipSystemScanResponse {
    /// Created
    Created
    (models::CreateShipSystemScan201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateShipWaypointScanResponse {
    /// Created
    Created
    (models::CreateShipWaypointScan201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateSurveyResponse {
    /// Created
    Created
    (models::CreateSurvey201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DockShipResponse {
    /// The ship has successfully docked at it's current location.
    TheShipHasSuccessfullyDockedAtIt
    (models::DockShip200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ExtractResourcesResponse {
    /// Created
    Created
    (models::ExtractResources201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetMyShipResponse {
    /// OK
    OK
    (models::GetMyShip200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetMyShipCargoResponse {
    /// OK
    OK
    (models::GetMyShipCargo200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetMyShipsResponse {
    /// OK
    OK
    (models::GetMyShips200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetShipCooldownResponse {
    /// OK
    OK
    (models::GetShipCooldown200Response)
    ,
    /// No cooldown
    NoCooldown
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetShipNavResponse {
    /// The current nav status of the ship.
    TheCurrentNavStatusOfTheShip
    (models::GetShipNav200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum JettisonResponse {
    /// OK
    OK
    (models::Jettison200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum JumpShipResponse {
    /// OK
    OK
    (models::JumpShip200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NavigateShipResponse {
    /// The successful transit information including the route details and changes to ship fuel, supplies, and crew wages paid. The route includes the expected time of arrival.
    TheSuccessfulTransitInformationIncludingTheRouteDetailsAndChangesToShipFuel
    (models::NavigateShip200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OrbitShipResponse {
    /// The ship has successfully moved into orbit at it's current location.
    TheShipHasSuccessfullyMovedIntoOrbitAtIt
    (models::OrbitShip200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PatchShipNavResponse {
    /// The updated nav status of the ship.
    TheUpdatedNavStatusOfTheShip
    (models::GetShipNav200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PurchaseCargoResponse {
    /// Created
    Created
    (models::PurchaseCargo201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PurchaseShipResponse {
    /// Created
    Created
    (models::PurchaseShip201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RefuelShipResponse {
    /// OK
    OK
    (models::RefuelShip200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SellCargoResponse {
    /// Created
    Created
    (models::SellCargo201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ShipRefineResponse {
    /// The ship has successfully started refining.
    TheShipHasSuccessfullyStartedRefining
    (models::ShipRefine200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TransferCargoResponse {
    /// Created
    Created
    (models::TransferCargo200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WarpShipResponse {
    /// The successful transit information including the route details and changes to ship fuel, supplies, and crew wages paid. The route includes the expected time of arrival.
    TheSuccessfulTransitInformationIncludingTheRouteDetailsAndChangesToShipFuel
    (models::NavigateShip200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetJumpGateResponse {
    /// OK
    OK
    (models::GetJumpGate200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetMarketResponse {
    /// OK
    OK
    (models::GetMarket200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetShipyardResponse {
    /// OK
    OK
    (models::GetShipyard200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetSystemResponse {
    /// OK
    OK
    (models::GetSystem200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetSystemWaypointsResponse {
    /// OK
    OK
    (models::GetSystemWaypoints200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetSystemsResponse {
    /// OK
    OK
    (models::GetSystems200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetWaypointResponse {
    /// OK
    OK
    (models::GetWaypoint200Response)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// My Agent Details
    async fn get_my_agent(
        &self,
        context: &C) -> Result<GetMyAgentResponse, ApiError>;

    /// Accept Contract
    async fn accept_contract(
        &self,
        contract_id: String,
        context: &C) -> Result<AcceptContractResponse, ApiError>;

    /// Deliver Contract
    async fn deliver_contract(
        &self,
        contract_id: String,
        deliver_contract_request: Option<models::DeliverContractRequest>,
        context: &C) -> Result<DeliverContractResponse, ApiError>;

    /// Fulfill Contract
    async fn fulfill_contract(
        &self,
        contract_id: String,
        context: &C) -> Result<FulfillContractResponse, ApiError>;

    /// Get Contract
    async fn get_contract(
        &self,
        contract_id: String,
        context: &C) -> Result<GetContractResponse, ApiError>;

    /// List Contracts
    async fn get_contracts(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetContractsResponse, ApiError>;

    /// Register New Agent
    async fn register(
        &self,
        register_request: Option<models::RegisterRequest>,
        context: &C) -> Result<RegisterResponse, ApiError>;

    /// Get Faction
    async fn get_faction(
        &self,
        faction_symbol: String,
        context: &C) -> Result<GetFactionResponse, ApiError>;

    /// List Factions
    async fn get_factions(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetFactionsResponse, ApiError>;

    /// Create Chart
    async fn create_chart(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateChartResponse, ApiError>;

    /// Scan Ships
    async fn create_ship_ship_scan(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateShipShipScanResponse, ApiError>;

    /// Scan Systems
    async fn create_ship_system_scan(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateShipSystemScanResponse, ApiError>;

    /// Scan Waypoints
    async fn create_ship_waypoint_scan(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateShipWaypointScanResponse, ApiError>;

    /// Create Survey
    async fn create_survey(
        &self,
        ship_symbol: String,
        context: &C) -> Result<CreateSurveyResponse, ApiError>;

    /// Dock Ship
    async fn dock_ship(
        &self,
        ship_symbol: String,
        context: &C) -> Result<DockShipResponse, ApiError>;

    /// Extract Resources
    async fn extract_resources(
        &self,
        ship_symbol: String,
        extract_resources_request: Option<models::ExtractResourcesRequest>,
        context: &C) -> Result<ExtractResourcesResponse, ApiError>;

    /// Get Ship
    async fn get_my_ship(
        &self,
        ship_symbol: String,
        context: &C) -> Result<GetMyShipResponse, ApiError>;

    /// Get Ship Cargo
    async fn get_my_ship_cargo(
        &self,
        ship_symbol: String,
        context: &C) -> Result<GetMyShipCargoResponse, ApiError>;

    /// List Ships
    async fn get_my_ships(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetMyShipsResponse, ApiError>;

    /// Get Ship Cooldown
    async fn get_ship_cooldown(
        &self,
        ship_symbol: String,
        context: &C) -> Result<GetShipCooldownResponse, ApiError>;

    /// Get Ship Nav
    async fn get_ship_nav(
        &self,
        ship_symbol: String,
        context: &C) -> Result<GetShipNavResponse, ApiError>;

    /// Jettison Cargo
    async fn jettison(
        &self,
        ship_symbol: String,
        jettison_request: Option<models::JettisonRequest>,
        context: &C) -> Result<JettisonResponse, ApiError>;

    /// Jump Ship
    async fn jump_ship(
        &self,
        ship_symbol: String,
        jump_ship_request: Option<models::JumpShipRequest>,
        context: &C) -> Result<JumpShipResponse, ApiError>;

    /// Navigate Ship
    async fn navigate_ship(
        &self,
        ship_symbol: String,
        navigate_ship_request: Option<models::NavigateShipRequest>,
        context: &C) -> Result<NavigateShipResponse, ApiError>;

    /// Orbit Ship
    async fn orbit_ship(
        &self,
        ship_symbol: String,
        context: &C) -> Result<OrbitShipResponse, ApiError>;

    /// Patch Ship Nav
    async fn patch_ship_nav(
        &self,
        ship_symbol: String,
        patch_ship_nav_request: Option<models::PatchShipNavRequest>,
        context: &C) -> Result<PatchShipNavResponse, ApiError>;

    /// Purchase Cargo
    async fn purchase_cargo(
        &self,
        ship_symbol: String,
        purchase_cargo_request: Option<models::PurchaseCargoRequest>,
        context: &C) -> Result<PurchaseCargoResponse, ApiError>;

    /// Purchase Ship
    async fn purchase_ship(
        &self,
        purchase_ship_request: Option<models::PurchaseShipRequest>,
        context: &C) -> Result<PurchaseShipResponse, ApiError>;

    /// Refuel Ship
    async fn refuel_ship(
        &self,
        ship_symbol: String,
        context: &C) -> Result<RefuelShipResponse, ApiError>;

    /// Sell Cargo
    async fn sell_cargo(
        &self,
        ship_symbol: String,
        sell_cargo_request: Option<models::SellCargoRequest>,
        context: &C) -> Result<SellCargoResponse, ApiError>;

    /// Ship Refine
    async fn ship_refine(
        &self,
        ship_symbol: String,
        ship_refine_request: Option<models::ShipRefineRequest>,
        context: &C) -> Result<ShipRefineResponse, ApiError>;

    /// Transfer Cargo
    async fn transfer_cargo(
        &self,
        ship_symbol: String,
        transfer_cargo_request: Option<models::TransferCargoRequest>,
        context: &C) -> Result<TransferCargoResponse, ApiError>;

    /// Warp Ship
    async fn warp_ship(
        &self,
        ship_symbol: String,
        navigate_ship_request: Option<models::NavigateShipRequest>,
        context: &C) -> Result<WarpShipResponse, ApiError>;

    /// Get Jump Gate
    async fn get_jump_gate(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        context: &C) -> Result<GetJumpGateResponse, ApiError>;

    /// Get Market
    async fn get_market(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        context: &C) -> Result<GetMarketResponse, ApiError>;

    /// Get Shipyard
    async fn get_shipyard(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        context: &C) -> Result<GetShipyardResponse, ApiError>;

    /// Get System
    async fn get_system(
        &self,
        system_symbol: String,
        context: &C) -> Result<GetSystemResponse, ApiError>;

    /// List Waypoints
    async fn get_system_waypoints(
        &self,
        system_symbol: String,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetSystemWaypointsResponse, ApiError>;

    /// List Systems
    async fn get_systems(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<GetSystemsResponse, ApiError>;

    /// Get Waypoint
    async fn get_waypoint(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        context: &C) -> Result<GetWaypointResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// My Agent Details
    async fn get_my_agent(
        &self,
        ) -> Result<GetMyAgentResponse, ApiError>;

    /// Accept Contract
    async fn accept_contract(
        &self,
        contract_id: String,
        ) -> Result<AcceptContractResponse, ApiError>;

    /// Deliver Contract
    async fn deliver_contract(
        &self,
        contract_id: String,
        deliver_contract_request: Option<models::DeliverContractRequest>,
        ) -> Result<DeliverContractResponse, ApiError>;

    /// Fulfill Contract
    async fn fulfill_contract(
        &self,
        contract_id: String,
        ) -> Result<FulfillContractResponse, ApiError>;

    /// Get Contract
    async fn get_contract(
        &self,
        contract_id: String,
        ) -> Result<GetContractResponse, ApiError>;

    /// List Contracts
    async fn get_contracts(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetContractsResponse, ApiError>;

    /// Register New Agent
    async fn register(
        &self,
        register_request: Option<models::RegisterRequest>,
        ) -> Result<RegisterResponse, ApiError>;

    /// Get Faction
    async fn get_faction(
        &self,
        faction_symbol: String,
        ) -> Result<GetFactionResponse, ApiError>;

    /// List Factions
    async fn get_factions(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetFactionsResponse, ApiError>;

    /// Create Chart
    async fn create_chart(
        &self,
        ship_symbol: String,
        ) -> Result<CreateChartResponse, ApiError>;

    /// Scan Ships
    async fn create_ship_ship_scan(
        &self,
        ship_symbol: String,
        ) -> Result<CreateShipShipScanResponse, ApiError>;

    /// Scan Systems
    async fn create_ship_system_scan(
        &self,
        ship_symbol: String,
        ) -> Result<CreateShipSystemScanResponse, ApiError>;

    /// Scan Waypoints
    async fn create_ship_waypoint_scan(
        &self,
        ship_symbol: String,
        ) -> Result<CreateShipWaypointScanResponse, ApiError>;

    /// Create Survey
    async fn create_survey(
        &self,
        ship_symbol: String,
        ) -> Result<CreateSurveyResponse, ApiError>;

    /// Dock Ship
    async fn dock_ship(
        &self,
        ship_symbol: String,
        ) -> Result<DockShipResponse, ApiError>;

    /// Extract Resources
    async fn extract_resources(
        &self,
        ship_symbol: String,
        extract_resources_request: Option<models::ExtractResourcesRequest>,
        ) -> Result<ExtractResourcesResponse, ApiError>;

    /// Get Ship
    async fn get_my_ship(
        &self,
        ship_symbol: String,
        ) -> Result<GetMyShipResponse, ApiError>;

    /// Get Ship Cargo
    async fn get_my_ship_cargo(
        &self,
        ship_symbol: String,
        ) -> Result<GetMyShipCargoResponse, ApiError>;

    /// List Ships
    async fn get_my_ships(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetMyShipsResponse, ApiError>;

    /// Get Ship Cooldown
    async fn get_ship_cooldown(
        &self,
        ship_symbol: String,
        ) -> Result<GetShipCooldownResponse, ApiError>;

    /// Get Ship Nav
    async fn get_ship_nav(
        &self,
        ship_symbol: String,
        ) -> Result<GetShipNavResponse, ApiError>;

    /// Jettison Cargo
    async fn jettison(
        &self,
        ship_symbol: String,
        jettison_request: Option<models::JettisonRequest>,
        ) -> Result<JettisonResponse, ApiError>;

    /// Jump Ship
    async fn jump_ship(
        &self,
        ship_symbol: String,
        jump_ship_request: Option<models::JumpShipRequest>,
        ) -> Result<JumpShipResponse, ApiError>;

    /// Navigate Ship
    async fn navigate_ship(
        &self,
        ship_symbol: String,
        navigate_ship_request: Option<models::NavigateShipRequest>,
        ) -> Result<NavigateShipResponse, ApiError>;

    /// Orbit Ship
    async fn orbit_ship(
        &self,
        ship_symbol: String,
        ) -> Result<OrbitShipResponse, ApiError>;

    /// Patch Ship Nav
    async fn patch_ship_nav(
        &self,
        ship_symbol: String,
        patch_ship_nav_request: Option<models::PatchShipNavRequest>,
        ) -> Result<PatchShipNavResponse, ApiError>;

    /// Purchase Cargo
    async fn purchase_cargo(
        &self,
        ship_symbol: String,
        purchase_cargo_request: Option<models::PurchaseCargoRequest>,
        ) -> Result<PurchaseCargoResponse, ApiError>;

    /// Purchase Ship
    async fn purchase_ship(
        &self,
        purchase_ship_request: Option<models::PurchaseShipRequest>,
        ) -> Result<PurchaseShipResponse, ApiError>;

    /// Refuel Ship
    async fn refuel_ship(
        &self,
        ship_symbol: String,
        ) -> Result<RefuelShipResponse, ApiError>;

    /// Sell Cargo
    async fn sell_cargo(
        &self,
        ship_symbol: String,
        sell_cargo_request: Option<models::SellCargoRequest>,
        ) -> Result<SellCargoResponse, ApiError>;

    /// Ship Refine
    async fn ship_refine(
        &self,
        ship_symbol: String,
        ship_refine_request: Option<models::ShipRefineRequest>,
        ) -> Result<ShipRefineResponse, ApiError>;

    /// Transfer Cargo
    async fn transfer_cargo(
        &self,
        ship_symbol: String,
        transfer_cargo_request: Option<models::TransferCargoRequest>,
        ) -> Result<TransferCargoResponse, ApiError>;

    /// Warp Ship
    async fn warp_ship(
        &self,
        ship_symbol: String,
        navigate_ship_request: Option<models::NavigateShipRequest>,
        ) -> Result<WarpShipResponse, ApiError>;

    /// Get Jump Gate
    async fn get_jump_gate(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        ) -> Result<GetJumpGateResponse, ApiError>;

    /// Get Market
    async fn get_market(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        ) -> Result<GetMarketResponse, ApiError>;

    /// Get Shipyard
    async fn get_shipyard(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        ) -> Result<GetShipyardResponse, ApiError>;

    /// Get System
    async fn get_system(
        &self,
        system_symbol: String,
        ) -> Result<GetSystemResponse, ApiError>;

    /// List Waypoints
    async fn get_system_waypoints(
        &self,
        system_symbol: String,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetSystemWaypointsResponse, ApiError>;

    /// List Systems
    async fn get_systems(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetSystemsResponse, ApiError>;

    /// Get Waypoint
    async fn get_waypoint(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        ) -> Result<GetWaypointResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// My Agent Details
    async fn get_my_agent(
        &self,
        ) -> Result<GetMyAgentResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_my_agent(&context).await
    }

    /// Accept Contract
    async fn accept_contract(
        &self,
        contract_id: String,
        ) -> Result<AcceptContractResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().accept_contract(contract_id, &context).await
    }

    /// Deliver Contract
    async fn deliver_contract(
        &self,
        contract_id: String,
        deliver_contract_request: Option<models::DeliverContractRequest>,
        ) -> Result<DeliverContractResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().deliver_contract(contract_id, deliver_contract_request, &context).await
    }

    /// Fulfill Contract
    async fn fulfill_contract(
        &self,
        contract_id: String,
        ) -> Result<FulfillContractResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().fulfill_contract(contract_id, &context).await
    }

    /// Get Contract
    async fn get_contract(
        &self,
        contract_id: String,
        ) -> Result<GetContractResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_contract(contract_id, &context).await
    }

    /// List Contracts
    async fn get_contracts(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetContractsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_contracts(page, limit, &context).await
    }

    /// Register New Agent
    async fn register(
        &self,
        register_request: Option<models::RegisterRequest>,
        ) -> Result<RegisterResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().register(register_request, &context).await
    }

    /// Get Faction
    async fn get_faction(
        &self,
        faction_symbol: String,
        ) -> Result<GetFactionResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_faction(faction_symbol, &context).await
    }

    /// List Factions
    async fn get_factions(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetFactionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_factions(page, limit, &context).await
    }

    /// Create Chart
    async fn create_chart(
        &self,
        ship_symbol: String,
        ) -> Result<CreateChartResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_chart(ship_symbol, &context).await
    }

    /// Scan Ships
    async fn create_ship_ship_scan(
        &self,
        ship_symbol: String,
        ) -> Result<CreateShipShipScanResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_ship_ship_scan(ship_symbol, &context).await
    }

    /// Scan Systems
    async fn create_ship_system_scan(
        &self,
        ship_symbol: String,
        ) -> Result<CreateShipSystemScanResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_ship_system_scan(ship_symbol, &context).await
    }

    /// Scan Waypoints
    async fn create_ship_waypoint_scan(
        &self,
        ship_symbol: String,
        ) -> Result<CreateShipWaypointScanResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_ship_waypoint_scan(ship_symbol, &context).await
    }

    /// Create Survey
    async fn create_survey(
        &self,
        ship_symbol: String,
        ) -> Result<CreateSurveyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_survey(ship_symbol, &context).await
    }

    /// Dock Ship
    async fn dock_ship(
        &self,
        ship_symbol: String,
        ) -> Result<DockShipResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().dock_ship(ship_symbol, &context).await
    }

    /// Extract Resources
    async fn extract_resources(
        &self,
        ship_symbol: String,
        extract_resources_request: Option<models::ExtractResourcesRequest>,
        ) -> Result<ExtractResourcesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().extract_resources(ship_symbol, extract_resources_request, &context).await
    }

    /// Get Ship
    async fn get_my_ship(
        &self,
        ship_symbol: String,
        ) -> Result<GetMyShipResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_my_ship(ship_symbol, &context).await
    }

    /// Get Ship Cargo
    async fn get_my_ship_cargo(
        &self,
        ship_symbol: String,
        ) -> Result<GetMyShipCargoResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_my_ship_cargo(ship_symbol, &context).await
    }

    /// List Ships
    async fn get_my_ships(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetMyShipsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_my_ships(page, limit, &context).await
    }

    /// Get Ship Cooldown
    async fn get_ship_cooldown(
        &self,
        ship_symbol: String,
        ) -> Result<GetShipCooldownResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_ship_cooldown(ship_symbol, &context).await
    }

    /// Get Ship Nav
    async fn get_ship_nav(
        &self,
        ship_symbol: String,
        ) -> Result<GetShipNavResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_ship_nav(ship_symbol, &context).await
    }

    /// Jettison Cargo
    async fn jettison(
        &self,
        ship_symbol: String,
        jettison_request: Option<models::JettisonRequest>,
        ) -> Result<JettisonResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().jettison(ship_symbol, jettison_request, &context).await
    }

    /// Jump Ship
    async fn jump_ship(
        &self,
        ship_symbol: String,
        jump_ship_request: Option<models::JumpShipRequest>,
        ) -> Result<JumpShipResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().jump_ship(ship_symbol, jump_ship_request, &context).await
    }

    /// Navigate Ship
    async fn navigate_ship(
        &self,
        ship_symbol: String,
        navigate_ship_request: Option<models::NavigateShipRequest>,
        ) -> Result<NavigateShipResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().navigate_ship(ship_symbol, navigate_ship_request, &context).await
    }

    /// Orbit Ship
    async fn orbit_ship(
        &self,
        ship_symbol: String,
        ) -> Result<OrbitShipResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().orbit_ship(ship_symbol, &context).await
    }

    /// Patch Ship Nav
    async fn patch_ship_nav(
        &self,
        ship_symbol: String,
        patch_ship_nav_request: Option<models::PatchShipNavRequest>,
        ) -> Result<PatchShipNavResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().patch_ship_nav(ship_symbol, patch_ship_nav_request, &context).await
    }

    /// Purchase Cargo
    async fn purchase_cargo(
        &self,
        ship_symbol: String,
        purchase_cargo_request: Option<models::PurchaseCargoRequest>,
        ) -> Result<PurchaseCargoResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().purchase_cargo(ship_symbol, purchase_cargo_request, &context).await
    }

    /// Purchase Ship
    async fn purchase_ship(
        &self,
        purchase_ship_request: Option<models::PurchaseShipRequest>,
        ) -> Result<PurchaseShipResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().purchase_ship(purchase_ship_request, &context).await
    }

    /// Refuel Ship
    async fn refuel_ship(
        &self,
        ship_symbol: String,
        ) -> Result<RefuelShipResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().refuel_ship(ship_symbol, &context).await
    }

    /// Sell Cargo
    async fn sell_cargo(
        &self,
        ship_symbol: String,
        sell_cargo_request: Option<models::SellCargoRequest>,
        ) -> Result<SellCargoResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().sell_cargo(ship_symbol, sell_cargo_request, &context).await
    }

    /// Ship Refine
    async fn ship_refine(
        &self,
        ship_symbol: String,
        ship_refine_request: Option<models::ShipRefineRequest>,
        ) -> Result<ShipRefineResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().ship_refine(ship_symbol, ship_refine_request, &context).await
    }

    /// Transfer Cargo
    async fn transfer_cargo(
        &self,
        ship_symbol: String,
        transfer_cargo_request: Option<models::TransferCargoRequest>,
        ) -> Result<TransferCargoResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().transfer_cargo(ship_symbol, transfer_cargo_request, &context).await
    }

    /// Warp Ship
    async fn warp_ship(
        &self,
        ship_symbol: String,
        navigate_ship_request: Option<models::NavigateShipRequest>,
        ) -> Result<WarpShipResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().warp_ship(ship_symbol, navigate_ship_request, &context).await
    }

    /// Get Jump Gate
    async fn get_jump_gate(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        ) -> Result<GetJumpGateResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_jump_gate(system_symbol, waypoint_symbol, &context).await
    }

    /// Get Market
    async fn get_market(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        ) -> Result<GetMarketResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_market(system_symbol, waypoint_symbol, &context).await
    }

    /// Get Shipyard
    async fn get_shipyard(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        ) -> Result<GetShipyardResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_shipyard(system_symbol, waypoint_symbol, &context).await
    }

    /// Get System
    async fn get_system(
        &self,
        system_symbol: String,
        ) -> Result<GetSystemResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_system(system_symbol, &context).await
    }

    /// List Waypoints
    async fn get_system_waypoints(
        &self,
        system_symbol: String,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetSystemWaypointsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_system_waypoints(system_symbol, page, limit, &context).await
    }

    /// List Systems
    async fn get_systems(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
        ) -> Result<GetSystemsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_systems(page, limit, &context).await
    }

    /// Get Waypoint
    async fn get_waypoint(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        ) -> Result<GetWaypointResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_waypoint(system_symbol, waypoint_symbol, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
