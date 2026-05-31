use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseContentEntityError {
    pub message: String,
    pub code: u32,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseContentEntity {
    pub error: ResponseContentEntityError,
}

#[derive(Debug, Clone)]
pub struct ResponseContent {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<ResponseContentEntity>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum FactionSymbol {
    #[default]
    #[serde(rename = "COSMIC")]
    Cosmic,
    #[serde(rename = "VOID")]
    Void,
    #[serde(rename = "GALACTIC")]
    Galactic,
    #[serde(rename = "QUANTUM")]
    Quantum,
    #[serde(rename = "DOMINION")]
    Dominion,
    #[serde(rename = "ASTRO")]
    Astro,
    #[serde(rename = "CORSAIRS")]
    Corsairs,
    #[serde(rename = "OBSIDIAN")]
    Obsidian,
    #[serde(rename = "AEGIS")]
    Aegis,
    #[serde(rename = "UNITED")]
    United,
    #[serde(rename = "SOLITARY")]
    Solitary,
    #[serde(rename = "COBALT")]
    Cobalt,
    #[serde(rename = "OMEGA")]
    Omega,
    #[serde(rename = "ECHO")]
    Echo,
    #[serde(rename = "LORDS")]
    Lords,
    #[serde(rename = "CULT")]
    Cult,
    #[serde(rename = "ANCIENTS")]
    Ancients,
    #[serde(rename = "SHADOW")]
    Shadow,
    #[serde(rename = "ETHEREAL")]
    Ethereal,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterRequest {
    /// The faction you choose determines your headquarters.
    #[serde(rename = "faction")]
    pub faction: FactionSymbol,
    /// Your desired agent symbol. This will be a unique name used to represent your agent, and will be the prefix for your ships.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Your email address. This is used if you reserved your call sign between resets.
    pub email: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Agent {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The headquarters of the agent.
    #[serde(rename = "headquarters")]
    pub headquarters: String,
    /// The number of credits the agent has available. Credits can be negative if funds have been overdrawn.
    #[serde(rename = "credits")]
    pub credits: i32,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Default, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ContractType {
    #[default]
    #[serde(rename = "PROCUREMENT")]
    Procurement,
    #[serde(rename = "TRANSPORT")]
    Transport,
    #[serde(rename = "SHUTTLE")]
    Shuttle,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractPayment {
    /// The amount of credits received up front for accepting the contract.
    #[serde(rename = "onAccepted")]
    pub on_accepted: i32,
    /// The amount of credits received when the contract is fulfilled.
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: i32,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractDeliverGood {
    /// The symbol of the trade good to deliver.
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    /// The destination where goods need to be delivered.
    #[serde(rename = "destinationSymbol")]
    pub destination_symbol: String,
    /// The number of units that need to be delivered on this contract.
    #[serde(rename = "unitsRequired")]
    pub units_required: i32,
    /// The number of units fulfilled on this contract.
    #[serde(rename = "unitsFulfilled")]
    pub units_fulfilled: i32,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractTerms {
    /// The deadline for the contract.
    #[serde(rename = "deadline")]
    pub deadline: String,
    #[serde(rename = "payment")]
    pub payment: ContractPayment,
    #[serde(rename = "deliver", skip_serializing_if = "Option::is_none")]
    pub deliver: Option<Vec<ContractDeliverGood>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Contract {
    #[serde(rename = "id")]
    pub id: String,
    /// The symbol of the faction that this contract is for.
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: ContractType,
    #[serde(rename = "terms")]
    pub terms: ContractTerms,
    /// Whether the contract has been accepted by the agent
    #[serde(rename = "accepted")]
    pub accepted: bool,
    /// Whether the contract has been fulfilled
    #[serde(rename = "fulfilled")]
    pub fulfilled: bool,
    /// The time at which the contract expires
    #[serde(rename = "expiration")]
    pub expiration: String,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ShipRole {
    #[default]
    #[serde(rename = "FABRICATOR")]
    Fabricator,
    #[serde(rename = "HARVESTER")]
    Harvester,
    #[serde(rename = "HAULER")]
    Hauler,
    #[serde(rename = "INTERCEPTOR")]
    Interceptor,
    #[serde(rename = "EXCAVATOR")]
    Excavator,
    #[serde(rename = "TRANSPORT")]
    Transport,
    #[serde(rename = "REPAIR")]
    Repair,
    #[serde(rename = "SURVEYOR")]
    Surveyor,
    #[serde(rename = "COMMAND")]
    Command,
    #[serde(rename = "CARRIER")]
    Carrier,
    #[serde(rename = "PATROL")]
    Patrol,
    #[serde(rename = "SATELLITE")]
    Satellite,
    #[serde(rename = "EXPLORER")]
    Explorer,
    #[serde(rename = "REFINERY")]
    Refinery,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipRegistration {
    /// The agent's registered name of the ship
    #[serde(rename = "name")]
    pub name: String,
    /// The symbol of the faction the ship is registered with
    #[serde(rename = "factionSymbol", skip_serializing_if = "Option::is_none")]
    pub faction_symbol: Option<String>,
    #[serde(rename = "role")]
    pub role: ShipRole,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum WaypointType {
    #[default]
    #[serde(rename = "PLANET")]
    Planet,
    #[serde(rename = "GAS_GIANT")]
    GasGiant,
    #[serde(rename = "MOON")]
    Moon,
    #[serde(rename = "ORBITAL_STATION")]
    OrbitalStation,
    #[serde(rename = "JUMP_GATE")]
    JumpGate,
    #[serde(rename = "ASTEROID_FIELD")]
    AsteroidField,
    #[serde(rename = "NEBULA")]
    Nebula,
    #[serde(rename = "DEBRIS_FIELD")]
    DebrisField,
    #[serde(rename = "GRAVITY_WELL")]
    GravityWell,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipNavRouteWaypoint {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub waypoint_type: WaypointType,
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipNavRoute {
    #[serde(rename = "destination")]
    pub destination: ShipNavRouteWaypoint,
    #[serde(rename = "departure")]
    pub departure: ShipNavRouteWaypoint,
    /// The date time of the ship's departure.
    #[serde(rename = "departureTime")]
    pub departure_time: String,
    /// The date time of the ship's arrival. If the ship is in-transit, this is the expected time of arrival.
    #[serde(rename = "arrival")]
    pub arrival: String,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ShipNavStatus {
    #[default]
    #[serde(rename = "IN_TRANSIT")]
    InTransit,
    #[serde(rename = "IN_ORBIT")]
    InOrbit,
    #[serde(rename = "DOCKED")]
    Docked,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ShipNavFlightMode {
    #[default]
    #[serde(rename = "DRIFT")]
    Drift,
    #[serde(rename = "STEALTH")]
    Stealth,
    #[serde(rename = "CRUISE")]
    Cruise,
    #[serde(rename = "BURN")]
    Burn,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipNav {
    /// The system symbol of the ship's current location.
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    /// The waypoint symbol of the ship's current location, or if the ship is in-transit, the waypoint symbol of the ship's destination.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "route")]
    pub route: ShipNavRoute,
    #[serde(rename = "status")]
    pub status: ShipNavStatus,
    #[serde(rename = "flightMode")]
    pub flight_mode: ShipNavFlightMode,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Rotation {
    #[default]
    #[serde(rename = "STRICT")]
    Strict,
    #[serde(rename = "RELAXED")]
    Relaxed,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipCrew {
    /// The current number of crew members on the ship.
    #[serde(rename = "current")]
    pub current: i32,
    /// The minimum number of crew members required to maintain the ship.
    #[serde(rename = "required")]
    pub required: i32,
    /// The maximum number of crew members the ship can support.
    #[serde(rename = "capacity")]
    pub capacity: i32,
    /// The rotation of crew shifts. A stricter shift improves the ship's performance. A more relaxed shift improves the crew's morale.
    #[serde(rename = "rotation")]
    pub rotation: Rotation,
    /// A rough measure of the crew's morale. A higher morale means the crew is happier and more productive. A lower morale means the ship is more prone to accidents.
    #[serde(rename = "morale")]
    pub morale: i32,
    /// The amount of credits per crew member paid per hour. Wages are paid when a ship docks at a civilized waypoint.
    #[serde(rename = "wages")]
    pub wages: i32,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ShipFrameSymbol {
    #[default]
    #[serde(rename = "FRAME_PROBE")]
    Probe,
    #[serde(rename = "FRAME_DRONE")]
    Drone,
    #[serde(rename = "FRAME_INTERCEPTOR")]
    Interceptor,
    #[serde(rename = "FRAME_RACER")]
    Racer,
    #[serde(rename = "FRAME_FIGHTER")]
    Fighter,
    #[serde(rename = "FRAME_FRIGATE")]
    Frigate,
    #[serde(rename = "FRAME_SHUTTLE")]
    Shuttle,
    #[serde(rename = "FRAME_EXPLORER")]
    Explorer,
    #[serde(rename = "FRAME_MINER")]
    Miner,
    #[serde(rename = "FRAME_LIGHT_FREIGHTER")]
    LightFreighter,
    #[serde(rename = "FRAME_HEAVY_FREIGHTER")]
    HeavyFreighter,
    #[serde(rename = "FRAME_TRANSPORT")]
    Transport,
    #[serde(rename = "FRAME_DESTROYER")]
    Destroyer,
    #[serde(rename = "FRAME_CRUISER")]
    Cruiser,
    #[serde(rename = "FRAME_CARRIER")]
    Carrier,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipRequirements {
    /// The amount of power required from the reactor.
    #[serde(rename = "power", skip_serializing_if = "Option::is_none")]
    pub power: Option<i32>,
    /// The number of crew required for operation.
    #[serde(rename = "crew", skip_serializing_if = "Option::is_none")]
    pub crew: Option<i32>,
    /// The number of module slots required for installation.
    #[serde(rename = "slots", skip_serializing_if = "Option::is_none")]
    pub slots: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipFrame {
    #[serde(rename = "symbol")]
    pub symbol: ShipFrameSymbol,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<i32>,
    #[serde(rename = "moduleSlots")]
    pub module_slots: i32,
    #[serde(rename = "mountingPoints")]
    pub mounting_points: i32,
    #[serde(rename = "fuelCapacity")]
    pub fuel_capacity: i32,
    #[serde(rename = "requirements")]
    pub requirements: ShipRequirements,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ShipReactorSymbol {
    #[default]
    #[serde(rename = "REACTOR_SOLAR_I")]
    SolarI,
    #[serde(rename = "REACTOR_FUSION_I")]
    FusionI,
    #[serde(rename = "REACTOR_FISSION_I")]
    FissionI,
    #[serde(rename = "REACTOR_CHEMICAL_I")]
    ChemicalI,
    #[serde(rename = "REACTOR_ANTIMATTER_I")]
    AntimatterI,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipReactor {
    #[serde(rename = "symbol")]
    pub symbol: ShipReactorSymbol,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<i32>,
    #[serde(rename = "powerOutput")]
    pub power_output: i32,
    #[serde(rename = "requirements")]
    pub requirements: ShipRequirements,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ShipEngineSymbol {
    #[default]
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
    ImpulseDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_I")]
    IonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    IonDriveIi,
    #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
    HyperDriveI,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipEngine {
    #[serde(rename = "symbol")]
    pub symbol: ShipEngineSymbol,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<i32>,
    #[serde(rename = "speed")]
    pub speed: f32,
    #[serde(rename = "requirements")]
    pub requirements: ShipRequirements,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ShipModuleSymbol {
    #[default]
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    MineralProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    CargoHoldI,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    CrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    EnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    PassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    MicroRefineryI,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    OreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    FuelRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    JumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    JumpDriveIi,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    JumpDriveIii,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    WarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    WarpDriveIi,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    WarpDriveIii,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ShieldGeneratorIi,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipModule {
    #[serde(rename = "symbol")]
    pub symbol: ShipModuleSymbol,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "requirements")]
    pub requirements: ShipRequirements,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ShipMountSymbol {
    #[default]
    #[serde(rename = "MOUNT_GAS_SIPHON_I")]
    GasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    GasSiphonIi,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    GasSiphonIii,
    #[serde(rename = "MOUNT_SURVEYOR_I")]
    SurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    SurveyorIi,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    SurveyorIii,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
    SensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    SensorArrayIi,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    SensorArrayIii,
    #[serde(rename = "MOUNT_MINING_LASER_I")]
    MiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MiningLaserIi,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MiningLaserIii,
    #[serde(rename = "MOUNT_LASER_CANNON_I")]
    LaserCannonI,
    #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
    MissileLauncherI,
    #[serde(rename = "MOUNT_TURRET_I")]
    TurretI,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShipMountDeposits {
    #[serde(rename = "QUARTZ_SAND")]
    QuartzSand,
    #[serde(rename = "SILICON_CRYSTALS")]
    SiliconCrystals,
    #[serde(rename = "PRECIOUS_STONES")]
    PreciousStones,
    #[serde(rename = "ICE_WATER")]
    IceWater,
    #[serde(rename = "AMMONIA_ICE")]
    AmmoniaIce,
    #[serde(rename = "IRON_ORE")]
    IronOre,
    #[serde(rename = "COPPER_ORE")]
    CopperOre,
    #[serde(rename = "SILVER_ORE")]
    SilverOre,
    #[serde(rename = "ALUMINUM_ORE")]
    AluminumOre,
    #[serde(rename = "GOLD_ORE")]
    GoldOre,
    #[serde(rename = "PLATINUM_ORE")]
    PlatinumOre,
    #[serde(rename = "DIAMONDS")]
    Diamonds,
    #[serde(rename = "URANITE_ORE")]
    UraniteOre,
    #[serde(rename = "MERITIUM_ORE")]
    MeritiumOre,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipMount {
    #[serde(rename = "symbol")]
    pub symbol: ShipMountSymbol,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "strength", skip_serializing_if = "Option::is_none")]
    pub strength: Option<i32>,
    #[serde(rename = "deposits", skip_serializing_if = "Option::is_none")]
    pub deposits: Option<Vec<ShipMountDeposits>>,
    #[serde(rename = "requirements")]
    pub requirements: ShipRequirements,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipCargoItem {
    /// The unique identifier of the cargo item type.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The name of the cargo item type.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the cargo item type.
    #[serde(rename = "description")]
    pub description: String,
    /// The number of units of the cargo item.
    #[serde(rename = "units")]
    pub units: i32,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipCargo {
    /// The max number of items that can be stored in the cargo hold.
    #[serde(rename = "capacity")]
    pub capacity: i32,
    /// The number of items currently stored in the cargo hold.
    #[serde(rename = "units")]
    pub units: i32,
    /// The items currently in the cargo hold.
    #[serde(rename = "inventory")]
    pub inventory: Vec<ShipCargoItem>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipFuelConsumed {
    /// The amount of fuel consumed by the most recent transit or action.
    #[serde(rename = "amount")]
    pub amount: i32,
    /// The time at which the fuel was consumed.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShipFuel {
    /// The current amount of fuel in the ship's tanks.
    #[serde(rename = "current")]
    pub current: i32,
    /// The maximum amount of fuel the ship's tanks can hold.
    #[serde(rename = "capacity")]
    pub capacity: i32,
    #[serde(rename = "consumed", skip_serializing_if = "Option::is_none")]
    pub consumed: Option<ShipFuelConsumed>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Ship {
    /// The globally unique identifier of the ship in the following format: `[AGENT_SYMBOL]_[HEX_ID]`
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "registration")]
    pub registration: ShipRegistration,
    #[serde(rename = "nav")]
    pub nav: ShipNav,
    #[serde(rename = "crew")]
    pub crew: ShipCrew,
    #[serde(rename = "frame")]
    pub frame: ShipFrame,
    #[serde(rename = "reactor")]
    pub reactor: ShipReactor,
    #[serde(rename = "engine")]
    pub engine: ShipEngine,
    #[serde(rename = "modules")]
    pub modules: Vec<ShipModule>,
    #[serde(rename = "mounts")]
    pub mounts: Vec<ShipMount>,
    #[serde(rename = "cargo")]
    pub cargo: ShipCargo,
    #[serde(rename = "fuel")]
    pub fuel: ShipFuel,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum FactionTraitSymbol {
    #[default]
    #[serde(rename = "BUREAUCRATIC")]
    Bureaucratic,
    #[serde(rename = "SECRETIVE")]
    Secretive,
    #[serde(rename = "CAPITALISTIC")]
    Capitalistic,
    #[serde(rename = "INDUSTRIOUS")]
    Industrious,
    #[serde(rename = "PEACEFUL")]
    Peaceful,
    #[serde(rename = "DISTRUSTFUL")]
    Distrustful,
    #[serde(rename = "WELCOMING")]
    Welcoming,
    #[serde(rename = "ANARCHIST")]
    Anarchist,
    #[serde(rename = "CONFLICTED")]
    Conflicted,
    #[serde(rename = "AUTHORITARIAN")]
    Authoritarian,
    #[serde(rename = "OLIGARCHICAL")]
    Oligarchical,
    #[serde(rename = "DYNASTIC")]
    Dynastic,
    #[serde(rename = "DEMOCRACTIC")]
    Democractic,
    #[serde(rename = "DECENTRALIZED")]
    Decentralized,
    #[serde(rename = "SMUGGLERS")]
    Smugglers,
    #[serde(rename = "SCAVENGERS")]
    Scavengers,
    #[serde(rename = "REBELLIOUS")]
    Rebellious,
    #[serde(rename = "EXILES")]
    Exiles,
    #[serde(rename = "PIRATES")]
    Pirates,
    #[serde(rename = "RAIDERS")]
    Raiders,
    #[serde(rename = "CLAN")]
    Clan,
    #[serde(rename = "GUILD")]
    Guild,
    #[serde(rename = "DOMINION")]
    Dominion,
    #[serde(rename = "FRINGE")]
    Fringe,
    #[serde(rename = "FORSAKEN")]
    Forsaken,
    #[serde(rename = "ISOLATED")]
    Isolated,
    #[serde(rename = "LOCALIZED")]
    Localized,
    #[serde(rename = "ESTABLISHED")]
    Established,
    #[serde(rename = "NOTABLE")]
    Notable,
    #[serde(rename = "DOMINANT")]
    Dominant,
    #[serde(rename = "INESCAPABLE")]
    Inescapable,
    #[serde(rename = "INNOVATIVE")]
    Innovative,
    #[serde(rename = "BOLD")]
    Bold,
    #[serde(rename = "VISIONARY")]
    Visionary,
    #[serde(rename = "CURIOUS")]
    Curious,
    #[serde(rename = "DARING")]
    Daring,
    #[serde(rename = "EXPLORATORY")]
    Exploratory,
    #[serde(rename = "RESOURCEFUL")]
    Resourceful,
    #[serde(rename = "FLEXIBLE")]
    Flexible,
    #[serde(rename = "COOPERATIVE")]
    Cooperative,
    #[serde(rename = "UNITED")]
    United,
    #[serde(rename = "STRATEGIC")]
    Strategic,
    #[serde(rename = "INTELLIGENT")]
    Intelligent,
    #[serde(rename = "RESEARCH_FOCUSED")]
    ResearchFocused,
    #[serde(rename = "COLLABORATIVE")]
    Collaborative,
    #[serde(rename = "PROGRESSIVE")]
    Progressive,
    #[serde(rename = "MILITARISTIC")]
    Militaristic,
    #[serde(rename = "TECHNOLOGICALLY_ADVANCED")]
    TechnologicallyAdvanced,
    #[serde(rename = "AGGRESSIVE")]
    Aggressive,
    #[serde(rename = "IMPERIALISTIC")]
    Imperialistic,
    #[serde(rename = "TREASURE_HUNTERS")]
    TreasureHunters,
    #[serde(rename = "DEXTEROUS")]
    Dexterous,
    #[serde(rename = "UNPREDICTABLE")]
    Unpredictable,
    #[serde(rename = "BRUTAL")]
    Brutal,
    #[serde(rename = "FLEETING")]
    Fleeting,
    #[serde(rename = "ADAPTABLE")]
    Adaptable,
    #[serde(rename = "SELF_SUFFICIENT")]
    SelfSufficient,
    #[serde(rename = "DEFENSIVE")]
    Defensive,
    #[serde(rename = "PROUD")]
    Proud,
    #[serde(rename = "DIVERSE")]
    Diverse,
    #[serde(rename = "INDEPENDENT")]
    Independent,
    #[serde(rename = "SELF_INTERESTED")]
    SelfInterested,
    #[serde(rename = "FRAGMENTED")]
    Fragmented,
    #[serde(rename = "COMMERCIAL")]
    Commercial,
    #[serde(rename = "FREE_MARKETS")]
    FreeMarkets,
    #[serde(rename = "ENTREPRENEURIAL")]
    Entrepreneurial,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FactionTrait {
    /// The unique identifier of the trait.
    #[serde(rename = "symbol")]
    pub symbol: FactionTraitSymbol,
    /// The name of the trait.
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the trait.
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Faction {
    #[serde(rename = "symbol")]
    pub symbol: FactionSymbol,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "headquarters")]
    pub headquarters: String,
    #[serde(rename = "traits")]
    pub traits: Vec<FactionTrait>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterResponseData {
    #[serde(rename = "agent")]
    pub agent: Agent,
    #[serde(rename = "contract")]
    pub contract: Contract,
    #[serde(rename = "faction")]
    pub faction: Faction,
    #[serde(rename = "ship")]
    pub ship: Ship,
    /// A Bearer token for accessing secured API endpoints.
    #[serde(rename = "token")]
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterResponse {
    #[serde(rename = "data")]
    pub data: RegisterResponseData,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "limit")]
    pub limit: i32,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMyShipsResponse {
    #[serde(rename = "data")]
    pub data: Vec<Ship>,
    #[serde(rename = "meta")]
    pub meta: Meta,
}
