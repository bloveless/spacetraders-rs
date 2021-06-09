//! All responses that come back from the API are in this module
use crate::shared;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::shared::Loan;

/// My Ip address response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MyIpAddress {
    /// My ip address
    pub ip: String,
}

/// A representation of the game status
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct GameStatus {
    /// The status of the games API servers
    pub status: String,
}

/// A representation of user info
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct UserInfoData {
    /// The users name
    pub username: String,
    /// The credits available to the user
    pub credits: i32,
    /// The number of ships owned by the user
    #[serde(rename = "shipCount")]
    pub ship_count: i32,
    /// The number of structures owned by the user
    #[serde(rename = "structureCount")]
    pub structure_count: i32,
    /// The date the user joined
    #[serde(rename = "joinedAt")]
    pub joined_at: DateTime<Utc>,
}

/// A representation of a user info request
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct UserInfo {
    /// The user info
    pub user: UserInfoData,
}

/// A representation of an available loan
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AvailableLoan {
    /// The loan type
    #[serde(rename = "type")]
    pub loan_type: shared::LoanType,
    /// The amount the user will receive from the loan
    pub amount: i32,
    /// The payback rate of the loan
    pub rate: f64,
    /// The loan term length
    #[serde(rename = "termInDays")]
    pub term_in_days: i32,
    /// Does this loan require some collateral or not
    #[serde(rename = "collateralRequired")]
    pub collateral_required: bool,
}

/// The representation of an available loans request
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AvailableLoans {
    /// List of loans that are available
    pub loans: Vec<AvailableLoan>,
}

/// A representation of a request loan response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct RequestLoan {
    /// The users new credits
    pub credits: i32,
    /// The loan granted information
    pub loan: shared::Loan,
}

/// The user associated with a pay loan response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct PayLoanUser {
    /// The users username
    pub username: String,
    /// The most up to date users credits
    pub credits: i32,
    /// The users ships
    pub ships: Vec<shared::Ship>,
    /// The users loans
    pub loans: Vec<Loan>,
}

/// A representation of a pay loan response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct PayLoanResponse {
    /// Users new credits after paying off the loan
    pub credits: i32,
    /// Users new loans after paying off the loan
    pub loans: Vec<shared::Loan>,
}

/// The representation of a ships for sale request
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipsForSale {
    /// The ships that are for sale
    pub ships: Vec<shared::ShipForSale>,
}

/// The representation of a purchase order response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct PurchaseOrder {
    /// The users new credits
    pub credits: i32,
    /// The order that was just created
    pub order: shared::Order,
    /// The updated ship information
    pub ship: shared::Ship,
}

/// The representation of the available locations response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct AvailableLocations {
    /// A list of locations that are available
    pub locations: Vec<shared::Location>,
}

/// The representation of a flight plan response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct FlightPlan {
    /// A flight plan
    #[serde(rename = "flightPlan")]
    pub flight_plan: shared::FlightPlanData,
}

/// The representation of a systems info response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsInfo {
    /// The list of systems info
    pub systems: Vec<shared::SystemsInfoData>,
}

/// The representation of a user
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct User {
    /// The users id
    pub id: String,
    /// The users username
    pub username: String,
    /// The users picture
    pub picture: Option<String>,
    /// The users email
    pub email: Option<String>,
    /// The users credits
    pub credits: i32,
    /// The date the user was created
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// The last time the user was updated
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

/// A representation of user info
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ClaimUsernameUser {
    /// The users name
    pub username: String,
    /// The credits available to the user
    pub credits: i32,
    /// The users ships
    pub ships: Vec<shared::Ship>,
    /// The users loans
    pub loans: Vec<shared::Loan>,
}

/// The representation of a claim username response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ClaimUsername {
    /// The token of the claimed username
    pub token: String,
    /// The user info of the claimed username
    pub user: ClaimUsernameUser,
}

/// The representation of the current user ships response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MyShips {
    /// The list of ships attached to the current user account
    pub ships: Vec<shared::Ship>,
}

/// The representation of a specific current user ship response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MyShip {
    /// The list of ships attached to the current user account
    pub ship: shared::Ship,
}

/// The representation of a location response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct LocationInfo {
    /// The location info
    pub location: shared::SystemsInfoLocation,
    #[serde(rename = "dockedShips")]
    /// The number of ships docked at the location
    pub docked_ships: i32,
}

/// The representation of your loan response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct LoanInfo {
    /// The list of loans attached to the current user account
    pub loans: Vec<shared::Loan>,
}

/// The representation of the location marketplace return
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct LocationMarketplace {
    /// The marketplace data for the requested location
    pub marketplace: Vec<shared::MarketplaceData>,
}

/// The representation of a purchase ship response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct PurchaseShip {
    /// The credits a user has after purchasing a ship
    pub credits: i32,
    /// The current data about the ship that was purchased
    pub ship: shared::Ship,
}

/// The representation of a jettison cargo response
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct JettisonCargo {
    /// The id of the ship that cargo was jettisoned from
    #[serde(rename = "shipId")]
    pub ship_id: String,
    /// The good which was jettisoned
    pub good: shared::Good,
    /// The quantity remaining of the good which was jettisoned
    #[serde(rename = "quantityRemaining")]
    pub quantity_remaining: i32,
}
