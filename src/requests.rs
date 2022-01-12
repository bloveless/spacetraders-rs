//! All requests being sent to the api are in this module
use crate::shared;
use serde::Serialize;

/// The representation of a purchase order request
#[derive(Serialize, Debug)]
pub struct PurchaseOrderRequest {
    /// The id of the ship to load the good into
    #[serde(rename(serialize = "shipId"))]
    pub ship_id: String,
    /// The type of good being purchased
    pub good: shared::Good,
    /// The quantity of good being purchased
    pub quantity: i32,
}

/// The representation of a purchase ship request
#[derive(Serialize, Debug)]
pub struct PurchaseShipRequest {
    /// The location to buy the ship from
    pub location: String,
    /// The type of ship to buy
    #[serde(rename(serialize = "type"))]
    pub ship_type: String,
}

/// The representation of a new loan request
#[derive(Serialize, Debug)]
pub struct RequestNewLoanRequest {
    /// The type of loan being requested
    #[serde(rename(serialize = "type"))]
    pub loan_type: shared::LoanType,
}

/// The representation of a flight plan request
#[derive(Serialize, Debug)]
pub struct FlightPlanRequest {
    /// The id of the ship to send to the destination
    #[serde(rename(serialize = "shipId"))]
    pub ship_id: String,
    /// The destination to send the ship to
    pub destination: String,
}

/// The representation of a sell order request
#[derive(Serialize, Debug)]
pub struct SellOrderRequest {
    /// The id of the ship which contains the good to be sold
    #[serde(rename(serialize = "shipId"))]
    pub ship_id: String,
    /// The good to be sold
    pub good: shared::Good,
    /// The quantity of good to sell
    pub quantity: i32,
}

/// The representation of a jettison cargo request
#[derive(Serialize, Debug)]
pub struct JettisonCargo {
    /// The good to be jettisoned
    pub good: shared::Good,
    /// The quantity of good to jettison
    pub quantity: i32,
}

/// The representation of a warp jump request
#[derive(Serialize, Debug)]
pub struct WarpJump {
    /// The ship id to attempt a warp jump
    #[serde(rename(serialize = "shipId"))]
    pub ship_id: String,
}
