use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
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
     GetWaypointResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/v2/factions$",
            r"^/v2/factions/(?P<factionSymbol>[^/?#]*)$",
            r"^/v2/my/agent$",
            r"^/v2/my/contracts$",
            r"^/v2/my/contracts/(?P<contractId>[^/?#]*)$",
            r"^/v2/my/contracts/(?P<contractId>[^/?#]*)/accept$",
            r"^/v2/my/contracts/(?P<contractId>[^/?#]*)/deliver$",
            r"^/v2/my/contracts/(?P<contractId>[^/?#]*)/fulfill$",
            r"^/v2/my/ships$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/cargo$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/chart$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/cooldown$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/dock$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/extract$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/jettison$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/jump$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/nav$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/navigate$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/orbit$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/purchase$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/refine$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/refuel$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/scan/ships$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/scan/systems$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/scan/waypoints$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/sell$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/survey$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/transfer$",
            r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/warp$",
            r"^/v2/register$",
            r"^/v2/systems$",
            r"^/v2/systems/(?P<systemSymbol>[^/?#]*)$",
            r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints$",
            r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints/(?P<waypointSymbol>[^/?#]*)$",
            r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints/(?P<waypointSymbol>[^/?#]*)/jump-gate$",
            r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints/(?P<waypointSymbol>[^/?#]*)/market$",
            r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints/(?P<waypointSymbol>[^/?#]*)/shipyard$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_FACTIONS: usize = 0;
    pub(crate) static ID_FACTIONS_FACTIONSYMBOL: usize = 1;
    lazy_static! {
        pub static ref REGEX_FACTIONS_FACTIONSYMBOL: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/factions/(?P<factionSymbol>[^/?#]*)$")
                .expect("Unable to create regex for FACTIONS_FACTIONSYMBOL");
    }
    pub(crate) static ID_MY_AGENT: usize = 2;
    pub(crate) static ID_MY_CONTRACTS: usize = 3;
    pub(crate) static ID_MY_CONTRACTS_CONTRACTID: usize = 4;
    lazy_static! {
        pub static ref REGEX_MY_CONTRACTS_CONTRACTID: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/contracts/(?P<contractId>[^/?#]*)$")
                .expect("Unable to create regex for MY_CONTRACTS_CONTRACTID");
    }
    pub(crate) static ID_MY_CONTRACTS_CONTRACTID_ACCEPT: usize = 5;
    lazy_static! {
        pub static ref REGEX_MY_CONTRACTS_CONTRACTID_ACCEPT: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/contracts/(?P<contractId>[^/?#]*)/accept$")
                .expect("Unable to create regex for MY_CONTRACTS_CONTRACTID_ACCEPT");
    }
    pub(crate) static ID_MY_CONTRACTS_CONTRACTID_DELIVER: usize = 6;
    lazy_static! {
        pub static ref REGEX_MY_CONTRACTS_CONTRACTID_DELIVER: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/contracts/(?P<contractId>[^/?#]*)/deliver$")
                .expect("Unable to create regex for MY_CONTRACTS_CONTRACTID_DELIVER");
    }
    pub(crate) static ID_MY_CONTRACTS_CONTRACTID_FULFILL: usize = 7;
    lazy_static! {
        pub static ref REGEX_MY_CONTRACTS_CONTRACTID_FULFILL: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/contracts/(?P<contractId>[^/?#]*)/fulfill$")
                .expect("Unable to create regex for MY_CONTRACTS_CONTRACTID_FULFILL");
    }
    pub(crate) static ID_MY_SHIPS: usize = 8;
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL: usize = 9;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_CARGO: usize = 10;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_CARGO: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/cargo$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_CARGO");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_CHART: usize = 11;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_CHART: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/chart$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_CHART");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_COOLDOWN: usize = 12;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_COOLDOWN: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/cooldown$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_COOLDOWN");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_DOCK: usize = 13;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_DOCK: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/dock$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_DOCK");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_EXTRACT: usize = 14;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_EXTRACT: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/extract$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_EXTRACT");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_JETTISON: usize = 15;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_JETTISON: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/jettison$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_JETTISON");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_JUMP: usize = 16;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_JUMP: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/jump$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_JUMP");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_NAV: usize = 17;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_NAV: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/nav$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_NAV");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_NAVIGATE: usize = 18;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_NAVIGATE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/navigate$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_NAVIGATE");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_ORBIT: usize = 19;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_ORBIT: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/orbit$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_ORBIT");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_PURCHASE: usize = 20;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_PURCHASE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/purchase$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_PURCHASE");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_REFINE: usize = 21;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_REFINE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/refine$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_REFINE");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_REFUEL: usize = 22;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_REFUEL: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/refuel$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_REFUEL");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS: usize = 23;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/scan/ships$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS: usize = 24;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/scan/systems$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS: usize = 25;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/scan/waypoints$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_SELL: usize = 26;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_SELL: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/sell$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_SELL");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_SURVEY: usize = 27;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_SURVEY: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/survey$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_SURVEY");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_TRANSFER: usize = 28;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_TRANSFER: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/transfer$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_TRANSFER");
    }
    pub(crate) static ID_MY_SHIPS_SHIPSYMBOL_WARP: usize = 29;
    lazy_static! {
        pub static ref REGEX_MY_SHIPS_SHIPSYMBOL_WARP: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/my/ships/(?P<shipSymbol>[^/?#]*)/warp$")
                .expect("Unable to create regex for MY_SHIPS_SHIPSYMBOL_WARP");
    }
    pub(crate) static ID_REGISTER: usize = 30;
    pub(crate) static ID_SYSTEMS: usize = 31;
    pub(crate) static ID_SYSTEMS_SYSTEMSYMBOL: usize = 32;
    lazy_static! {
        pub static ref REGEX_SYSTEMS_SYSTEMSYMBOL: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/systems/(?P<systemSymbol>[^/?#]*)$")
                .expect("Unable to create regex for SYSTEMS_SYSTEMSYMBOL");
    }
    pub(crate) static ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS: usize = 33;
    lazy_static! {
        pub static ref REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints$")
                .expect("Unable to create regex for SYSTEMS_SYSTEMSYMBOL_WAYPOINTS");
    }
    pub(crate) static ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL: usize = 34;
    lazy_static! {
        pub static ref REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints/(?P<waypointSymbol>[^/?#]*)$")
                .expect("Unable to create regex for SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL");
    }
    pub(crate) static ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE: usize = 35;
    lazy_static! {
        pub static ref REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints/(?P<waypointSymbol>[^/?#]*)/jump-gate$")
                .expect("Unable to create regex for SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE");
    }
    pub(crate) static ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET: usize = 36;
    lazy_static! {
        pub static ref REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints/(?P<waypointSymbol>[^/?#]*)/market$")
                .expect("Unable to create regex for SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET");
    }
    pub(crate) static ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD: usize = 37;
    lazy_static! {
        pub static ref REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v2/systems/(?P<systemSymbol>[^/?#]*)/waypoints/(?P<waypointSymbol>[^/?#]*)/shipyard$")
                .expect("Unable to create regex for SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match method {

            // GetMyAgent - GET /my/agent
            hyper::Method::GET if path.matched(paths::ID_MY_AGENT) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.get_my_agent(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetMyAgentResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MY_AGENT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AcceptContract - POST /my/contracts/{contractId}/accept
            hyper::Method::POST if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_ACCEPT) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_CONTRACTS_CONTRACTID_ACCEPT
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_CONTRACTS_CONTRACTID_ACCEPT in set but failed match against \"{}\"", path, paths::REGEX_MY_CONTRACTS_CONTRACTID_ACCEPT.as_str())
                    );

                let param_contract_id = match percent_encoding::percent_decode(path_params["contractId"].as_bytes()).decode_utf8() {
                    Ok(param_contract_id) => match param_contract_id.parse::<String>() {
                        Ok(param_contract_id) => param_contract_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter contractId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["contractId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.accept_contract(
                                            param_contract_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AcceptContractResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACCEPT_CONTRACT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeliverContract - POST /my/contracts/{contractId}/deliver
            hyper::Method::POST if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_DELIVER) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_CONTRACTS_CONTRACTID_DELIVER
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_CONTRACTS_CONTRACTID_DELIVER in set but failed match against \"{}\"", path, paths::REGEX_MY_CONTRACTS_CONTRACTID_DELIVER.as_str())
                    );

                let param_contract_id = match percent_encoding::percent_decode(path_params["contractId"].as_bytes()).decode_utf8() {
                    Ok(param_contract_id) => match param_contract_id.parse::<String>() {
                        Ok(param_contract_id) => param_contract_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter contractId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["contractId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_deliver_contract_request: Option<models::DeliverContractRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_deliver_contract_request) => param_deliver_contract_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.deliver_contract(
                                            param_contract_id,
                                            param_deliver_contract_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeliverContractResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELIVER_CONTRACT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter DeliverContractRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter DeliverContractRequest")),
                        }
            },

            // FulfillContract - POST /my/contracts/{contractId}/fulfill
            hyper::Method::POST if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_FULFILL) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_CONTRACTS_CONTRACTID_FULFILL
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_CONTRACTS_CONTRACTID_FULFILL in set but failed match against \"{}\"", path, paths::REGEX_MY_CONTRACTS_CONTRACTID_FULFILL.as_str())
                    );

                let param_contract_id = match percent_encoding::percent_decode(path_params["contractId"].as_bytes()).decode_utf8() {
                    Ok(param_contract_id) => match param_contract_id.parse::<String>() {
                        Ok(param_contract_id) => param_contract_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter contractId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["contractId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.fulfill_contract(
                                            param_contract_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                FulfillContractResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for FULFILL_CONTRACT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetContract - GET /my/contracts/{contractId}
            hyper::Method::GET if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_CONTRACTS_CONTRACTID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_CONTRACTS_CONTRACTID in set but failed match against \"{}\"", path, paths::REGEX_MY_CONTRACTS_CONTRACTID.as_str())
                    );

                let param_contract_id = match percent_encoding::percent_decode(path_params["contractId"].as_bytes()).decode_utf8() {
                    Ok(param_contract_id) => match param_contract_id.parse::<String>() {
                        Ok(param_contract_id) => param_contract_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter contractId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["contractId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_contract(
                                            param_contract_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetContractResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONTRACT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetContracts - GET /my/contracts
            hyper::Method::GET if path.matched(paths::ID_MY_CONTRACTS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_page = query_params.iter().filter(|e| e.0 == "page").map(|e| e.1.clone())
                    .next();
                let param_page = match param_page {
                    Some(param_page) => {
                        let param_page =
                            <i32 as std::str::FromStr>::from_str
                                (&param_page);
                        match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.clone())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_contracts(
                                            param_page,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetContractsResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONTRACTS_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // Register - POST /register
            hyper::Method::POST if path.matched(paths::ID_REGISTER) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_register_request: Option<models::RegisterRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_register_request) => param_register_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.register(
                                            param_register_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RegisterResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter RegisterRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter RegisterRequest")),
                        }
            },

            // GetFaction - GET /factions/{factionSymbol}
            hyper::Method::GET if path.matched(paths::ID_FACTIONS_FACTIONSYMBOL) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_FACTIONS_FACTIONSYMBOL
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FACTIONS_FACTIONSYMBOL in set but failed match against \"{}\"", path, paths::REGEX_FACTIONS_FACTIONSYMBOL.as_str())
                    );

                let param_faction_symbol = match percent_encoding::percent_decode(path_params["factionSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_faction_symbol) => match param_faction_symbol.parse::<String>() {
                        Ok(param_faction_symbol) => param_faction_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter factionSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["factionSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_faction(
                                            param_faction_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetFactionResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_FACTION_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetFactions - GET /factions
            hyper::Method::GET if path.matched(paths::ID_FACTIONS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_page = query_params.iter().filter(|e| e.0 == "page").map(|e| e.1.clone())
                    .next();
                let param_page = match param_page {
                    Some(param_page) => {
                        let param_page =
                            <i32 as std::str::FromStr>::from_str
                                (&param_page);
                        match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.clone())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_factions(
                                            param_page,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetFactionsResponse::Status200
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_FACTIONS_STATUS200"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // CreateChart - POST /my/ships/{shipSymbol}/chart
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_CHART) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_CHART
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_CHART in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_CHART.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.create_chart(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateChartResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_CHART_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // CreateShipShipScan - POST /my/ships/{shipSymbol}/scan/ships
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.create_ship_ship_scan(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateShipShipScanResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_SHIP_SHIP_SCAN_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // CreateShipSystemScan - POST /my/ships/{shipSymbol}/scan/systems
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.create_ship_system_scan(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateShipSystemScanResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_SHIP_SYSTEM_SCAN_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // CreateShipWaypointScan - POST /my/ships/{shipSymbol}/scan/waypoints
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.create_ship_waypoint_scan(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateShipWaypointScanResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_SHIP_WAYPOINT_SCAN_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // CreateSurvey - POST /my/ships/{shipSymbol}/survey
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SURVEY) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_SURVEY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_SURVEY in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_SURVEY.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.create_survey(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateSurveyResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_SURVEY_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DockShip - POST /my/ships/{shipSymbol}/dock
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_DOCK) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_DOCK
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_DOCK in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_DOCK.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.dock_ship(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DockShipResponse::TheShipHasSuccessfullyDockedAtIt
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DOCK_SHIP_THE_SHIP_HAS_SUCCESSFULLY_DOCKED_AT_IT"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ExtractResources - POST /my/ships/{shipSymbol}/extract
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_EXTRACT) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_EXTRACT
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_EXTRACT in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_EXTRACT.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_extract_resources_request: Option<models::ExtractResourcesRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_extract_resources_request) => param_extract_resources_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.extract_resources(
                                            param_ship_symbol,
                                            param_extract_resources_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ExtractResourcesResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for EXTRACT_RESOURCES_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ExtractResourcesRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ExtractResourcesRequest")),
                        }
            },

            // GetMyShip - GET /my/ships/{shipSymbol}
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_my_ship(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetMyShipResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MY_SHIP_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetMyShipCargo - GET /my/ships/{shipSymbol}/cargo
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_CARGO) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_CARGO
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_CARGO in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_CARGO.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_my_ship_cargo(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetMyShipCargoResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MY_SHIP_CARGO_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetMyShips - GET /my/ships
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_page = query_params.iter().filter(|e| e.0 == "page").map(|e| e.1.clone())
                    .next();
                let param_page = match param_page {
                    Some(param_page) => {
                        let param_page =
                            <i32 as std::str::FromStr>::from_str
                                (&param_page);
                        match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.clone())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_my_ships(
                                            param_page,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetMyShipsResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MY_SHIPS_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetShipCooldown - GET /my/ships/{shipSymbol}/cooldown
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_COOLDOWN) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_COOLDOWN
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_COOLDOWN in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_COOLDOWN.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_ship_cooldown(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetShipCooldownResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SHIP_COOLDOWN_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetShipCooldownResponse::NoCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetShipNav - GET /my/ships/{shipSymbol}/nav
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_NAV) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_NAV
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_NAV in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_NAV.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_ship_nav(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetShipNavResponse::TheCurrentNavStatusOfTheShip
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SHIP_NAV_THE_CURRENT_NAV_STATUS_OF_THE_SHIP"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // Jettison - POST /my/ships/{shipSymbol}/jettison
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_JETTISON) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_JETTISON
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_JETTISON in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_JETTISON.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_jettison_request: Option<models::JettisonRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_jettison_request) => param_jettison_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.jettison(
                                            param_ship_symbol,
                                            param_jettison_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                JettisonResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for JETTISON_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter JettisonRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter JettisonRequest")),
                        }
            },

            // JumpShip - POST /my/ships/{shipSymbol}/jump
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_JUMP) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_JUMP
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_JUMP in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_JUMP.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_jump_ship_request: Option<models::JumpShipRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_jump_ship_request) => param_jump_ship_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.jump_ship(
                                            param_ship_symbol,
                                            param_jump_ship_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                JumpShipResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for JUMP_SHIP_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter JumpShipRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter JumpShipRequest")),
                        }
            },

            // NavigateShip - POST /my/ships/{shipSymbol}/navigate
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_NAVIGATE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_NAVIGATE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_NAVIGATE in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_NAVIGATE.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_navigate_ship_request: Option<models::NavigateShipRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_navigate_ship_request) => param_navigate_ship_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.navigate_ship(
                                            param_ship_symbol,
                                            param_navigate_ship_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                NavigateShipResponse::TheSuccessfulTransitInformationIncludingTheRouteDetailsAndChangesToShipFuel
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for NAVIGATE_SHIP_THE_SUCCESSFUL_TRANSIT_INFORMATION_INCLUDING_THE_ROUTE_DETAILS_AND_CHANGES_TO_SHIP_FUEL"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter NavigateShipRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter NavigateShipRequest")),
                        }
            },

            // OrbitShip - POST /my/ships/{shipSymbol}/orbit
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_ORBIT) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_ORBIT
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_ORBIT in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_ORBIT.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.orbit_ship(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                OrbitShipResponse::TheShipHasSuccessfullyMovedIntoOrbitAtIt
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ORBIT_SHIP_THE_SHIP_HAS_SUCCESSFULLY_MOVED_INTO_ORBIT_AT_IT"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PatchShipNav - PATCH /my/ships/{shipSymbol}/nav
            hyper::Method::PATCH if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_NAV) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_NAV
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_NAV in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_NAV.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_patch_ship_nav_request: Option<models::PatchShipNavRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_patch_ship_nav_request) => param_patch_ship_nav_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.patch_ship_nav(
                                            param_ship_symbol,
                                            param_patch_ship_nav_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PatchShipNavResponse::TheUpdatedNavStatusOfTheShip
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_SHIP_NAV_THE_UPDATED_NAV_STATUS_OF_THE_SHIP"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter PatchShipNavRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter PatchShipNavRequest")),
                        }
            },

            // PurchaseCargo - POST /my/ships/{shipSymbol}/purchase
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_PURCHASE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_PURCHASE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_PURCHASE in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_PURCHASE.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_purchase_cargo_request: Option<models::PurchaseCargoRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_purchase_cargo_request) => param_purchase_cargo_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.purchase_cargo(
                                            param_ship_symbol,
                                            param_purchase_cargo_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PurchaseCargoResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PURCHASE_CARGO_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter PurchaseCargoRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter PurchaseCargoRequest")),
                        }
            },

            // PurchaseShip - POST /my/ships
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_purchase_ship_request: Option<models::PurchaseShipRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_purchase_ship_request) => param_purchase_ship_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.purchase_ship(
                                            param_purchase_ship_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PurchaseShipResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PURCHASE_SHIP_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter PurchaseShipRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter PurchaseShipRequest")),
                        }
            },

            // RefuelShip - POST /my/ships/{shipSymbol}/refuel
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_REFUEL) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_REFUEL
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_REFUEL in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_REFUEL.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.refuel_ship(
                                            param_ship_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RefuelShipResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REFUEL_SHIP_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SellCargo - POST /my/ships/{shipSymbol}/sell
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SELL) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_SELL
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_SELL in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_SELL.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_sell_cargo_request: Option<models::SellCargoRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_sell_cargo_request) => param_sell_cargo_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.sell_cargo(
                                            param_ship_symbol,
                                            param_sell_cargo_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SellCargoResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SELL_CARGO_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter SellCargoRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter SellCargoRequest")),
                        }
            },

            // ShipRefine - POST /my/ships/{shipSymbol}/refine
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_REFINE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_REFINE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_REFINE in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_REFINE.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_ship_refine_request: Option<models::ShipRefineRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_ship_refine_request) => param_ship_refine_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.ship_refine(
                                            param_ship_symbol,
                                            param_ship_refine_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ShipRefineResponse::TheShipHasSuccessfullyStartedRefining
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SHIP_REFINE_THE_SHIP_HAS_SUCCESSFULLY_STARTED_REFINING"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ShipRefineRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ShipRefineRequest")),
                        }
            },

            // TransferCargo - POST /my/ships/{shipSymbol}/transfer
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_TRANSFER) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_TRANSFER
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_TRANSFER in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_TRANSFER.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_transfer_cargo_request: Option<models::TransferCargoRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_transfer_cargo_request) => param_transfer_cargo_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.transfer_cargo(
                                            param_ship_symbol,
                                            param_transfer_cargo_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TransferCargoResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for TRANSFER_CARGO_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter TransferCargoRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter TransferCargoRequest")),
                        }
            },

            // WarpShip - POST /my/ships/{shipSymbol}/warp
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_WARP) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MY_SHIPS_SHIPSYMBOL_WARP
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_SHIPS_SHIPSYMBOL_WARP in set but failed match against \"{}\"", path, paths::REGEX_MY_SHIPS_SHIPSYMBOL_WARP.as_str())
                    );

                let param_ship_symbol = match percent_encoding::percent_decode(path_params["shipSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_ship_symbol) => match param_ship_symbol.parse::<String>() {
                        Ok(param_ship_symbol) => param_ship_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter shipSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["shipSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_navigate_ship_request: Option<models::NavigateShipRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_navigate_ship_request) => param_navigate_ship_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.warp_ship(
                                            param_ship_symbol,
                                            param_navigate_ship_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                WarpShipResponse::TheSuccessfulTransitInformationIncludingTheRouteDetailsAndChangesToShipFuel
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for WARP_SHIP_THE_SUCCESSFUL_TRANSIT_INFORMATION_INCLUDING_THE_ROUTE_DETAILS_AND_CHANGES_TO_SHIP_FUEL"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter NavigateShipRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter NavigateShipRequest")),
                        }
            },

            // GetJumpGate - GET /systems/{systemSymbol}/waypoints/{waypointSymbol}/jump-gate
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE in set but failed match against \"{}\"", path, paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE.as_str())
                    );

                let param_system_symbol = match percent_encoding::percent_decode(path_params["systemSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_system_symbol) => match param_system_symbol.parse::<String>() {
                        Ok(param_system_symbol) => param_system_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter systemSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["systemSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_waypoint_symbol = match percent_encoding::percent_decode(path_params["waypointSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_waypoint_symbol) => match param_waypoint_symbol.parse::<String>() {
                        Ok(param_waypoint_symbol) => param_waypoint_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter waypointSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["waypointSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_jump_gate(
                                            param_system_symbol,
                                            param_waypoint_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetJumpGateResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_JUMP_GATE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetMarket - GET /systems/{systemSymbol}/waypoints/{waypointSymbol}/market
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET in set but failed match against \"{}\"", path, paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET.as_str())
                    );

                let param_system_symbol = match percent_encoding::percent_decode(path_params["systemSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_system_symbol) => match param_system_symbol.parse::<String>() {
                        Ok(param_system_symbol) => param_system_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter systemSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["systemSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_waypoint_symbol = match percent_encoding::percent_decode(path_params["waypointSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_waypoint_symbol) => match param_waypoint_symbol.parse::<String>() {
                        Ok(param_waypoint_symbol) => param_waypoint_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter waypointSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["waypointSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_market(
                                            param_system_symbol,
                                            param_waypoint_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetMarketResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MARKET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetShipyard - GET /systems/{systemSymbol}/waypoints/{waypointSymbol}/shipyard
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD in set but failed match against \"{}\"", path, paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD.as_str())
                    );

                let param_system_symbol = match percent_encoding::percent_decode(path_params["systemSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_system_symbol) => match param_system_symbol.parse::<String>() {
                        Ok(param_system_symbol) => param_system_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter systemSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["systemSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_waypoint_symbol = match percent_encoding::percent_decode(path_params["waypointSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_waypoint_symbol) => match param_waypoint_symbol.parse::<String>() {
                        Ok(param_waypoint_symbol) => param_waypoint_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter waypointSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["waypointSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_shipyard(
                                            param_system_symbol,
                                            param_waypoint_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetShipyardResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SHIPYARD_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetSystem - GET /systems/{systemSymbol}
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_SYSTEMS_SYSTEMSYMBOL
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SYSTEMS_SYSTEMSYMBOL in set but failed match against \"{}\"", path, paths::REGEX_SYSTEMS_SYSTEMSYMBOL.as_str())
                    );

                let param_system_symbol = match percent_encoding::percent_decode(path_params["systemSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_system_symbol) => match param_system_symbol.parse::<String>() {
                        Ok(param_system_symbol) => param_system_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter systemSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["systemSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_system(
                                            param_system_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetSystemResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SYSTEM_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetSystemWaypoints - GET /systems/{systemSymbol}/waypoints
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SYSTEMS_SYSTEMSYMBOL_WAYPOINTS in set but failed match against \"{}\"", path, paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS.as_str())
                    );

                let param_system_symbol = match percent_encoding::percent_decode(path_params["systemSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_system_symbol) => match param_system_symbol.parse::<String>() {
                        Ok(param_system_symbol) => param_system_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter systemSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["systemSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_page = query_params.iter().filter(|e| e.0 == "page").map(|e| e.1.clone())
                    .next();
                let param_page = match param_page {
                    Some(param_page) => {
                        let param_page =
                            <i32 as std::str::FromStr>::from_str
                                (&param_page);
                        match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.clone())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_system_waypoints(
                                            param_system_symbol,
                                            param_page,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetSystemWaypointsResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SYSTEM_WAYPOINTS_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetSystems - GET /systems
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_page = query_params.iter().filter(|e| e.0 == "page").map(|e| e.1.clone())
                    .next();
                let param_page = match param_page {
                    Some(param_page) => {
                        let param_page =
                            <i32 as std::str::FromStr>::from_str
                                (&param_page);
                        match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.clone())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_systems(
                                            param_page,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetSystemsResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SYSTEMS_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetWaypoint - GET /systems/{systemSymbol}/waypoints/{waypointSymbol}
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL in set but failed match against \"{}\"", path, paths::REGEX_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL.as_str())
                    );

                let param_system_symbol = match percent_encoding::percent_decode(path_params["systemSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_system_symbol) => match param_system_symbol.parse::<String>() {
                        Ok(param_system_symbol) => param_system_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter systemSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["systemSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_waypoint_symbol = match percent_encoding::percent_decode(path_params["waypointSymbol"].as_bytes()).decode_utf8() {
                    Ok(param_waypoint_symbol) => match param_waypoint_symbol.parse::<String>() {
                        Ok(param_waypoint_symbol) => param_waypoint_symbol,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter waypointSymbol: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["waypointSymbol"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_waypoint(
                                            param_system_symbol,
                                            param_waypoint_symbol,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetWaypointResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_WAYPOINT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_FACTIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_FACTIONS_FACTIONSYMBOL) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_AGENT) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_CONTRACTS) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_ACCEPT) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_DELIVER) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_FULFILL) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_CARGO) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_CHART) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_COOLDOWN) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_DOCK) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_EXTRACT) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_JETTISON) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_JUMP) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_NAV) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_NAVIGATE) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_ORBIT) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_PURCHASE) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_REFINE) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_REFUEL) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SELL) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SURVEY) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_TRANSFER) => method_not_allowed(),
            _ if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_WARP) => method_not_allowed(),
            _ if path.matched(paths::ID_REGISTER) => method_not_allowed(),
            _ if path.matched(paths::ID_SYSTEMS) => method_not_allowed(),
            _ if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL) => method_not_allowed(),
            _ if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS) => method_not_allowed(),
            _ if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL) => method_not_allowed(),
            _ if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE) => method_not_allowed(),
            _ if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET) => method_not_allowed(),
            _ if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // GetMyAgent - GET /my/agent
            hyper::Method::GET if path.matched(paths::ID_MY_AGENT) => Some("GetMyAgent"),
            // AcceptContract - POST /my/contracts/{contractId}/accept
            hyper::Method::POST if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_ACCEPT) => Some("AcceptContract"),
            // DeliverContract - POST /my/contracts/{contractId}/deliver
            hyper::Method::POST if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_DELIVER) => Some("DeliverContract"),
            // FulfillContract - POST /my/contracts/{contractId}/fulfill
            hyper::Method::POST if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID_FULFILL) => Some("FulfillContract"),
            // GetContract - GET /my/contracts/{contractId}
            hyper::Method::GET if path.matched(paths::ID_MY_CONTRACTS_CONTRACTID) => Some("GetContract"),
            // GetContracts - GET /my/contracts
            hyper::Method::GET if path.matched(paths::ID_MY_CONTRACTS) => Some("GetContracts"),
            // Register - POST /register
            hyper::Method::POST if path.matched(paths::ID_REGISTER) => Some("Register"),
            // GetFaction - GET /factions/{factionSymbol}
            hyper::Method::GET if path.matched(paths::ID_FACTIONS_FACTIONSYMBOL) => Some("GetFaction"),
            // GetFactions - GET /factions
            hyper::Method::GET if path.matched(paths::ID_FACTIONS) => Some("GetFactions"),
            // CreateChart - POST /my/ships/{shipSymbol}/chart
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_CHART) => Some("CreateChart"),
            // CreateShipShipScan - POST /my/ships/{shipSymbol}/scan/ships
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_SHIPS) => Some("CreateShipShipScan"),
            // CreateShipSystemScan - POST /my/ships/{shipSymbol}/scan/systems
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_SYSTEMS) => Some("CreateShipSystemScan"),
            // CreateShipWaypointScan - POST /my/ships/{shipSymbol}/scan/waypoints
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SCAN_WAYPOINTS) => Some("CreateShipWaypointScan"),
            // CreateSurvey - POST /my/ships/{shipSymbol}/survey
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SURVEY) => Some("CreateSurvey"),
            // DockShip - POST /my/ships/{shipSymbol}/dock
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_DOCK) => Some("DockShip"),
            // ExtractResources - POST /my/ships/{shipSymbol}/extract
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_EXTRACT) => Some("ExtractResources"),
            // GetMyShip - GET /my/ships/{shipSymbol}
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL) => Some("GetMyShip"),
            // GetMyShipCargo - GET /my/ships/{shipSymbol}/cargo
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_CARGO) => Some("GetMyShipCargo"),
            // GetMyShips - GET /my/ships
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS) => Some("GetMyShips"),
            // GetShipCooldown - GET /my/ships/{shipSymbol}/cooldown
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_COOLDOWN) => Some("GetShipCooldown"),
            // GetShipNav - GET /my/ships/{shipSymbol}/nav
            hyper::Method::GET if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_NAV) => Some("GetShipNav"),
            // Jettison - POST /my/ships/{shipSymbol}/jettison
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_JETTISON) => Some("Jettison"),
            // JumpShip - POST /my/ships/{shipSymbol}/jump
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_JUMP) => Some("JumpShip"),
            // NavigateShip - POST /my/ships/{shipSymbol}/navigate
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_NAVIGATE) => Some("NavigateShip"),
            // OrbitShip - POST /my/ships/{shipSymbol}/orbit
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_ORBIT) => Some("OrbitShip"),
            // PatchShipNav - PATCH /my/ships/{shipSymbol}/nav
            hyper::Method::PATCH if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_NAV) => Some("PatchShipNav"),
            // PurchaseCargo - POST /my/ships/{shipSymbol}/purchase
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_PURCHASE) => Some("PurchaseCargo"),
            // PurchaseShip - POST /my/ships
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS) => Some("PurchaseShip"),
            // RefuelShip - POST /my/ships/{shipSymbol}/refuel
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_REFUEL) => Some("RefuelShip"),
            // SellCargo - POST /my/ships/{shipSymbol}/sell
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_SELL) => Some("SellCargo"),
            // ShipRefine - POST /my/ships/{shipSymbol}/refine
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_REFINE) => Some("ShipRefine"),
            // TransferCargo - POST /my/ships/{shipSymbol}/transfer
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_TRANSFER) => Some("TransferCargo"),
            // WarpShip - POST /my/ships/{shipSymbol}/warp
            hyper::Method::POST if path.matched(paths::ID_MY_SHIPS_SHIPSYMBOL_WARP) => Some("WarpShip"),
            // GetJumpGate - GET /systems/{systemSymbol}/waypoints/{waypointSymbol}/jump-gate
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_JUMP_GATE) => Some("GetJumpGate"),
            // GetMarket - GET /systems/{systemSymbol}/waypoints/{waypointSymbol}/market
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_MARKET) => Some("GetMarket"),
            // GetShipyard - GET /systems/{systemSymbol}/waypoints/{waypointSymbol}/shipyard
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL_SHIPYARD) => Some("GetShipyard"),
            // GetSystem - GET /systems/{systemSymbol}
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL) => Some("GetSystem"),
            // GetSystemWaypoints - GET /systems/{systemSymbol}/waypoints
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS) => Some("GetSystemWaypoints"),
            // GetSystems - GET /systems
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS) => Some("GetSystems"),
            // GetWaypoint - GET /systems/{systemSymbol}/waypoints/{waypointSymbol}
            hyper::Method::GET if path.matched(paths::ID_SYSTEMS_SYSTEMSYMBOL_WAYPOINTS_WAYPOINTSYMBOL) => Some("GetWaypoint"),
            _ => None,
        }
    }
}
