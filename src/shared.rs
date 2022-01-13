//! The shared module contains all common structs and enums used in the API
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

/// The various loan types
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub enum LoanType {
    /// Startup
    #[serde(rename = "STARTUP")]
    Startup,
    /// Enterprise
    #[serde(rename = "ENTERPRISE")]
    Enterprise,
}

/// The various goods in the game
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub enum Good {
    /// Fuel
    #[serde(rename = "FUEL")]
    Fuel,
    /// Chemicals
    #[serde(rename = "CHEMICALS")]
    Chemicals,
    /// Metals
    #[serde(rename = "METALS")]
    Metals,
    /// Drones
    #[serde(rename = "DRONES")]
    Drones,
    /// Food
    #[serde(rename = "FOOD")]
    Food,
    /// Consumer goods
    #[serde(rename = "CONSUMER_GOODS")]
    ConsumerGoods,
    /// Explosives
    #[serde(rename = "EXPLOSIVES")]
    Explosives,
    /// Narcotics
    #[serde(rename = "NARCOTICS")]
    Narcotics,
    /// Textiles
    #[serde(rename = "TEXTILES")]
    Textiles,
    /// Electronics
    #[serde(rename = "ELECTRONICS")]
    Electronics,
    /// Machinery
    #[serde(rename = "MACHINERY")]
    Machinery,
    /// Construction materials
    #[serde(rename = "CONSTRUCTION_MATERIALS")]
    ConstructionMaterials,
    /// Ship plating
    #[serde(rename = "SHIP_PLATING")]
    ShipPlating,
    /// Rare metals
    #[serde(rename = "RARE_METALS")]
    RareMetals,
    /// Protein Synthesizers
    #[serde(rename = "PROTEIN_SYNTHESIZERS")]
    ProteinSynthesizers,
    /// Research
    #[serde(rename = "RESEARCH")]
    Research,
    /// Precision Instruments
    #[serde(rename = "PRECISION_INSTRUMENTS")]
    PrecisionInstruments,
    /// Nanobots
    #[serde(rename = "NANOBOTS")]
    Nanobots,
    /// Biometric Firearms
    #[serde(rename = "BIOMETRIC_FIREARMS")]
    BiometricFirearms,
    /// Ship parts
    #[serde(rename = "SHIP_PARTS")]
    ShipParts,
    /// Exotic Plasma
    #[serde(rename = "EXOTIC_PLASMA")]
    ExoticPlasma,
    /// Fusion Reactors
    #[serde(rename = "FUSION_REACTORS")]
    FusionReactors,
    /// Zuco Crystals
    #[serde(rename = "ZUCO_CRYSTALS")]
    ZucoCrystals,
    /// Unstable Compounds
    #[serde(rename = "UNSTABLE_COMPOUNDS")]
    UnstableCompounds,
    /// Unknown is used when a string can't be converted to a known good
    Unknown,
}

impl Good {
    /// Get the volume consumed by a good
    pub fn get_volume(&self) -> i32 {
        match *self {
            Good::Fuel => 1,
            Good::Chemicals => 1,
            Good::Metals => 1,
            Good::Drones => 1,
            Good::Food => 1,
            Good::ConsumerGoods => 1,
            Good::Explosives => 1,
            Good::Narcotics => 1,
            Good::Textiles => 1,
            Good::Electronics => 1,
            Good::Machinery => 2,
            Good::ConstructionMaterials => 1,
            Good::ShipPlating => 2,
            Good::RareMetals => 1,
            Good::ProteinSynthesizers => 1,
            Good::Research => 1,
            Good::PrecisionInstruments => 1,
            Good::Nanobots => 1,
            Good::BiometricFirearms => 1,
            Good::ShipParts => 4,
            Good::ExoticPlasma => 4,
            Good::FusionReactors => 6,
            Good::ZucoCrystals => 1,
            Good::UnstableCompounds => 3,
            Good::Unknown => 1,
        }
    }
}

impl From<String> for Good {
    fn from(good: String) -> Self {
        match good.as_str() {
            "Metals" => Good::Metals,
            "RareMetals" => Good::RareMetals,
            "Chemicals" => Good::Chemicals,
            "Fuel" => Good::Fuel,
            "Food" => Good::Food,
            "Drones" => Good::Drones,
            "Textiles" => Good::Textiles,
            "ConsumerGoods" => Good::ConsumerGoods,
            "Machinery" => Good::Machinery,
            "ConstructionMaterials" => Good::ConstructionMaterials,
            "Electronics" => Good::Electronics,
            "Research" => Good::Research,
            "ShipParts" => Good::ShipParts,
            "ShipPlating" => Good::ShipPlating,
            "FusionReactors" => Good::FusionReactors,
            "ExoticPlasma" => Good::ExoticPlasma,
            "UnstableCompounds" => Good::UnstableCompounds,
            "ProteinSynthesizers" => Good::ProteinSynthesizers,
            "BiometricFirearms" => Good::BiometricFirearms,
            "Explosives" => Good::Explosives,
            "Nanobots" => Good::Nanobots,
            "PrecisionInstruments" => Good::PrecisionInstruments,
            "Narcotics" => Good::Narcotics,
            _ => Good::Unknown,
        }
    }
}

impl fmt::Display for Good {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The various location types
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub enum LocationType {
    /// A planet
    #[serde(rename = "PLANET")]
    Planet,
    /// A moon
    #[serde(rename = "MOON")]
    Moon,
    /// A Gas Giant
    #[serde(rename = "GAS_GIANT")]
    GasGiant,
    /// An asteroid
    #[serde(rename = "ASTEROID")]
    Asteroid,
    /// A Wormhole
    #[serde(rename = "WORMHOLE")]
    Wormhole,
    /// A nebula
    #[serde(rename = "NEBULA")]
    Nebula,
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A representation of a users ship
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Ship {
    /// The ships id
    pub id: String,
    /// The current location of the ship or None if the ship is in transit
    pub location: Option<String>,
    /// Any cargo within the ship
    pub cargo: Vec<Cargo>,
    /// The volume available in the ships cargo
    #[serde(rename = "spaceAvailable")]
    pub space_available: i32,
    /// The type of the ship
    #[serde(rename = "type")]
    pub ship_type: String,
    /// The class of the ship
    pub class: String,
    /// The maximum cargo volume of the ship
    #[serde(rename = "maxCargo")]
    pub max_cargo: i32,
    /// The speed rating of the ship
    pub speed: i32,
    /// The manufacturer of the ship
    pub manufacturer: String,
    /// The defensive rating of the ship
    pub plating: i32,
    /// The offensive rating of the ship
    pub weapons: i32,
    /// The ships current X coordinate
    pub x: Option<i32>,
    /// The ships current Y coordinate
    pub y: Option<i32>,
    #[serde(rename = "flightPlanId")]
    /// The ships current flight plan
    pub flight_plan_id: Option<String>,
    #[serde(rename = "loadingSpeed")]
    /// The loading speed of the ship
    pub loading_speed: i32,
    /// The goods that this ship is restricted to carrying
    #[serde(rename = "restrictedGoods")]
    pub restricted_goods: Option<Vec<Good>>,
}

/// A representation of cargo within a ship
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Cargo {
    /// The good in the cargo
    pub good: Good,
    /// The quantity of the good
    pub quantity: i32,
    /// The total volume consumed by the good
    #[serde(rename = "totalVolume")]
    pub total_volume: i32,
}

/// A representation of a purchase order
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Order {
    /// The good that was purchased
    pub good: Good,
    /// The quantity of the good which was purchased
    pub quantity: i32,
    /// The price per unit of the good at the time of purchase
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: i32,
    /// The total amount of the order
    pub total: i32,
}

/// A representation of a loan
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Loan {
    /// The id of the loan
    pub id: String,
    /// The due date of the loan
    pub due: DateTime<Utc>,
    /// The repayment amount of the loan
    #[serde(rename = "repaymentAmount")]
    pub repayment_amount: i32,
    /// The current loan status
    pub status: String,
    /// The type of the loan
    #[serde(rename = "type")]
    pub loan_type: LoanType,
}

/// A representation of a purchase location for a ship for sale
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct PurchaseLocation {
    /// The system of the ship for sale
    pub system: String,
    /// The location of the ship for sale
    pub location: String,
    /// The price of the ship at this location
    pub price: i32,
}

/// A representation of a ship for sales
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ShipForSale {
    /// The type of the ship
    #[serde(rename = "type")]
    pub ship_type: String,
    /// The class of the ship
    pub class: String,
    /// The maximum cargo volume of the ship
    #[serde(rename = "maxCargo")]
    pub max_cargo: i32,
    /// The ships speed rating
    pub speed: i32,
    /// The manufacturer of the ship
    pub manufacturer: String,
    /// The ships defensive rating
    pub plating: i32,
    /// The weapons rating of the ship
    pub weapons: i32,
    /// The locations that this ship can be purchased at
    #[serde(rename = "purchaseLocations")]
    pub purchase_locations: Vec<PurchaseLocation>,
    /// The goods that this ship is restricted to carrying
    #[serde(rename = "restrictedGoods")]
    pub restricted_goods: Option<Vec<Good>>,
    /// The loading speed of the ship
    #[serde(rename = "loadingSpeed")]
    pub loading_speed: i32,
}

/// A representation of a location in a system
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Location {
    /// The system symbol for this location
    pub symbol: String,
    /// The type of this location
    #[serde(rename = "type")]
    pub location_type: String,
    /// The friendly name of this location
    pub name: String,
    /// X coordinate of the location
    pub x: i32,
    /// Y coordinate of the location
    pub y: i32,
}

/// A representation of an error message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ErrorMessageData {
    /// The API sent error code
    pub code: i32,
    /// The message sent from the API about the error
    pub message: String,
}

/// A representation of a single flight plan
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct FlightPlanData {
    /// The id of the flight plan
    pub id: String,
    /// The id of the ship on this flight plan
    #[serde(rename = "shipId")]
    pub ship_id: String,
    /// The fuel consumed by this flight plan
    #[serde(rename = "fuelConsumed")]
    pub fuel_consumed: i32,
    /// The fuel remaining after the flight plan completes
    #[serde(rename = "fuelRemaining")]
    pub fuel_remaining: i32,
    /// The time remaining until the ship arrives at the destination in seconds
    #[serde(rename = "timeRemainingInSeconds")]
    pub time_remaining_in_seconds: i32,
    /// The date time that the flight plan was created at
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// The DateTime at which the ship will arrive
    #[serde(rename = "arrivesAt")]
    pub arrives_at: DateTime<Utc>,
    /// The termination DateTime of the flight plan. I believe this could be before the arrives at
    /// date if the flight plan is cancelled mid flight (which might not be developed yet)
    #[serde(rename = "terminatedAt")]
    pub terminated_at: Option<DateTime<Utc>>,
    /// The ships destination
    pub destination: String,
    /// The ships departure location
    pub departure: String,
    /// The distance of the flight plan
    pub distance: i32,
}

/// The structures that exist at a location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Structures {
    /// The id of the structure
    pub id: String,
    /// The structure type
    #[serde(rename = "type")]
    pub structure_type: String,
    /// The structure location
    pub location: String,
}

/// A representation of a location within a system
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsInfoLocation {
    /// The system symbol of this location
    pub symbol: String,
    /// The type of this system location
    #[serde(rename = "type")]
    pub systems_info_type: LocationType,
    /// The friendly name of the system location
    pub name: String,
    /// X coordinate of the system location
    pub x: i32,
    /// Y coordinate of the system location
    pub y: i32,
    /// The current ansible progress. I believe when this gets to 1 the anomaly opens.
    #[serde(rename = "ansibleProgress")]
    pub ansible_progress: Option<f64>,
    /// The anomaly info about this sytem location
    pub anomaly: Option<String>,
    /// The structures within a system location
    pub structures: Option<Vec<Structures>>,
    /// Any messages relating to this system location
    pub messages: Option<Vec<String>>,
    /// Whether or not the system allows construction
    #[serde(rename = "allowsConstruction")]
    pub allows_construction: bool,
}

/// The representation of a system
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemsInfoData {
    /// The system symbol
    pub symbol: String,
    /// The friendly name of the system
    pub name: String,
    /// Locations within the system
    pub locations: Vec<SystemsInfoLocation>,
}

/// The representation of a single good within a marketplace
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct MarketplaceData {
    /// The type of good for this marketplace data
    pub symbol: Good,
    /// How much volume this good consumes
    #[serde(rename = "volumePerUnit")]
    pub volume_per_unit: i32,
    /// The price per unit of the good
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: i32,
    /// The purchase price per unit of a good
    #[serde(rename = "purchasePricePerUnit")]
    pub purchase_price_per_unit: i32,
    /// The sell price per unit of a good
    #[serde(rename = "sellPricePerUnit")]
    pub sell_price_per_unit: i32,
    /// How much of the good is available at this location
    #[serde(rename = "quantityAvailable")]
    pub quantity_available: i32,
    /// DEPRECATED: Spread
    pub spread: i32,
}

/// Available structure types
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub enum StructureType {
    /// Fuel Refinery
    #[serde(rename = "FUEL_REFINERY")]
    FuelRefinery,
    /// Warp Gate
    #[serde(rename = "WARP_GATE")]
    WarpGate,
    /// Mine
    #[serde(rename = "MINE")]
    Mine,
    /// Chemical Plant
    #[serde(rename = "CHEMICAL_PLANT")]
    ChemicalPlant,
    /// Farm
    #[serde(rename = "FARM")]
    Farm,
}

/// A representation of structure found in marketplace data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct StructureOwnedBy {
    /// The user who owns the structure
    pub username: String,
}

/// An error response returned from the API
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ErrorMessage {
    /// The data about the error
    pub error: ErrorMessageData,
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error Code: {} Error Message: {}",
            self.error.code, self.error.message
        )
    }
}

impl Error for ErrorMessage {}
