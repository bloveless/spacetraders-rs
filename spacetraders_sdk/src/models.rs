#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AcceptContract200Response {
    #[serde(rename = "data")]
    pub data: models::AcceptContract200ResponseData,

}

impl AcceptContract200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::AcceptContract200ResponseData, ) -> AcceptContract200Response {
        AcceptContract200Response {
            data,
        }
    }
}

/// Converts the AcceptContract200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AcceptContract200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AcceptContract200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AcceptContract200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::AcceptContract200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AcceptContract200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::AcceptContract200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AcceptContract200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AcceptContract200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in AcceptContract200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AcceptContract200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AcceptContract200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AcceptContract200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AcceptContract200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AcceptContract200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AcceptContract200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AcceptContract200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AcceptContract200ResponseData {
    #[serde(rename = "agent")]
    pub agent: models::Agent,

    #[serde(rename = "contract")]
    pub contract: models::Contract,

}

impl AcceptContract200ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(agent: models::Agent, contract: models::Contract, ) -> AcceptContract200ResponseData {
        AcceptContract200ResponseData {
            agent,
            contract,
        }
    }
}

/// Converts the AcceptContract200ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AcceptContract200ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping agent in query parameter serialization

            // Skipping contract in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AcceptContract200ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AcceptContract200ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub agent: Vec<models::Agent>,
            pub contract: Vec<models::Contract>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AcceptContract200ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "agent" => intermediate_rep.agent.push(<models::Agent as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contract" => intermediate_rep.contract.push(<models::Contract as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AcceptContract200ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AcceptContract200ResponseData {
            agent: intermediate_rep.agent.into_iter().next().ok_or_else(|| "agent missing in AcceptContract200ResponseData".to_string())?,
            contract: intermediate_rep.contract.into_iter().next().ok_or_else(|| "contract missing in AcceptContract200ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AcceptContract200ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AcceptContract200ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AcceptContract200ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AcceptContract200ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AcceptContract200ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AcceptContract200ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AcceptContract200ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

impl Agent {
    #[allow(clippy::new_without_default)]
    pub fn new(account_id: String, symbol: String, headquarters: String, credits: i32, ) -> Agent {
        Agent {
            account_id,
            symbol,
            headquarters,
            credits,
        }
    }
}

/// Converts the Agent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Agent {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("accountId".to_string()),
            Some(self.account_id.to_string()),


            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("headquarters".to_string()),
            Some(self.headquarters.to_string()),


            Some("credits".to_string()),
            Some(self.credits.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Agent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Agent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub account_id: Vec<String>,
            pub symbol: Vec<String>,
            pub headquarters: Vec<String>,
            pub credits: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Agent".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "accountId" => intermediate_rep.account_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "headquarters" => intermediate_rep.headquarters.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "credits" => intermediate_rep.credits.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Agent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Agent {
            account_id: intermediate_rep.account_id.into_iter().next().ok_or_else(|| "accountId missing in Agent".to_string())?,
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in Agent".to_string())?,
            headquarters: intermediate_rep.headquarters.into_iter().next().ok_or_else(|| "headquarters missing in Agent".to_string())?,
            credits: intermediate_rep.credits.into_iter().next().ok_or_else(|| "credits missing in Agent".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Agent> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Agent>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Agent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Agent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Agent> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Agent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Agent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The chart of a system or waypoint, which makes the location visible to other agents.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Chart {
    #[serde(rename = "waypointSymbol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub waypoint_symbol: Option<String>,

    #[serde(rename = "submittedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub submitted_by: Option<String>,

    #[serde(rename = "submittedOn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub submitted_on: Option<chrono::DateTime::<chrono::Utc>>,

}

impl Chart {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Chart {
        Chart {
            waypoint_symbol: None,
            submitted_by: None,
            submitted_on: None,
        }
    }
}

/// Converts the Chart value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Chart {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.waypoint_symbol.as_ref().map(|waypoint_symbol| {
                vec![
                    "waypointSymbol".to_string(),
                    waypoint_symbol.to_string(),
                ].join(",")
            }),


            self.submitted_by.as_ref().map(|submitted_by| {
                vec![
                    "submittedBy".to_string(),
                    submitted_by.to_string(),
                ].join(",")
            }),

            // Skipping submittedOn in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Chart value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Chart {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub waypoint_symbol: Vec<String>,
            pub submitted_by: Vec<String>,
            pub submitted_on: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Chart".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "waypointSymbol" => intermediate_rep.waypoint_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "submittedBy" => intermediate_rep.submitted_by.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "submittedOn" => intermediate_rep.submitted_on.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Chart".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Chart {
            waypoint_symbol: intermediate_rep.waypoint_symbol.into_iter().next(),
            submitted_by: intermediate_rep.submitted_by.into_iter().next(),
            submitted_on: intermediate_rep.submitted_on.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Chart> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Chart>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Chart>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Chart - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Chart> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Chart as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Chart - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConnectedSystem {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "sectorSymbol")]
    pub sector_symbol: String,

    #[serde(rename = "type")]
    pub r#type: models::SystemType,

    /// The symbol of the faction that owns the connected jump gate in the system.
    #[serde(rename = "factionSymbol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub faction_symbol: Option<String>,

    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

    #[serde(rename = "distance")]
    pub distance: i32,

}

impl ConnectedSystem {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, sector_symbol: String, r#type: models::SystemType, x: i32, y: i32, distance: i32, ) -> ConnectedSystem {
        ConnectedSystem {
            symbol,
            sector_symbol,
            r#type,
            faction_symbol: None,
            x,
            y,
            distance,
        }
    }
}

/// Converts the ConnectedSystem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConnectedSystem {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("sectorSymbol".to_string()),
            Some(self.sector_symbol.to_string()),

            // Skipping type in query parameter serialization


            self.faction_symbol.as_ref().map(|faction_symbol| {
                vec![
                    "factionSymbol".to_string(),
                    faction_symbol.to_string(),
                ].join(",")
            }),


            Some("x".to_string()),
            Some(self.x.to_string()),


            Some("y".to_string()),
            Some(self.y.to_string()),


            Some("distance".to_string()),
            Some(self.distance.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConnectedSystem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConnectedSystem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub sector_symbol: Vec<String>,
            pub r#type: Vec<models::SystemType>,
            pub faction_symbol: Vec<String>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
            pub distance: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConnectedSystem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sectorSymbol" => intermediate_rep.sector_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::SystemType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "factionSymbol" => intermediate_rep.faction_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "distance" => intermediate_rep.distance.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConnectedSystem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConnectedSystem {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ConnectedSystem".to_string())?,
            sector_symbol: intermediate_rep.sector_symbol.into_iter().next().ok_or_else(|| "sectorSymbol missing in ConnectedSystem".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ConnectedSystem".to_string())?,
            faction_symbol: intermediate_rep.faction_symbol.into_iter().next(),
            x: intermediate_rep.x.into_iter().next().ok_or_else(|| "x missing in ConnectedSystem".to_string())?,
            y: intermediate_rep.y.into_iter().next().ok_or_else(|| "y missing in ConnectedSystem".to_string())?,
            distance: intermediate_rep.distance.into_iter().next().ok_or_else(|| "distance missing in ConnectedSystem".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConnectedSystem> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConnectedSystem>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConnectedSystem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConnectedSystem - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConnectedSystem> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConnectedSystem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConnectedSystem - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Contract {
    #[serde(rename = "id")]
    pub id: String,

    /// The symbol of the faction that this contract is for.
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "terms")]
    pub terms: models::ContractTerms,

    /// Whether the contract has been accepted by the agent
    #[serde(rename = "accepted")]
    pub accepted: bool,

    /// Whether the contract has been fulfilled
    #[serde(rename = "fulfilled")]
    pub fulfilled: bool,

    /// The time at which the contract expires
    #[serde(rename = "expiration")]
    pub expiration: chrono::DateTime::<chrono::Utc>,

}

impl Contract {
    #[allow(clippy::new_without_default)]
    pub fn new(id: String, faction_symbol: String, r#type: String, terms: models::ContractTerms, expiration: chrono::DateTime::<chrono::Utc>, ) -> Contract {
        Contract {
            id,
            faction_symbol,
            r#type,
            terms,
            accepted: false,
            fulfilled: false,
            expiration,
        }
    }
}

/// Converts the Contract value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Contract {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("factionSymbol".to_string()),
            Some(self.faction_symbol.to_string()),


            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping terms in query parameter serialization


            Some("accepted".to_string()),
            Some(self.accepted.to_string()),


            Some("fulfilled".to_string()),
            Some(self.fulfilled.to_string()),

            // Skipping expiration in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Contract value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Contract {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub faction_symbol: Vec<String>,
            pub r#type: Vec<String>,
            pub terms: Vec<models::ContractTerms>,
            pub accepted: Vec<bool>,
            pub fulfilled: Vec<bool>,
            pub expiration: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Contract".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "factionSymbol" => intermediate_rep.faction_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "terms" => intermediate_rep.terms.push(<models::ContractTerms as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "accepted" => intermediate_rep.accepted.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "fulfilled" => intermediate_rep.fulfilled.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expiration" => intermediate_rep.expiration.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Contract".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Contract {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in Contract".to_string())?,
            faction_symbol: intermediate_rep.faction_symbol.into_iter().next().ok_or_else(|| "factionSymbol missing in Contract".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in Contract".to_string())?,
            terms: intermediate_rep.terms.into_iter().next().ok_or_else(|| "terms missing in Contract".to_string())?,
            accepted: intermediate_rep.accepted.into_iter().next().ok_or_else(|| "accepted missing in Contract".to_string())?,
            fulfilled: intermediate_rep.fulfilled.into_iter().next().ok_or_else(|| "fulfilled missing in Contract".to_string())?,
            expiration: intermediate_rep.expiration.into_iter().next().ok_or_else(|| "expiration missing in Contract".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Contract> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Contract>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Contract>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Contract - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Contract> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Contract as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Contract - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The details of a delivery contract. Includes the type of good, units needed, and the destination.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

impl ContractDeliverGood {
    #[allow(clippy::new_without_default)]
    pub fn new(trade_symbol: String, destination_symbol: String, units_required: i32, units_fulfilled: i32, ) -> ContractDeliverGood {
        ContractDeliverGood {
            trade_symbol,
            destination_symbol,
            units_required,
            units_fulfilled,
        }
    }
}

/// Converts the ContractDeliverGood value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ContractDeliverGood {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("tradeSymbol".to_string()),
            Some(self.trade_symbol.to_string()),


            Some("destinationSymbol".to_string()),
            Some(self.destination_symbol.to_string()),


            Some("unitsRequired".to_string()),
            Some(self.units_required.to_string()),


            Some("unitsFulfilled".to_string()),
            Some(self.units_fulfilled.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContractDeliverGood value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContractDeliverGood {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trade_symbol: Vec<String>,
            pub destination_symbol: Vec<String>,
            pub units_required: Vec<i32>,
            pub units_fulfilled: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContractDeliverGood".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tradeSymbol" => intermediate_rep.trade_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "destinationSymbol" => intermediate_rep.destination_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "unitsRequired" => intermediate_rep.units_required.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "unitsFulfilled" => intermediate_rep.units_fulfilled.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContractDeliverGood".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContractDeliverGood {
            trade_symbol: intermediate_rep.trade_symbol.into_iter().next().ok_or_else(|| "tradeSymbol missing in ContractDeliverGood".to_string())?,
            destination_symbol: intermediate_rep.destination_symbol.into_iter().next().ok_or_else(|| "destinationSymbol missing in ContractDeliverGood".to_string())?,
            units_required: intermediate_rep.units_required.into_iter().next().ok_or_else(|| "unitsRequired missing in ContractDeliverGood".to_string())?,
            units_fulfilled: intermediate_rep.units_fulfilled.into_iter().next().ok_or_else(|| "unitsFulfilled missing in ContractDeliverGood".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContractDeliverGood> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ContractDeliverGood>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContractDeliverGood>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ContractDeliverGood - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ContractDeliverGood> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContractDeliverGood as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ContractDeliverGood - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContractPayment {
    /// The amount of credits received up front for accepting the contract.
    #[serde(rename = "onAccepted")]
    pub on_accepted: i32,

    /// The amount of credits received when the contract is fulfilled.
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: i32,

}

impl ContractPayment {
    #[allow(clippy::new_without_default)]
    pub fn new(on_accepted: i32, on_fulfilled: i32, ) -> ContractPayment {
        ContractPayment {
            on_accepted,
            on_fulfilled,
        }
    }
}

/// Converts the ContractPayment value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ContractPayment {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("onAccepted".to_string()),
            Some(self.on_accepted.to_string()),


            Some("onFulfilled".to_string()),
            Some(self.on_fulfilled.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContractPayment value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContractPayment {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub on_accepted: Vec<i32>,
            pub on_fulfilled: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContractPayment".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "onAccepted" => intermediate_rep.on_accepted.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "onFulfilled" => intermediate_rep.on_fulfilled.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContractPayment".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContractPayment {
            on_accepted: intermediate_rep.on_accepted.into_iter().next().ok_or_else(|| "onAccepted missing in ContractPayment".to_string())?,
            on_fulfilled: intermediate_rep.on_fulfilled.into_iter().next().ok_or_else(|| "onFulfilled missing in ContractPayment".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContractPayment> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ContractPayment>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContractPayment>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ContractPayment - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ContractPayment> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContractPayment as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ContractPayment - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContractTerms {
    /// The deadline for the contract.
    #[serde(rename = "deadline")]
    pub deadline: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "payment")]
    pub payment: models::ContractPayment,

    #[serde(rename = "deliver")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deliver: Option<Vec<models::ContractDeliverGood>>,

}

impl ContractTerms {
    #[allow(clippy::new_without_default)]
    pub fn new(deadline: chrono::DateTime::<chrono::Utc>, payment: models::ContractPayment, ) -> ContractTerms {
        ContractTerms {
            deadline,
            payment,
            deliver: None,
        }
    }
}

/// Converts the ContractTerms value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ContractTerms {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping deadline in query parameter serialization

            // Skipping payment in query parameter serialization

            // Skipping deliver in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContractTerms value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContractTerms {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub deadline: Vec<chrono::DateTime::<chrono::Utc>>,
            pub payment: Vec<models::ContractPayment>,
            pub deliver: Vec<Vec<models::ContractDeliverGood>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContractTerms".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "deadline" => intermediate_rep.deadline.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "payment" => intermediate_rep.payment.push(<models::ContractPayment as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "deliver" => return std::result::Result::Err("Parsing a container in this style is not supported in ContractTerms".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContractTerms".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContractTerms {
            deadline: intermediate_rep.deadline.into_iter().next().ok_or_else(|| "deadline missing in ContractTerms".to_string())?,
            payment: intermediate_rep.payment.into_iter().next().ok_or_else(|| "payment missing in ContractTerms".to_string())?,
            deliver: intermediate_rep.deliver.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContractTerms> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ContractTerms>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContractTerms>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ContractTerms - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ContractTerms> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContractTerms as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ContractTerms - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A cooldown is a period of time in which a ship cannot perform certain actions.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Cooldown {
    /// The symbol of the ship that is on cooldown
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,

    /// The total duration of the cooldown in seconds
    #[serde(rename = "totalSeconds")]
    pub total_seconds: u32,

    /// The remaining duration of the cooldown in seconds
    #[serde(rename = "remainingSeconds")]
    pub remaining_seconds: u32,

    /// The date and time when the cooldown expires in ISO 8601 format
    #[serde(rename = "expiration")]
    pub expiration: chrono::DateTime::<chrono::Utc>,

}

impl Cooldown {
    #[allow(clippy::new_without_default)]
    pub fn new(ship_symbol: String, total_seconds: u32, remaining_seconds: u32, expiration: chrono::DateTime::<chrono::Utc>, ) -> Cooldown {
        Cooldown {
            ship_symbol,
            total_seconds,
            remaining_seconds,
            expiration,
        }
    }
}

/// Converts the Cooldown value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Cooldown {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("shipSymbol".to_string()),
            Some(self.ship_symbol.to_string()),


            Some("totalSeconds".to_string()),
            Some(self.total_seconds.to_string()),


            Some("remainingSeconds".to_string()),
            Some(self.remaining_seconds.to_string()),

            // Skipping expiration in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Cooldown value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Cooldown {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub ship_symbol: Vec<String>,
            pub total_seconds: Vec<u32>,
            pub remaining_seconds: Vec<u32>,
            pub expiration: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Cooldown".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "shipSymbol" => intermediate_rep.ship_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "totalSeconds" => intermediate_rep.total_seconds.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "remainingSeconds" => intermediate_rep.remaining_seconds.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expiration" => intermediate_rep.expiration.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Cooldown".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Cooldown {
            ship_symbol: intermediate_rep.ship_symbol.into_iter().next().ok_or_else(|| "shipSymbol missing in Cooldown".to_string())?,
            total_seconds: intermediate_rep.total_seconds.into_iter().next().ok_or_else(|| "totalSeconds missing in Cooldown".to_string())?,
            remaining_seconds: intermediate_rep.remaining_seconds.into_iter().next().ok_or_else(|| "remainingSeconds missing in Cooldown".to_string())?,
            expiration: intermediate_rep.expiration.into_iter().next().ok_or_else(|| "expiration missing in Cooldown".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Cooldown> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Cooldown>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Cooldown>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Cooldown - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Cooldown> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Cooldown as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Cooldown - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateChart201Response {
    #[serde(rename = "data")]
    pub data: models::CreateChart201ResponseData,

}

impl CreateChart201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::CreateChart201ResponseData, ) -> CreateChart201Response {
        CreateChart201Response {
            data,
        }
    }
}

/// Converts the CreateChart201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateChart201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateChart201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateChart201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::CreateChart201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateChart201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::CreateChart201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateChart201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateChart201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in CreateChart201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateChart201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateChart201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateChart201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateChart201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateChart201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateChart201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateChart201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateChart201ResponseData {
    #[serde(rename = "chart")]
    pub chart: models::Chart,

    #[serde(rename = "waypoint")]
    pub waypoint: models::Waypoint,

}

impl CreateChart201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(chart: models::Chart, waypoint: models::Waypoint, ) -> CreateChart201ResponseData {
        CreateChart201ResponseData {
            chart,
            waypoint,
        }
    }
}

/// Converts the CreateChart201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateChart201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping chart in query parameter serialization

            // Skipping waypoint in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateChart201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateChart201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub chart: Vec<models::Chart>,
            pub waypoint: Vec<models::Waypoint>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateChart201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "chart" => intermediate_rep.chart.push(<models::Chart as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "waypoint" => intermediate_rep.waypoint.push(<models::Waypoint as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateChart201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateChart201ResponseData {
            chart: intermediate_rep.chart.into_iter().next().ok_or_else(|| "chart missing in CreateChart201ResponseData".to_string())?,
            waypoint: intermediate_rep.waypoint.into_iter().next().ok_or_else(|| "waypoint missing in CreateChart201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateChart201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateChart201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateChart201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateChart201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateChart201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateChart201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateChart201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateShipShipScan201Response {
    #[serde(rename = "data")]
    pub data: models::CreateShipShipScan201ResponseData,

}

impl CreateShipShipScan201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::CreateShipShipScan201ResponseData, ) -> CreateShipShipScan201Response {
        CreateShipShipScan201Response {
            data,
        }
    }
}

/// Converts the CreateShipShipScan201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateShipShipScan201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateShipShipScan201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateShipShipScan201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::CreateShipShipScan201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateShipShipScan201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::CreateShipShipScan201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateShipShipScan201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateShipShipScan201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in CreateShipShipScan201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateShipShipScan201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateShipShipScan201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateShipShipScan201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateShipShipScan201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateShipShipScan201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateShipShipScan201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateShipShipScan201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateShipShipScan201ResponseData {
    #[serde(rename = "cooldown")]
    pub cooldown: models::Cooldown,

    #[serde(rename = "ships")]
    pub ships: Vec<models::ScannedShip>,

}

impl CreateShipShipScan201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(cooldown: models::Cooldown, ships: Vec<models::ScannedShip>, ) -> CreateShipShipScan201ResponseData {
        CreateShipShipScan201ResponseData {
            cooldown,
            ships,
        }
    }
}

/// Converts the CreateShipShipScan201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateShipShipScan201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping ships in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateShipShipScan201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateShipShipScan201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::Cooldown>,
            pub ships: Vec<Vec<models::ScannedShip>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateShipShipScan201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(<models::Cooldown as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "ships" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateShipShipScan201ResponseData".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateShipShipScan201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateShipShipScan201ResponseData {
            cooldown: intermediate_rep.cooldown.into_iter().next().ok_or_else(|| "cooldown missing in CreateShipShipScan201ResponseData".to_string())?,
            ships: intermediate_rep.ships.into_iter().next().ok_or_else(|| "ships missing in CreateShipShipScan201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateShipShipScan201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateShipShipScan201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateShipShipScan201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateShipShipScan201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateShipShipScan201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateShipShipScan201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateShipShipScan201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateShipSystemScan201Response {
    #[serde(rename = "data")]
    pub data: models::CreateShipSystemScan201ResponseData,

}

impl CreateShipSystemScan201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::CreateShipSystemScan201ResponseData, ) -> CreateShipSystemScan201Response {
        CreateShipSystemScan201Response {
            data,
        }
    }
}

/// Converts the CreateShipSystemScan201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateShipSystemScan201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateShipSystemScan201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateShipSystemScan201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::CreateShipSystemScan201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateShipSystemScan201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::CreateShipSystemScan201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateShipSystemScan201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateShipSystemScan201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in CreateShipSystemScan201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateShipSystemScan201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateShipSystemScan201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateShipSystemScan201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateShipSystemScan201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateShipSystemScan201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateShipSystemScan201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateShipSystemScan201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateShipSystemScan201ResponseData {
    #[serde(rename = "cooldown")]
    pub cooldown: models::Cooldown,

    #[serde(rename = "systems")]
    pub systems: Vec<models::ScannedSystem>,

}

impl CreateShipSystemScan201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(cooldown: models::Cooldown, systems: Vec<models::ScannedSystem>, ) -> CreateShipSystemScan201ResponseData {
        CreateShipSystemScan201ResponseData {
            cooldown,
            systems,
        }
    }
}

/// Converts the CreateShipSystemScan201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateShipSystemScan201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping systems in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateShipSystemScan201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateShipSystemScan201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::Cooldown>,
            pub systems: Vec<Vec<models::ScannedSystem>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateShipSystemScan201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(<models::Cooldown as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "systems" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateShipSystemScan201ResponseData".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateShipSystemScan201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateShipSystemScan201ResponseData {
            cooldown: intermediate_rep.cooldown.into_iter().next().ok_or_else(|| "cooldown missing in CreateShipSystemScan201ResponseData".to_string())?,
            systems: intermediate_rep.systems.into_iter().next().ok_or_else(|| "systems missing in CreateShipSystemScan201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateShipSystemScan201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateShipSystemScan201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateShipSystemScan201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateShipSystemScan201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateShipSystemScan201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateShipSystemScan201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateShipSystemScan201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateShipWaypointScan201Response {
    #[serde(rename = "data")]
    pub data: models::CreateShipWaypointScan201ResponseData,

}

impl CreateShipWaypointScan201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::CreateShipWaypointScan201ResponseData, ) -> CreateShipWaypointScan201Response {
        CreateShipWaypointScan201Response {
            data,
        }
    }
}

/// Converts the CreateShipWaypointScan201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateShipWaypointScan201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateShipWaypointScan201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateShipWaypointScan201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::CreateShipWaypointScan201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateShipWaypointScan201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::CreateShipWaypointScan201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateShipWaypointScan201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateShipWaypointScan201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in CreateShipWaypointScan201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateShipWaypointScan201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateShipWaypointScan201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateShipWaypointScan201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateShipWaypointScan201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateShipWaypointScan201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateShipWaypointScan201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateShipWaypointScan201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateShipWaypointScan201ResponseData {
    #[serde(rename = "cooldown")]
    pub cooldown: models::Cooldown,

    #[serde(rename = "waypoints")]
    pub waypoints: Vec<models::ScannedWaypoint>,

}

impl CreateShipWaypointScan201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(cooldown: models::Cooldown, waypoints: Vec<models::ScannedWaypoint>, ) -> CreateShipWaypointScan201ResponseData {
        CreateShipWaypointScan201ResponseData {
            cooldown,
            waypoints,
        }
    }
}

/// Converts the CreateShipWaypointScan201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateShipWaypointScan201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping waypoints in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateShipWaypointScan201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateShipWaypointScan201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::Cooldown>,
            pub waypoints: Vec<Vec<models::ScannedWaypoint>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateShipWaypointScan201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(<models::Cooldown as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "waypoints" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateShipWaypointScan201ResponseData".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateShipWaypointScan201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateShipWaypointScan201ResponseData {
            cooldown: intermediate_rep.cooldown.into_iter().next().ok_or_else(|| "cooldown missing in CreateShipWaypointScan201ResponseData".to_string())?,
            waypoints: intermediate_rep.waypoints.into_iter().next().ok_or_else(|| "waypoints missing in CreateShipWaypointScan201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateShipWaypointScan201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateShipWaypointScan201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateShipWaypointScan201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateShipWaypointScan201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateShipWaypointScan201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateShipWaypointScan201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateShipWaypointScan201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateSurvey201Response {
    #[serde(rename = "data")]
    pub data: models::CreateSurvey201ResponseData,

}

impl CreateSurvey201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::CreateSurvey201ResponseData, ) -> CreateSurvey201Response {
        CreateSurvey201Response {
            data,
        }
    }
}

/// Converts the CreateSurvey201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateSurvey201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateSurvey201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateSurvey201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::CreateSurvey201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateSurvey201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::CreateSurvey201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateSurvey201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateSurvey201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in CreateSurvey201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateSurvey201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateSurvey201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateSurvey201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateSurvey201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateSurvey201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateSurvey201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateSurvey201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateSurvey201ResponseData {
    #[serde(rename = "cooldown")]
    pub cooldown: models::Cooldown,

    #[serde(rename = "surveys")]
    pub surveys: Vec<models::Survey>,

}

impl CreateSurvey201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(cooldown: models::Cooldown, surveys: Vec<models::Survey>, ) -> CreateSurvey201ResponseData {
        CreateSurvey201ResponseData {
            cooldown,
            surveys,
        }
    }
}

/// Converts the CreateSurvey201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateSurvey201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping surveys in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateSurvey201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateSurvey201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::Cooldown>,
            pub surveys: Vec<Vec<models::Survey>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateSurvey201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(<models::Cooldown as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "surveys" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateSurvey201ResponseData".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateSurvey201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateSurvey201ResponseData {
            cooldown: intermediate_rep.cooldown.into_iter().next().ok_or_else(|| "cooldown missing in CreateSurvey201ResponseData".to_string())?,
            surveys: intermediate_rep.surveys.into_iter().next().ok_or_else(|| "surveys missing in CreateSurvey201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateSurvey201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateSurvey201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateSurvey201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateSurvey201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateSurvey201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateSurvey201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateSurvey201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeliverContract200Response {
    #[serde(rename = "data")]
    pub data: models::DeliverContract200ResponseData,

}

impl DeliverContract200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::DeliverContract200ResponseData, ) -> DeliverContract200Response {
        DeliverContract200Response {
            data,
        }
    }
}

/// Converts the DeliverContract200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeliverContract200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeliverContract200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeliverContract200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::DeliverContract200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DeliverContract200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::DeliverContract200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DeliverContract200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeliverContract200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in DeliverContract200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeliverContract200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeliverContract200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DeliverContract200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DeliverContract200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DeliverContract200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DeliverContract200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DeliverContract200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeliverContract200ResponseData {
    #[serde(rename = "contract")]
    pub contract: models::Contract,

    #[serde(rename = "cargo")]
    pub cargo: models::ShipCargo,

}

impl DeliverContract200ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(contract: models::Contract, cargo: models::ShipCargo, ) -> DeliverContract200ResponseData {
        DeliverContract200ResponseData {
            contract,
            cargo,
        }
    }
}

/// Converts the DeliverContract200ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeliverContract200ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping contract in query parameter serialization

            // Skipping cargo in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeliverContract200ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeliverContract200ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub contract: Vec<models::Contract>,
            pub cargo: Vec<models::ShipCargo>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DeliverContract200ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "contract" => intermediate_rep.contract.push(<models::Contract as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cargo" => intermediate_rep.cargo.push(<models::ShipCargo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DeliverContract200ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeliverContract200ResponseData {
            contract: intermediate_rep.contract.into_iter().next().ok_or_else(|| "contract missing in DeliverContract200ResponseData".to_string())?,
            cargo: intermediate_rep.cargo.into_iter().next().ok_or_else(|| "cargo missing in DeliverContract200ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeliverContract200ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeliverContract200ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DeliverContract200ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DeliverContract200ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DeliverContract200ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DeliverContract200ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DeliverContract200ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeliverContractRequest {
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,

    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,

    #[serde(rename = "units")]
    pub units: i32,

}

impl DeliverContractRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(ship_symbol: String, trade_symbol: String, units: i32, ) -> DeliverContractRequest {
        DeliverContractRequest {
            ship_symbol,
            trade_symbol,
            units,
        }
    }
}

/// Converts the DeliverContractRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeliverContractRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("shipSymbol".to_string()),
            Some(self.ship_symbol.to_string()),


            Some("tradeSymbol".to_string()),
            Some(self.trade_symbol.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeliverContractRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeliverContractRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub ship_symbol: Vec<String>,
            pub trade_symbol: Vec<String>,
            pub units: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DeliverContractRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "shipSymbol" => intermediate_rep.ship_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tradeSymbol" => intermediate_rep.trade_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DeliverContractRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeliverContractRequest {
            ship_symbol: intermediate_rep.ship_symbol.into_iter().next().ok_or_else(|| "shipSymbol missing in DeliverContractRequest".to_string())?,
            trade_symbol: intermediate_rep.trade_symbol.into_iter().next().ok_or_else(|| "tradeSymbol missing in DeliverContractRequest".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in DeliverContractRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeliverContractRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeliverContractRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DeliverContractRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DeliverContractRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DeliverContractRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DeliverContractRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DeliverContractRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DockShip200Response {
    #[serde(rename = "data")]
    pub data: models::OrbitShip200ResponseData,

}

impl DockShip200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::OrbitShip200ResponseData, ) -> DockShip200Response {
        DockShip200Response {
            data,
        }
    }
}

/// Converts the DockShip200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DockShip200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DockShip200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DockShip200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::OrbitShip200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DockShip200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::OrbitShip200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DockShip200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DockShip200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in DockShip200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DockShip200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DockShip200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DockShip200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DockShip200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DockShip200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DockShip200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DockShip200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExtractResources201Response {
    #[serde(rename = "data")]
    pub data: models::ExtractResources201ResponseData,

}

impl ExtractResources201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::ExtractResources201ResponseData, ) -> ExtractResources201Response {
        ExtractResources201Response {
            data,
        }
    }
}

/// Converts the ExtractResources201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ExtractResources201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExtractResources201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExtractResources201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::ExtractResources201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExtractResources201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::ExtractResources201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExtractResources201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExtractResources201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in ExtractResources201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExtractResources201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ExtractResources201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExtractResources201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExtractResources201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ExtractResources201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExtractResources201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExtractResources201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExtractResources201ResponseData {
    #[serde(rename = "cooldown")]
    pub cooldown: models::Cooldown,

    #[serde(rename = "extraction")]
    pub extraction: models::Extraction,

    #[serde(rename = "cargo")]
    pub cargo: models::ShipCargo,

}

impl ExtractResources201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(cooldown: models::Cooldown, extraction: models::Extraction, cargo: models::ShipCargo, ) -> ExtractResources201ResponseData {
        ExtractResources201ResponseData {
            cooldown,
            extraction,
            cargo,
        }
    }
}

/// Converts the ExtractResources201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ExtractResources201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping extraction in query parameter serialization

            // Skipping cargo in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExtractResources201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExtractResources201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::Cooldown>,
            pub extraction: Vec<models::Extraction>,
            pub cargo: Vec<models::ShipCargo>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExtractResources201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(<models::Cooldown as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "extraction" => intermediate_rep.extraction.push(<models::Extraction as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cargo" => intermediate_rep.cargo.push(<models::ShipCargo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExtractResources201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExtractResources201ResponseData {
            cooldown: intermediate_rep.cooldown.into_iter().next().ok_or_else(|| "cooldown missing in ExtractResources201ResponseData".to_string())?,
            extraction: intermediate_rep.extraction.into_iter().next().ok_or_else(|| "extraction missing in ExtractResources201ResponseData".to_string())?,
            cargo: intermediate_rep.cargo.into_iter().next().ok_or_else(|| "cargo missing in ExtractResources201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExtractResources201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ExtractResources201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExtractResources201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExtractResources201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ExtractResources201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExtractResources201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExtractResources201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExtractResourcesRequest {
    #[serde(rename = "survey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub survey: Option<models::Survey>,

}

impl ExtractResourcesRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ExtractResourcesRequest {
        ExtractResourcesRequest {
            survey: None,
        }
    }
}

/// Converts the ExtractResourcesRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ExtractResourcesRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping survey in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExtractResourcesRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExtractResourcesRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub survey: Vec<models::Survey>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExtractResourcesRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "survey" => intermediate_rep.survey.push(<models::Survey as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExtractResourcesRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExtractResourcesRequest {
            survey: intermediate_rep.survey.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExtractResourcesRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ExtractResourcesRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExtractResourcesRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExtractResourcesRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ExtractResourcesRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExtractResourcesRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExtractResourcesRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Extraction {
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,

    #[serde(rename = "yield")]
    pub r#yield: models::ExtractionYield,

}

impl Extraction {
    #[allow(clippy::new_without_default)]
    pub fn new(ship_symbol: String, r#yield: models::ExtractionYield, ) -> Extraction {
        Extraction {
            ship_symbol,
            r#yield,
        }
    }
}

/// Converts the Extraction value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Extraction {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("shipSymbol".to_string()),
            Some(self.ship_symbol.to_string()),

            // Skipping yield in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Extraction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Extraction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub ship_symbol: Vec<String>,
            pub r#yield: Vec<models::ExtractionYield>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Extraction".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "shipSymbol" => intermediate_rep.ship_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "yield" => intermediate_rep.r#yield.push(<models::ExtractionYield as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Extraction".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Extraction {
            ship_symbol: intermediate_rep.ship_symbol.into_iter().next().ok_or_else(|| "shipSymbol missing in Extraction".to_string())?,
            r#yield: intermediate_rep.r#yield.into_iter().next().ok_or_else(|| "yield missing in Extraction".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Extraction> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Extraction>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Extraction>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Extraction - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Extraction> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Extraction as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Extraction - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExtractionYield {
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// The number of units extracted that were placed into the ship's cargo hold.
    #[serde(rename = "units")]
    pub units: i32,

}

impl ExtractionYield {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, units: i32, ) -> ExtractionYield {
        ExtractionYield {
            symbol,
            units,
        }
    }
}

/// Converts the ExtractionYield value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ExtractionYield {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExtractionYield value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExtractionYield {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub units: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExtractionYield".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExtractionYield".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExtractionYield {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ExtractionYield".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in ExtractionYield".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExtractionYield> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ExtractionYield>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExtractionYield>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExtractionYield - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ExtractionYield> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExtractionYield as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExtractionYield - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Faction {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "headquarters")]
    pub headquarters: String,

    #[serde(rename = "traits")]
    pub traits: Vec<models::FactionTrait>,

}

impl Faction {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, description: String, headquarters: String, traits: Vec<models::FactionTrait>, ) -> Faction {
        Faction {
            symbol,
            name,
            description,
            headquarters,
            traits,
        }
    }
}

/// Converts the Faction value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Faction {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),


            Some("headquarters".to_string()),
            Some(self.headquarters.to_string()),

            // Skipping traits in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Faction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Faction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub headquarters: Vec<String>,
            pub traits: Vec<Vec<models::FactionTrait>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Faction".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "headquarters" => intermediate_rep.headquarters.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "traits" => return std::result::Result::Err("Parsing a container in this style is not supported in Faction".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Faction".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Faction {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in Faction".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in Faction".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in Faction".to_string())?,
            headquarters: intermediate_rep.headquarters.into_iter().next().ok_or_else(|| "headquarters missing in Faction".to_string())?,
            traits: intermediate_rep.traits.into_iter().next().ok_or_else(|| "traits missing in Faction".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Faction> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Faction>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Faction>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Faction - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Faction> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Faction as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Faction - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FactionTrait {
    /// The unique identifier of the trait.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// The name of the trait.
    #[serde(rename = "name")]
    pub name: String,

    /// A description of the trait.
    #[serde(rename = "description")]
    pub description: String,

}

impl FactionTrait {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, description: String, ) -> FactionTrait {
        FactionTrait {
            symbol,
            name,
            description,
        }
    }
}

/// Converts the FactionTrait value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FactionTrait {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FactionTrait value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FactionTrait {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FactionTrait".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FactionTrait".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FactionTrait {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in FactionTrait".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in FactionTrait".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in FactionTrait".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FactionTrait> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FactionTrait>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FactionTrait>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FactionTrait - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FactionTrait> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FactionTrait as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FactionTrait - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FulfillContract200Response {
    #[serde(rename = "data")]
    pub data: models::AcceptContract200ResponseData,

}

impl FulfillContract200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::AcceptContract200ResponseData, ) -> FulfillContract200Response {
        FulfillContract200Response {
            data,
        }
    }
}

/// Converts the FulfillContract200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FulfillContract200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FulfillContract200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FulfillContract200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::AcceptContract200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FulfillContract200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::AcceptContract200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FulfillContract200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FulfillContract200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in FulfillContract200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FulfillContract200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FulfillContract200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FulfillContract200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FulfillContract200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FulfillContract200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FulfillContract200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FulfillContract200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetContract200Response {
    #[serde(rename = "data")]
    pub data: models::Contract,

}

impl GetContract200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Contract, ) -> GetContract200Response {
        GetContract200Response {
            data,
        }
    }
}

/// Converts the GetContract200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetContract200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetContract200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetContract200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Contract>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetContract200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Contract as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetContract200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetContract200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetContract200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetContract200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetContract200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetContract200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetContract200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetContract200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetContract200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetContract200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetContracts200Response {
    #[serde(rename = "data")]
    pub data: Vec<models::Contract>,

    #[serde(rename = "meta")]
    pub meta: models::Meta,

}

impl GetContracts200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<models::Contract>, meta: models::Meta, ) -> GetContracts200Response {
        GetContracts200Response {
            data,
            meta,
        }
    }
}

/// Converts the GetContracts200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetContracts200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

            // Skipping meta in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetContracts200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetContracts200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::Contract>>,
            pub meta: Vec<models::Meta>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetContracts200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in GetContracts200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "meta" => intermediate_rep.meta.push(<models::Meta as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetContracts200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetContracts200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetContracts200Response".to_string())?,
            meta: intermediate_rep.meta.into_iter().next().ok_or_else(|| "meta missing in GetContracts200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetContracts200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetContracts200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetContracts200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetContracts200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetContracts200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetContracts200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetContracts200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetFaction200Response {
    #[serde(rename = "data")]
    pub data: models::Faction,

}

impl GetFaction200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Faction, ) -> GetFaction200Response {
        GetFaction200Response {
            data,
        }
    }
}

/// Converts the GetFaction200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetFaction200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetFaction200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetFaction200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Faction>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetFaction200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Faction as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetFaction200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetFaction200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetFaction200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetFaction200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetFaction200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetFaction200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetFaction200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetFaction200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetFaction200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetFaction200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetFactions200Response {
    #[serde(rename = "data")]
    pub data: Vec<models::Faction>,

    #[serde(rename = "meta")]
    pub meta: models::Meta,

}

impl GetFactions200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<models::Faction>, meta: models::Meta, ) -> GetFactions200Response {
        GetFactions200Response {
            data,
            meta,
        }
    }
}

/// Converts the GetFactions200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetFactions200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

            // Skipping meta in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetFactions200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetFactions200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::Faction>>,
            pub meta: Vec<models::Meta>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetFactions200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in GetFactions200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "meta" => intermediate_rep.meta.push(<models::Meta as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetFactions200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetFactions200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetFactions200Response".to_string())?,
            meta: intermediate_rep.meta.into_iter().next().ok_or_else(|| "meta missing in GetFactions200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetFactions200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetFactions200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetFactions200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetFactions200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetFactions200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetFactions200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetFactions200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetJumpGate200Response {
    #[serde(rename = "data")]
    pub data: models::JumpGate,

}

impl GetJumpGate200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::JumpGate, ) -> GetJumpGate200Response {
        GetJumpGate200Response {
            data,
        }
    }
}

/// Converts the GetJumpGate200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetJumpGate200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetJumpGate200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetJumpGate200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::JumpGate>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetJumpGate200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::JumpGate as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetJumpGate200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetJumpGate200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetJumpGate200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetJumpGate200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetJumpGate200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetJumpGate200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetJumpGate200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetJumpGate200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetJumpGate200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetJumpGate200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMarket200Response {
    #[serde(rename = "data")]
    pub data: models::Market,

}

impl GetMarket200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Market, ) -> GetMarket200Response {
        GetMarket200Response {
            data,
        }
    }
}

/// Converts the GetMarket200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetMarket200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetMarket200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetMarket200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Market>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetMarket200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Market as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetMarket200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetMarket200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetMarket200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetMarket200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetMarket200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetMarket200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetMarket200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetMarket200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetMarket200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetMarket200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMyAgent200Response {
    #[serde(rename = "data")]
    pub data: models::Agent,

}

impl GetMyAgent200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Agent, ) -> GetMyAgent200Response {
        GetMyAgent200Response {
            data,
        }
    }
}

/// Converts the GetMyAgent200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetMyAgent200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetMyAgent200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetMyAgent200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Agent>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetMyAgent200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Agent as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetMyAgent200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetMyAgent200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetMyAgent200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetMyAgent200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetMyAgent200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetMyAgent200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetMyAgent200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetMyAgent200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetMyAgent200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetMyAgent200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMyShip200Response {
    #[serde(rename = "data")]
    pub data: models::Ship,

}

impl GetMyShip200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Ship, ) -> GetMyShip200Response {
        GetMyShip200Response {
            data,
        }
    }
}

/// Converts the GetMyShip200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetMyShip200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetMyShip200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetMyShip200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Ship>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetMyShip200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Ship as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetMyShip200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetMyShip200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetMyShip200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetMyShip200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetMyShip200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetMyShip200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetMyShip200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetMyShip200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetMyShip200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetMyShip200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMyShipCargo200Response {
    #[serde(rename = "data")]
    pub data: models::ShipCargo,

}

impl GetMyShipCargo200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::ShipCargo, ) -> GetMyShipCargo200Response {
        GetMyShipCargo200Response {
            data,
        }
    }
}

/// Converts the GetMyShipCargo200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetMyShipCargo200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetMyShipCargo200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetMyShipCargo200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::ShipCargo>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetMyShipCargo200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::ShipCargo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetMyShipCargo200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetMyShipCargo200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetMyShipCargo200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetMyShipCargo200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetMyShipCargo200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetMyShipCargo200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetMyShipCargo200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetMyShipCargo200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetMyShipCargo200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetMyShipCargo200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetMyShips200Response {
    #[serde(rename = "data")]
    pub data: Vec<models::Ship>,

    #[serde(rename = "meta")]
    pub meta: models::Meta,

}

impl GetMyShips200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<models::Ship>, meta: models::Meta, ) -> GetMyShips200Response {
        GetMyShips200Response {
            data,
            meta,
        }
    }
}

/// Converts the GetMyShips200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetMyShips200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

            // Skipping meta in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetMyShips200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetMyShips200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::Ship>>,
            pub meta: Vec<models::Meta>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetMyShips200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in GetMyShips200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "meta" => intermediate_rep.meta.push(<models::Meta as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetMyShips200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetMyShips200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetMyShips200Response".to_string())?,
            meta: intermediate_rep.meta.into_iter().next().ok_or_else(|| "meta missing in GetMyShips200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetMyShips200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetMyShips200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetMyShips200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetMyShips200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetMyShips200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetMyShips200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetMyShips200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetShipCooldown200Response {
    #[serde(rename = "data")]
    pub data: models::Cooldown,

}

impl GetShipCooldown200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Cooldown, ) -> GetShipCooldown200Response {
        GetShipCooldown200Response {
            data,
        }
    }
}

/// Converts the GetShipCooldown200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetShipCooldown200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetShipCooldown200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetShipCooldown200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Cooldown>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetShipCooldown200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Cooldown as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetShipCooldown200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetShipCooldown200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetShipCooldown200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetShipCooldown200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetShipCooldown200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetShipCooldown200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetShipCooldown200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetShipCooldown200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetShipCooldown200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetShipCooldown200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetShipNav200Response {
    #[serde(rename = "data")]
    pub data: models::ShipNav,

}

impl GetShipNav200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::ShipNav, ) -> GetShipNav200Response {
        GetShipNav200Response {
            data,
        }
    }
}

/// Converts the GetShipNav200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetShipNav200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetShipNav200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetShipNav200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::ShipNav>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetShipNav200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::ShipNav as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetShipNav200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetShipNav200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetShipNav200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetShipNav200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetShipNav200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetShipNav200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetShipNav200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetShipNav200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetShipNav200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetShipNav200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetShipyard200Response {
    #[serde(rename = "data")]
    pub data: models::Shipyard,

}

impl GetShipyard200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Shipyard, ) -> GetShipyard200Response {
        GetShipyard200Response {
            data,
        }
    }
}

/// Converts the GetShipyard200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetShipyard200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetShipyard200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetShipyard200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Shipyard>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetShipyard200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Shipyard as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetShipyard200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetShipyard200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetShipyard200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetShipyard200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetShipyard200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetShipyard200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetShipyard200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetShipyard200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetShipyard200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetShipyard200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSystem200Response {
    #[serde(rename = "data")]
    pub data: models::System,

}

impl GetSystem200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::System, ) -> GetSystem200Response {
        GetSystem200Response {
            data,
        }
    }
}

/// Converts the GetSystem200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetSystem200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetSystem200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetSystem200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::System>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetSystem200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::System as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetSystem200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetSystem200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetSystem200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetSystem200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetSystem200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetSystem200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetSystem200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetSystem200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetSystem200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetSystem200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSystemWaypoints200Response {
    #[serde(rename = "data")]
    pub data: Vec<models::Waypoint>,

    #[serde(rename = "meta")]
    pub meta: models::Meta,

}

impl GetSystemWaypoints200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<models::Waypoint>, meta: models::Meta, ) -> GetSystemWaypoints200Response {
        GetSystemWaypoints200Response {
            data,
            meta,
        }
    }
}

/// Converts the GetSystemWaypoints200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetSystemWaypoints200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

            // Skipping meta in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetSystemWaypoints200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetSystemWaypoints200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::Waypoint>>,
            pub meta: Vec<models::Meta>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetSystemWaypoints200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in GetSystemWaypoints200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "meta" => intermediate_rep.meta.push(<models::Meta as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetSystemWaypoints200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetSystemWaypoints200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetSystemWaypoints200Response".to_string())?,
            meta: intermediate_rep.meta.into_iter().next().ok_or_else(|| "meta missing in GetSystemWaypoints200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetSystemWaypoints200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetSystemWaypoints200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetSystemWaypoints200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetSystemWaypoints200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetSystemWaypoints200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetSystemWaypoints200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetSystemWaypoints200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetSystems200Response {
    #[serde(rename = "data")]
    pub data: Vec<models::System>,

    #[serde(rename = "meta")]
    pub meta: models::Meta,

}

impl GetSystems200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<models::System>, meta: models::Meta, ) -> GetSystems200Response {
        GetSystems200Response {
            data,
            meta,
        }
    }
}

/// Converts the GetSystems200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetSystems200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

            // Skipping meta in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetSystems200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetSystems200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::System>>,
            pub meta: Vec<models::Meta>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetSystems200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in GetSystems200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "meta" => intermediate_rep.meta.push(<models::Meta as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetSystems200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetSystems200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetSystems200Response".to_string())?,
            meta: intermediate_rep.meta.into_iter().next().ok_or_else(|| "meta missing in GetSystems200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetSystems200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetSystems200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetSystems200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetSystems200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetSystems200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetSystems200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetSystems200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetWaypoint200Response {
    #[serde(rename = "data")]
    pub data: models::Waypoint,

}

impl GetWaypoint200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Waypoint, ) -> GetWaypoint200Response {
        GetWaypoint200Response {
            data,
        }
    }
}

/// Converts the GetWaypoint200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetWaypoint200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetWaypoint200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetWaypoint200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Waypoint>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetWaypoint200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Waypoint as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetWaypoint200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetWaypoint200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in GetWaypoint200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetWaypoint200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetWaypoint200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetWaypoint200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetWaypoint200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetWaypoint200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetWaypoint200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetWaypoint200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Jettison200Response {
    #[serde(rename = "data")]
    pub data: models::Jettison200ResponseData,

}

impl Jettison200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Jettison200ResponseData, ) -> Jettison200Response {
        Jettison200Response {
            data,
        }
    }
}

/// Converts the Jettison200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Jettison200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Jettison200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Jettison200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Jettison200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Jettison200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Jettison200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Jettison200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Jettison200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in Jettison200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Jettison200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Jettison200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Jettison200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Jettison200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Jettison200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Jettison200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Jettison200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Jettison200ResponseData {
    #[serde(rename = "cargo")]
    pub cargo: models::ShipCargo,

}

impl Jettison200ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(cargo: models::ShipCargo, ) -> Jettison200ResponseData {
        Jettison200ResponseData {
            cargo,
        }
    }
}

/// Converts the Jettison200ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Jettison200ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cargo in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Jettison200ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Jettison200ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cargo: Vec<models::ShipCargo>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Jettison200ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cargo" => intermediate_rep.cargo.push(<models::ShipCargo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Jettison200ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Jettison200ResponseData {
            cargo: intermediate_rep.cargo.into_iter().next().ok_or_else(|| "cargo missing in Jettison200ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Jettison200ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Jettison200ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Jettison200ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Jettison200ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Jettison200ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Jettison200ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Jettison200ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JettisonRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "units")]
    pub units: u32,

}

impl JettisonRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, units: u32, ) -> JettisonRequest {
        JettisonRequest {
            symbol,
            units,
        }
    }
}

/// Converts the JettisonRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JettisonRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JettisonRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JettisonRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub units: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing JettisonRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing JettisonRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JettisonRequest {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in JettisonRequest".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in JettisonRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JettisonRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<JettisonRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<JettisonRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for JettisonRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<JettisonRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <JettisonRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into JettisonRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JumpGate {
    /// The maximum jump range of the gate.
    #[serde(rename = "jumpRange")]
    pub jump_range: f64,

    /// The symbol of the faction that owns the gate.
    #[serde(rename = "factionSymbol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub faction_symbol: Option<String>,

    /// The systems within range of the gate that have a corresponding gate.
    #[serde(rename = "connectedSystems")]
    pub connected_systems: Vec<models::ConnectedSystem>,

}

impl JumpGate {
    #[allow(clippy::new_without_default)]
    pub fn new(jump_range: f64, connected_systems: Vec<models::ConnectedSystem>, ) -> JumpGate {
        JumpGate {
            jump_range,
            faction_symbol: None,
            connected_systems,
        }
    }
}

/// Converts the JumpGate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JumpGate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("jumpRange".to_string()),
            Some(self.jump_range.to_string()),


            self.faction_symbol.as_ref().map(|faction_symbol| {
                vec![
                    "factionSymbol".to_string(),
                    faction_symbol.to_string(),
                ].join(",")
            }),

            // Skipping connectedSystems in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JumpGate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JumpGate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub jump_range: Vec<f64>,
            pub faction_symbol: Vec<String>,
            pub connected_systems: Vec<Vec<models::ConnectedSystem>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing JumpGate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "jumpRange" => intermediate_rep.jump_range.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "factionSymbol" => intermediate_rep.faction_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "connectedSystems" => return std::result::Result::Err("Parsing a container in this style is not supported in JumpGate".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing JumpGate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JumpGate {
            jump_range: intermediate_rep.jump_range.into_iter().next().ok_or_else(|| "jumpRange missing in JumpGate".to_string())?,
            faction_symbol: intermediate_rep.faction_symbol.into_iter().next(),
            connected_systems: intermediate_rep.connected_systems.into_iter().next().ok_or_else(|| "connectedSystems missing in JumpGate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JumpGate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<JumpGate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<JumpGate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for JumpGate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<JumpGate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <JumpGate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into JumpGate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JumpShip200Response {
    #[serde(rename = "data")]
    pub data: models::JumpShip200ResponseData,

}

impl JumpShip200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::JumpShip200ResponseData, ) -> JumpShip200Response {
        JumpShip200Response {
            data,
        }
    }
}

/// Converts the JumpShip200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JumpShip200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JumpShip200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JumpShip200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::JumpShip200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing JumpShip200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::JumpShip200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing JumpShip200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JumpShip200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in JumpShip200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JumpShip200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<JumpShip200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<JumpShip200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for JumpShip200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<JumpShip200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <JumpShip200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into JumpShip200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JumpShip200ResponseData {
    #[serde(rename = "cooldown")]
    pub cooldown: models::Cooldown,

    #[serde(rename = "nav")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nav: Option<models::ShipNav>,

}

impl JumpShip200ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(cooldown: models::Cooldown, ) -> JumpShip200ResponseData {
        JumpShip200ResponseData {
            cooldown,
            nav: None,
        }
    }
}

/// Converts the JumpShip200ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JumpShip200ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping nav in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JumpShip200ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JumpShip200ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::Cooldown>,
            pub nav: Vec<models::ShipNav>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing JumpShip200ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(<models::Cooldown as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nav" => intermediate_rep.nav.push(<models::ShipNav as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing JumpShip200ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JumpShip200ResponseData {
            cooldown: intermediate_rep.cooldown.into_iter().next().ok_or_else(|| "cooldown missing in JumpShip200ResponseData".to_string())?,
            nav: intermediate_rep.nav.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JumpShip200ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<JumpShip200ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<JumpShip200ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for JumpShip200ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<JumpShip200ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <JumpShip200ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into JumpShip200ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JumpShipRequest {
    /// The system symbol to jump to.
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,

}

impl JumpShipRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(system_symbol: String, ) -> JumpShipRequest {
        JumpShipRequest {
            system_symbol,
        }
    }
}

/// Converts the JumpShipRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JumpShipRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("systemSymbol".to_string()),
            Some(self.system_symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JumpShipRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JumpShipRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub system_symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing JumpShipRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "systemSymbol" => intermediate_rep.system_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing JumpShipRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JumpShipRequest {
            system_symbol: intermediate_rep.system_symbol.into_iter().next().ok_or_else(|| "systemSymbol missing in JumpShipRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<JumpShipRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<JumpShipRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<JumpShipRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for JumpShipRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<JumpShipRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <JumpShipRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into JumpShipRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Market {
    /// The symbol of the market. The symbol is the same as the waypoint where the market is located.
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// The list of goods that are exported from this market.
    #[serde(rename = "exports")]
    pub exports: Vec<models::TradeGood>,

    /// The list of goods that are sought as imports in this market.
    #[serde(rename = "imports")]
    pub imports: Vec<models::TradeGood>,

    /// The list of goods that are bought and sold between agents at this market.
    #[serde(rename = "exchange")]
    pub exchange: Vec<models::TradeGood>,

    /// The list of recent transactions at this market. Visible only when a ship is present at the market.
    #[serde(rename = "transactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transactions: Option<Vec<models::MarketTransaction>>,

    /// The list of goods that are traded at this market. Visible only when a ship is present at the market.
    #[serde(rename = "tradeGoods")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trade_goods: Option<Vec<models::MarketTradeGood>>,

}

impl Market {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, exports: Vec<models::TradeGood>, imports: Vec<models::TradeGood>, exchange: Vec<models::TradeGood>, ) -> Market {
        Market {
            symbol,
            exports,
            imports,
            exchange,
            transactions: None,
            trade_goods: None,
        }
    }
}

/// Converts the Market value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Market {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping exports in query parameter serialization

            // Skipping imports in query parameter serialization

            // Skipping exchange in query parameter serialization

            // Skipping transactions in query parameter serialization

            // Skipping tradeGoods in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Market value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Market {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub exports: Vec<Vec<models::TradeGood>>,
            pub imports: Vec<Vec<models::TradeGood>>,
            pub exchange: Vec<Vec<models::TradeGood>>,
            pub transactions: Vec<Vec<models::MarketTransaction>>,
            pub trade_goods: Vec<Vec<models::MarketTradeGood>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Market".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "exports" => return std::result::Result::Err("Parsing a container in this style is not supported in Market".to_string()),
                    "imports" => return std::result::Result::Err("Parsing a container in this style is not supported in Market".to_string()),
                    "exchange" => return std::result::Result::Err("Parsing a container in this style is not supported in Market".to_string()),
                    "transactions" => return std::result::Result::Err("Parsing a container in this style is not supported in Market".to_string()),
                    "tradeGoods" => return std::result::Result::Err("Parsing a container in this style is not supported in Market".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Market".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Market {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in Market".to_string())?,
            exports: intermediate_rep.exports.into_iter().next().ok_or_else(|| "exports missing in Market".to_string())?,
            imports: intermediate_rep.imports.into_iter().next().ok_or_else(|| "imports missing in Market".to_string())?,
            exchange: intermediate_rep.exchange.into_iter().next().ok_or_else(|| "exchange missing in Market".to_string())?,
            transactions: intermediate_rep.transactions.into_iter().next(),
            trade_goods: intermediate_rep.trade_goods.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Market> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Market>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Market>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Market - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Market> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Market as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Market - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MarketTradeGood {
    /// The symbol of the trade good.
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// The typical volume flowing through the market for this type of good. The larger the trade volume, the more stable prices will be.
    #[serde(rename = "tradeVolume")]
    pub trade_volume: u32,

    /// A rough estimate of the total supply of this good in the marketplace.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "supply")]
    pub supply: String,

    /// The price at which this good can be purchased from the market.
    #[serde(rename = "purchasePrice")]
    pub purchase_price: u32,

    /// The price at which this good can be sold to the market.
    #[serde(rename = "sellPrice")]
    pub sell_price: u32,

}

impl MarketTradeGood {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, trade_volume: u32, supply: String, purchase_price: u32, sell_price: u32, ) -> MarketTradeGood {
        MarketTradeGood {
            symbol,
            trade_volume,
            supply,
            purchase_price,
            sell_price,
        }
    }
}

/// Converts the MarketTradeGood value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MarketTradeGood {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("tradeVolume".to_string()),
            Some(self.trade_volume.to_string()),


            Some("supply".to_string()),
            Some(self.supply.to_string()),


            Some("purchasePrice".to_string()),
            Some(self.purchase_price.to_string()),


            Some("sellPrice".to_string()),
            Some(self.sell_price.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MarketTradeGood value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MarketTradeGood {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub trade_volume: Vec<u32>,
            pub supply: Vec<String>,
            pub purchase_price: Vec<u32>,
            pub sell_price: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MarketTradeGood".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tradeVolume" => intermediate_rep.trade_volume.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "supply" => intermediate_rep.supply.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "purchasePrice" => intermediate_rep.purchase_price.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sellPrice" => intermediate_rep.sell_price.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MarketTradeGood".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MarketTradeGood {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in MarketTradeGood".to_string())?,
            trade_volume: intermediate_rep.trade_volume.into_iter().next().ok_or_else(|| "tradeVolume missing in MarketTradeGood".to_string())?,
            supply: intermediate_rep.supply.into_iter().next().ok_or_else(|| "supply missing in MarketTradeGood".to_string())?,
            purchase_price: intermediate_rep.purchase_price.into_iter().next().ok_or_else(|| "purchasePrice missing in MarketTradeGood".to_string())?,
            sell_price: intermediate_rep.sell_price.into_iter().next().ok_or_else(|| "sellPrice missing in MarketTradeGood".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MarketTradeGood> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MarketTradeGood>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MarketTradeGood>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MarketTradeGood - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MarketTradeGood> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MarketTradeGood as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MarketTradeGood - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MarketTransaction {
    /// The symbol of the waypoint where the transaction took place.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,

    /// The symbol of the ship that made the transaction.
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,

    /// The symbol of the trade good.
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,

    /// The type of transaction.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    /// The number of units of the transaction.
    #[serde(rename = "units")]
    pub units: u32,

    /// The price per unit of the transaction.
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: u32,

    /// The total price of the transaction.
    #[serde(rename = "totalPrice")]
    pub total_price: u32,

    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime::<chrono::Utc>,

}

impl MarketTransaction {
    #[allow(clippy::new_without_default)]
    pub fn new(waypoint_symbol: String, ship_symbol: String, trade_symbol: String, r#type: String, units: u32, price_per_unit: u32, total_price: u32, timestamp: chrono::DateTime::<chrono::Utc>, ) -> MarketTransaction {
        MarketTransaction {
            waypoint_symbol,
            ship_symbol,
            trade_symbol,
            r#type,
            units,
            price_per_unit,
            total_price,
            timestamp,
        }
    }
}

/// Converts the MarketTransaction value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MarketTransaction {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("waypointSymbol".to_string()),
            Some(self.waypoint_symbol.to_string()),


            Some("shipSymbol".to_string()),
            Some(self.ship_symbol.to_string()),


            Some("tradeSymbol".to_string()),
            Some(self.trade_symbol.to_string()),


            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),


            Some("pricePerUnit".to_string()),
            Some(self.price_per_unit.to_string()),


            Some("totalPrice".to_string()),
            Some(self.total_price.to_string()),

            // Skipping timestamp in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MarketTransaction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MarketTransaction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub waypoint_symbol: Vec<String>,
            pub ship_symbol: Vec<String>,
            pub trade_symbol: Vec<String>,
            pub r#type: Vec<String>,
            pub units: Vec<u32>,
            pub price_per_unit: Vec<u32>,
            pub total_price: Vec<u32>,
            pub timestamp: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MarketTransaction".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "waypointSymbol" => intermediate_rep.waypoint_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "shipSymbol" => intermediate_rep.ship_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tradeSymbol" => intermediate_rep.trade_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pricePerUnit" => intermediate_rep.price_per_unit.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "totalPrice" => intermediate_rep.total_price.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp" => intermediate_rep.timestamp.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MarketTransaction".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MarketTransaction {
            waypoint_symbol: intermediate_rep.waypoint_symbol.into_iter().next().ok_or_else(|| "waypointSymbol missing in MarketTransaction".to_string())?,
            ship_symbol: intermediate_rep.ship_symbol.into_iter().next().ok_or_else(|| "shipSymbol missing in MarketTransaction".to_string())?,
            trade_symbol: intermediate_rep.trade_symbol.into_iter().next().ok_or_else(|| "tradeSymbol missing in MarketTransaction".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in MarketTransaction".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in MarketTransaction".to_string())?,
            price_per_unit: intermediate_rep.price_per_unit.into_iter().next().ok_or_else(|| "pricePerUnit missing in MarketTransaction".to_string())?,
            total_price: intermediate_rep.total_price.into_iter().next().ok_or_else(|| "totalPrice missing in MarketTransaction".to_string())?,
            timestamp: intermediate_rep.timestamp.into_iter().next().ok_or_else(|| "timestamp missing in MarketTransaction".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MarketTransaction> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MarketTransaction>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MarketTransaction>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MarketTransaction - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MarketTransaction> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MarketTransaction as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MarketTransaction - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Meta {
    #[serde(rename = "total")]
    pub total: i32,

    #[serde(rename = "page")]
    pub page: i32,

    #[serde(rename = "limit")]
    pub limit: i32,

}

impl Meta {
    #[allow(clippy::new_without_default)]
    pub fn new(total: i32, page: i32, limit: i32, ) -> Meta {
        Meta {
            total,
            page,
            limit,
        }
    }
}

/// Converts the Meta value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Meta {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("total".to_string()),
            Some(self.total.to_string()),


            Some("page".to_string()),
            Some(self.page.to_string()),


            Some("limit".to_string()),
            Some(self.limit.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Meta value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Meta {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub total: Vec<i32>,
            pub page: Vec<i32>,
            pub limit: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Meta".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "total" => intermediate_rep.total.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "page" => intermediate_rep.page.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "limit" => intermediate_rep.limit.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Meta".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Meta {
            total: intermediate_rep.total.into_iter().next().ok_or_else(|| "total missing in Meta".to_string())?,
            page: intermediate_rep.page.into_iter().next().ok_or_else(|| "page missing in Meta".to_string())?,
            limit: intermediate_rep.limit.into_iter().next().ok_or_else(|| "limit missing in Meta".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Meta> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Meta>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Meta>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Meta - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Meta> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Meta as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Meta - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NavigateShip200Response {
    #[serde(rename = "data")]
    pub data: models::NavigateShip200ResponseData,

}

impl NavigateShip200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::NavigateShip200ResponseData, ) -> NavigateShip200Response {
        NavigateShip200Response {
            data,
        }
    }
}

/// Converts the NavigateShip200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NavigateShip200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NavigateShip200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NavigateShip200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::NavigateShip200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NavigateShip200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::NavigateShip200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NavigateShip200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NavigateShip200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in NavigateShip200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NavigateShip200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NavigateShip200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NavigateShip200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NavigateShip200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NavigateShip200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NavigateShip200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NavigateShip200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NavigateShip200ResponseData {
    #[serde(rename = "fuel")]
    pub fuel: models::ShipFuel,

    #[serde(rename = "nav")]
    pub nav: models::ShipNav,

}

impl NavigateShip200ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(fuel: models::ShipFuel, nav: models::ShipNav, ) -> NavigateShip200ResponseData {
        NavigateShip200ResponseData {
            fuel,
            nav,
        }
    }
}

/// Converts the NavigateShip200ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NavigateShip200ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping fuel in query parameter serialization

            // Skipping nav in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NavigateShip200ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NavigateShip200ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub fuel: Vec<models::ShipFuel>,
            pub nav: Vec<models::ShipNav>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NavigateShip200ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "fuel" => intermediate_rep.fuel.push(<models::ShipFuel as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nav" => intermediate_rep.nav.push(<models::ShipNav as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NavigateShip200ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NavigateShip200ResponseData {
            fuel: intermediate_rep.fuel.into_iter().next().ok_or_else(|| "fuel missing in NavigateShip200ResponseData".to_string())?,
            nav: intermediate_rep.nav.into_iter().next().ok_or_else(|| "nav missing in NavigateShip200ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NavigateShip200ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NavigateShip200ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NavigateShip200ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NavigateShip200ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NavigateShip200ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NavigateShip200ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NavigateShip200ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NavigateShipRequest {
    /// The target destination.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,

}

impl NavigateShipRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(waypoint_symbol: String, ) -> NavigateShipRequest {
        NavigateShipRequest {
            waypoint_symbol,
        }
    }
}

/// Converts the NavigateShipRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NavigateShipRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("waypointSymbol".to_string()),
            Some(self.waypoint_symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NavigateShipRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NavigateShipRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub waypoint_symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NavigateShipRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "waypointSymbol" => intermediate_rep.waypoint_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NavigateShipRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NavigateShipRequest {
            waypoint_symbol: intermediate_rep.waypoint_symbol.into_iter().next().ok_or_else(|| "waypointSymbol missing in NavigateShipRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NavigateShipRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NavigateShipRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NavigateShipRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NavigateShipRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NavigateShipRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NavigateShipRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NavigateShipRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OrbitShip200Response {
    #[serde(rename = "data")]
    pub data: models::OrbitShip200ResponseData,

}

impl OrbitShip200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::OrbitShip200ResponseData, ) -> OrbitShip200Response {
        OrbitShip200Response {
            data,
        }
    }
}

/// Converts the OrbitShip200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OrbitShip200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OrbitShip200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OrbitShip200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::OrbitShip200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OrbitShip200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::OrbitShip200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing OrbitShip200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OrbitShip200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in OrbitShip200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OrbitShip200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OrbitShip200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OrbitShip200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OrbitShip200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OrbitShip200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OrbitShip200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OrbitShip200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OrbitShip200ResponseData {
    #[serde(rename = "nav")]
    pub nav: models::ShipNav,

}

impl OrbitShip200ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(nav: models::ShipNav, ) -> OrbitShip200ResponseData {
        OrbitShip200ResponseData {
            nav,
        }
    }
}

/// Converts the OrbitShip200ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OrbitShip200ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping nav in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OrbitShip200ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OrbitShip200ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub nav: Vec<models::ShipNav>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OrbitShip200ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "nav" => intermediate_rep.nav.push(<models::ShipNav as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing OrbitShip200ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OrbitShip200ResponseData {
            nav: intermediate_rep.nav.into_iter().next().ok_or_else(|| "nav missing in OrbitShip200ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OrbitShip200ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OrbitShip200ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OrbitShip200ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OrbitShip200ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OrbitShip200ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OrbitShip200ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OrbitShip200ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PatchShipNavRequest {
    #[serde(rename = "flightMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub flight_mode: Option<models::ShipNavFlightMode>,

}

impl PatchShipNavRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PatchShipNavRequest {
        PatchShipNavRequest {
            flight_mode: None,
        }
    }
}

/// Converts the PatchShipNavRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PatchShipNavRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping flightMode in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PatchShipNavRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PatchShipNavRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub flight_mode: Vec<models::ShipNavFlightMode>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PatchShipNavRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "flightMode" => intermediate_rep.flight_mode.push(<models::ShipNavFlightMode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PatchShipNavRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PatchShipNavRequest {
            flight_mode: intermediate_rep.flight_mode.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PatchShipNavRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PatchShipNavRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PatchShipNavRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PatchShipNavRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PatchShipNavRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PatchShipNavRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PatchShipNavRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PurchaseCargo201Response {
    #[serde(rename = "data")]
    pub data: models::SellCargo201ResponseData,

}

impl PurchaseCargo201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::SellCargo201ResponseData, ) -> PurchaseCargo201Response {
        PurchaseCargo201Response {
            data,
        }
    }
}

/// Converts the PurchaseCargo201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PurchaseCargo201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PurchaseCargo201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PurchaseCargo201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::SellCargo201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PurchaseCargo201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::SellCargo201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PurchaseCargo201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PurchaseCargo201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in PurchaseCargo201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PurchaseCargo201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PurchaseCargo201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PurchaseCargo201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PurchaseCargo201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PurchaseCargo201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PurchaseCargo201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PurchaseCargo201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PurchaseCargoRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "units")]
    pub units: i32,

}

impl PurchaseCargoRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, units: i32, ) -> PurchaseCargoRequest {
        PurchaseCargoRequest {
            symbol,
            units,
        }
    }
}

/// Converts the PurchaseCargoRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PurchaseCargoRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PurchaseCargoRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PurchaseCargoRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub units: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PurchaseCargoRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PurchaseCargoRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PurchaseCargoRequest {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in PurchaseCargoRequest".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in PurchaseCargoRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PurchaseCargoRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PurchaseCargoRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PurchaseCargoRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PurchaseCargoRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PurchaseCargoRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PurchaseCargoRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PurchaseCargoRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PurchaseShip201Response {
    #[serde(rename = "data")]
    pub data: models::PurchaseShip201ResponseData,

}

impl PurchaseShip201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::PurchaseShip201ResponseData, ) -> PurchaseShip201Response {
        PurchaseShip201Response {
            data,
        }
    }
}

/// Converts the PurchaseShip201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PurchaseShip201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PurchaseShip201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PurchaseShip201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::PurchaseShip201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PurchaseShip201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::PurchaseShip201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PurchaseShip201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PurchaseShip201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in PurchaseShip201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PurchaseShip201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PurchaseShip201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PurchaseShip201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PurchaseShip201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PurchaseShip201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PurchaseShip201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PurchaseShip201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PurchaseShip201ResponseData {
    #[serde(rename = "agent")]
    pub agent: models::Agent,

    #[serde(rename = "ship")]
    pub ship: models::Ship,

    #[serde(rename = "transaction")]
    pub transaction: models::ShipyardTransaction,

}

impl PurchaseShip201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(agent: models::Agent, ship: models::Ship, transaction: models::ShipyardTransaction, ) -> PurchaseShip201ResponseData {
        PurchaseShip201ResponseData {
            agent,
            ship,
            transaction,
        }
    }
}

/// Converts the PurchaseShip201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PurchaseShip201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping agent in query parameter serialization

            // Skipping ship in query parameter serialization

            // Skipping transaction in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PurchaseShip201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PurchaseShip201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub agent: Vec<models::Agent>,
            pub ship: Vec<models::Ship>,
            pub transaction: Vec<models::ShipyardTransaction>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PurchaseShip201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "agent" => intermediate_rep.agent.push(<models::Agent as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ship" => intermediate_rep.ship.push(<models::Ship as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "transaction" => intermediate_rep.transaction.push(<models::ShipyardTransaction as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PurchaseShip201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PurchaseShip201ResponseData {
            agent: intermediate_rep.agent.into_iter().next().ok_or_else(|| "agent missing in PurchaseShip201ResponseData".to_string())?,
            ship: intermediate_rep.ship.into_iter().next().ok_or_else(|| "ship missing in PurchaseShip201ResponseData".to_string())?,
            transaction: intermediate_rep.transaction.into_iter().next().ok_or_else(|| "transaction missing in PurchaseShip201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PurchaseShip201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PurchaseShip201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PurchaseShip201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PurchaseShip201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PurchaseShip201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PurchaseShip201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PurchaseShip201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PurchaseShipRequest {
    #[serde(rename = "shipType")]
    pub ship_type: models::ShipType,

    /// The symbol of the waypoint you want to purchase the ship at.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,

}

impl PurchaseShipRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(ship_type: models::ShipType, waypoint_symbol: String, ) -> PurchaseShipRequest {
        PurchaseShipRequest {
            ship_type,
            waypoint_symbol,
        }
    }
}

/// Converts the PurchaseShipRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PurchaseShipRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping shipType in query parameter serialization


            Some("waypointSymbol".to_string()),
            Some(self.waypoint_symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PurchaseShipRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PurchaseShipRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub ship_type: Vec<models::ShipType>,
            pub waypoint_symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PurchaseShipRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "shipType" => intermediate_rep.ship_type.push(<models::ShipType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "waypointSymbol" => intermediate_rep.waypoint_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PurchaseShipRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PurchaseShipRequest {
            ship_type: intermediate_rep.ship_type.into_iter().next().ok_or_else(|| "shipType missing in PurchaseShipRequest".to_string())?,
            waypoint_symbol: intermediate_rep.waypoint_symbol.into_iter().next().ok_or_else(|| "waypointSymbol missing in PurchaseShipRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PurchaseShipRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PurchaseShipRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PurchaseShipRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PurchaseShipRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PurchaseShipRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PurchaseShipRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PurchaseShipRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RefuelShip200Response {
    #[serde(rename = "data")]
    pub data: models::RefuelShip200ResponseData,

}

impl RefuelShip200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::RefuelShip200ResponseData, ) -> RefuelShip200Response {
        RefuelShip200Response {
            data,
        }
    }
}

/// Converts the RefuelShip200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RefuelShip200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RefuelShip200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RefuelShip200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::RefuelShip200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RefuelShip200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::RefuelShip200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RefuelShip200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RefuelShip200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in RefuelShip200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RefuelShip200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RefuelShip200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RefuelShip200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RefuelShip200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RefuelShip200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RefuelShip200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RefuelShip200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RefuelShip200ResponseData {
    #[serde(rename = "agent")]
    pub agent: models::Agent,

    #[serde(rename = "fuel")]
    pub fuel: models::ShipFuel,

}

impl RefuelShip200ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(agent: models::Agent, fuel: models::ShipFuel, ) -> RefuelShip200ResponseData {
        RefuelShip200ResponseData {
            agent,
            fuel,
        }
    }
}

/// Converts the RefuelShip200ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RefuelShip200ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping agent in query parameter serialization

            // Skipping fuel in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RefuelShip200ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RefuelShip200ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub agent: Vec<models::Agent>,
            pub fuel: Vec<models::ShipFuel>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RefuelShip200ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "agent" => intermediate_rep.agent.push(<models::Agent as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "fuel" => intermediate_rep.fuel.push(<models::ShipFuel as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RefuelShip200ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RefuelShip200ResponseData {
            agent: intermediate_rep.agent.into_iter().next().ok_or_else(|| "agent missing in RefuelShip200ResponseData".to_string())?,
            fuel: intermediate_rep.fuel.into_iter().next().ok_or_else(|| "fuel missing in RefuelShip200ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RefuelShip200ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RefuelShip200ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RefuelShip200ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RefuelShip200ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RefuelShip200ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RefuelShip200ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RefuelShip200ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Register201Response {
    #[serde(rename = "data")]
    pub data: models::Register201ResponseData,

}

impl Register201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Register201ResponseData, ) -> Register201Response {
        Register201Response {
            data,
        }
    }
}

/// Converts the Register201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Register201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Register201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Register201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Register201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Register201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Register201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Register201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Register201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in Register201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Register201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Register201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Register201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Register201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Register201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Register201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Register201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Register201ResponseData {
    #[serde(rename = "agent")]
    pub agent: models::Agent,

    #[serde(rename = "contract")]
    pub contract: models::Contract,

    #[serde(rename = "faction")]
    pub faction: models::Faction,

    #[serde(rename = "ship")]
    pub ship: models::Ship,

    /// A Bearer token for accessing secured API endpoints.
    #[serde(rename = "token")]
    pub token: String,

}

impl Register201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(agent: models::Agent, contract: models::Contract, faction: models::Faction, ship: models::Ship, token: String, ) -> Register201ResponseData {
        Register201ResponseData {
            agent,
            contract,
            faction,
            ship,
            token,
        }
    }
}

/// Converts the Register201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Register201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping agent in query parameter serialization

            // Skipping contract in query parameter serialization

            // Skipping faction in query parameter serialization

            // Skipping ship in query parameter serialization


            Some("token".to_string()),
            Some(self.token.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Register201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Register201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub agent: Vec<models::Agent>,
            pub contract: Vec<models::Contract>,
            pub faction: Vec<models::Faction>,
            pub ship: Vec<models::Ship>,
            pub token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Register201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "agent" => intermediate_rep.agent.push(<models::Agent as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contract" => intermediate_rep.contract.push(<models::Contract as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "faction" => intermediate_rep.faction.push(<models::Faction as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ship" => intermediate_rep.ship.push(<models::Ship as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "token" => intermediate_rep.token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Register201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Register201ResponseData {
            agent: intermediate_rep.agent.into_iter().next().ok_or_else(|| "agent missing in Register201ResponseData".to_string())?,
            contract: intermediate_rep.contract.into_iter().next().ok_or_else(|| "contract missing in Register201ResponseData".to_string())?,
            faction: intermediate_rep.faction.into_iter().next().ok_or_else(|| "faction missing in Register201ResponseData".to_string())?,
            ship: intermediate_rep.ship.into_iter().next().ok_or_else(|| "ship missing in Register201ResponseData".to_string())?,
            token: intermediate_rep.token.into_iter().next().ok_or_else(|| "token missing in Register201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Register201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Register201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Register201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Register201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Register201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Register201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Register201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegisterRequest {
    /// The faction you choose determines your headquarters.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "faction")]
    pub faction: String,

    /// How other agents will see your ships and information.
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl RegisterRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(faction: String, symbol: String, ) -> RegisterRequest {
        RegisterRequest {
            faction,
            symbol,
        }
    }
}

/// Converts the RegisterRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RegisterRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("faction".to_string()),
            Some(self.faction.to_string()),


            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RegisterRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RegisterRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub faction: Vec<String>,
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RegisterRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "faction" => intermediate_rep.faction.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RegisterRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RegisterRequest {
            faction: intermediate_rep.faction.into_iter().next().ok_or_else(|| "faction missing in RegisterRequest".to_string())?,
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in RegisterRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RegisterRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RegisterRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RegisterRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RegisterRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RegisterRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RegisterRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RegisterRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The ship that was scanned. Details include information about the ship that could be detected by the scanner.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScannedShip {
    /// The globally unique identifier of the ship.
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "registration")]
    pub registration: models::ShipRegistration,

    #[serde(rename = "nav")]
    pub nav: models::ShipNav,

    #[serde(rename = "frame")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub frame: Option<models::ScannedShipFrame>,

    #[serde(rename = "reactor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactor: Option<models::ScannedShipReactor>,

    #[serde(rename = "engine")]
    pub engine: models::ScannedShipEngine,

    #[serde(rename = "mounts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mounts: Option<Vec<models::ScannedShipMountsInner>>,

}

impl ScannedShip {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, registration: models::ShipRegistration, nav: models::ShipNav, engine: models::ScannedShipEngine, ) -> ScannedShip {
        ScannedShip {
            symbol,
            registration,
            nav,
            frame: None,
            reactor: None,
            engine,
            mounts: None,
        }
    }
}

/// Converts the ScannedShip value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScannedShip {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping registration in query parameter serialization

            // Skipping nav in query parameter serialization

            // Skipping frame in query parameter serialization

            // Skipping reactor in query parameter serialization

            // Skipping engine in query parameter serialization

            // Skipping mounts in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScannedShip value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScannedShip {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub registration: Vec<models::ShipRegistration>,
            pub nav: Vec<models::ShipNav>,
            pub frame: Vec<models::ScannedShipFrame>,
            pub reactor: Vec<models::ScannedShipReactor>,
            pub engine: Vec<models::ScannedShipEngine>,
            pub mounts: Vec<Vec<models::ScannedShipMountsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScannedShip".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "registration" => intermediate_rep.registration.push(<models::ShipRegistration as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nav" => intermediate_rep.nav.push(<models::ShipNav as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "frame" => intermediate_rep.frame.push(<models::ScannedShipFrame as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reactor" => intermediate_rep.reactor.push(<models::ScannedShipReactor as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "engine" => intermediate_rep.engine.push(<models::ScannedShipEngine as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "mounts" => return std::result::Result::Err("Parsing a container in this style is not supported in ScannedShip".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScannedShip".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScannedShip {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ScannedShip".to_string())?,
            registration: intermediate_rep.registration.into_iter().next().ok_or_else(|| "registration missing in ScannedShip".to_string())?,
            nav: intermediate_rep.nav.into_iter().next().ok_or_else(|| "nav missing in ScannedShip".to_string())?,
            frame: intermediate_rep.frame.into_iter().next(),
            reactor: intermediate_rep.reactor.into_iter().next(),
            engine: intermediate_rep.engine.into_iter().next().ok_or_else(|| "engine missing in ScannedShip".to_string())?,
            mounts: intermediate_rep.mounts.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScannedShip> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScannedShip>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScannedShip>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScannedShip - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScannedShip> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScannedShip as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScannedShip - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The engine of the ship.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScannedShipEngine {
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl ScannedShipEngine {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ) -> ScannedShipEngine {
        ScannedShipEngine {
            symbol,
        }
    }
}

/// Converts the ScannedShipEngine value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScannedShipEngine {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScannedShipEngine value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScannedShipEngine {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScannedShipEngine".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScannedShipEngine".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScannedShipEngine {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ScannedShipEngine".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScannedShipEngine> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScannedShipEngine>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScannedShipEngine>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScannedShipEngine - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScannedShipEngine> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScannedShipEngine as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScannedShipEngine - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The frame of the ship.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScannedShipFrame {
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl ScannedShipFrame {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ) -> ScannedShipFrame {
        ScannedShipFrame {
            symbol,
        }
    }
}

/// Converts the ScannedShipFrame value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScannedShipFrame {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScannedShipFrame value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScannedShipFrame {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScannedShipFrame".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScannedShipFrame".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScannedShipFrame {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ScannedShipFrame".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScannedShipFrame> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScannedShipFrame>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScannedShipFrame>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScannedShipFrame - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScannedShipFrame> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScannedShipFrame as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScannedShipFrame - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A mount on the ship.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScannedShipMountsInner {
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl ScannedShipMountsInner {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ) -> ScannedShipMountsInner {
        ScannedShipMountsInner {
            symbol,
        }
    }
}

/// Converts the ScannedShipMountsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScannedShipMountsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScannedShipMountsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScannedShipMountsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScannedShipMountsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScannedShipMountsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScannedShipMountsInner {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ScannedShipMountsInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScannedShipMountsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScannedShipMountsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScannedShipMountsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScannedShipMountsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScannedShipMountsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScannedShipMountsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScannedShipMountsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The reactor of the ship.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScannedShipReactor {
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl ScannedShipReactor {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ) -> ScannedShipReactor {
        ScannedShipReactor {
            symbol,
        }
    }
}

/// Converts the ScannedShipReactor value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScannedShipReactor {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScannedShipReactor value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScannedShipReactor {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScannedShipReactor".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScannedShipReactor".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScannedShipReactor {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ScannedShipReactor".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScannedShipReactor> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScannedShipReactor>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScannedShipReactor>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScannedShipReactor - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScannedShipReactor> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScannedShipReactor as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScannedShipReactor - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScannedSystem {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "sectorSymbol")]
    pub sector_symbol: String,

    #[serde(rename = "type")]
    pub r#type: models::SystemType,

    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

    #[serde(rename = "distance")]
    pub distance: i32,

}

impl ScannedSystem {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, sector_symbol: String, r#type: models::SystemType, x: i32, y: i32, distance: i32, ) -> ScannedSystem {
        ScannedSystem {
            symbol,
            sector_symbol,
            r#type,
            x,
            y,
            distance,
        }
    }
}

/// Converts the ScannedSystem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScannedSystem {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("sectorSymbol".to_string()),
            Some(self.sector_symbol.to_string()),

            // Skipping type in query parameter serialization


            Some("x".to_string()),
            Some(self.x.to_string()),


            Some("y".to_string()),
            Some(self.y.to_string()),


            Some("distance".to_string()),
            Some(self.distance.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScannedSystem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScannedSystem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub sector_symbol: Vec<String>,
            pub r#type: Vec<models::SystemType>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
            pub distance: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScannedSystem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sectorSymbol" => intermediate_rep.sector_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::SystemType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "distance" => intermediate_rep.distance.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScannedSystem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScannedSystem {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ScannedSystem".to_string())?,
            sector_symbol: intermediate_rep.sector_symbol.into_iter().next().ok_or_else(|| "sectorSymbol missing in ScannedSystem".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ScannedSystem".to_string())?,
            x: intermediate_rep.x.into_iter().next().ok_or_else(|| "x missing in ScannedSystem".to_string())?,
            y: intermediate_rep.y.into_iter().next().ok_or_else(|| "y missing in ScannedSystem".to_string())?,
            distance: intermediate_rep.distance.into_iter().next().ok_or_else(|| "distance missing in ScannedSystem".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScannedSystem> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScannedSystem>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScannedSystem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScannedSystem - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScannedSystem> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScannedSystem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScannedSystem - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScannedWaypoint {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "type")]
    pub r#type: models::WaypointType,

    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,

    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

    #[serde(rename = "orbitals")]
    pub orbitals: Vec<models::WaypointOrbital>,

    #[serde(rename = "faction")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub faction: Option<models::WaypointFaction>,

    /// The traits of the waypoint.
    #[serde(rename = "traits")]
    pub traits: Vec<models::WaypointTrait>,

    #[serde(rename = "chart")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chart: Option<models::Chart>,

}

impl ScannedWaypoint {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, r#type: models::WaypointType, system_symbol: String, x: i32, y: i32, orbitals: Vec<models::WaypointOrbital>, traits: Vec<models::WaypointTrait>, ) -> ScannedWaypoint {
        ScannedWaypoint {
            symbol,
            r#type,
            system_symbol,
            x,
            y,
            orbitals,
            faction: None,
            traits,
            chart: None,
        }
    }
}

/// Converts the ScannedWaypoint value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScannedWaypoint {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping type in query parameter serialization


            Some("systemSymbol".to_string()),
            Some(self.system_symbol.to_string()),


            Some("x".to_string()),
            Some(self.x.to_string()),


            Some("y".to_string()),
            Some(self.y.to_string()),

            // Skipping orbitals in query parameter serialization

            // Skipping faction in query parameter serialization

            // Skipping traits in query parameter serialization

            // Skipping chart in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScannedWaypoint value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScannedWaypoint {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub r#type: Vec<models::WaypointType>,
            pub system_symbol: Vec<String>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
            pub orbitals: Vec<Vec<models::WaypointOrbital>>,
            pub faction: Vec<models::WaypointFaction>,
            pub traits: Vec<Vec<models::WaypointTrait>>,
            pub chart: Vec<models::Chart>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScannedWaypoint".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::WaypointType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "systemSymbol" => intermediate_rep.system_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "orbitals" => return std::result::Result::Err("Parsing a container in this style is not supported in ScannedWaypoint".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "faction" => intermediate_rep.faction.push(<models::WaypointFaction as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "traits" => return std::result::Result::Err("Parsing a container in this style is not supported in ScannedWaypoint".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "chart" => intermediate_rep.chart.push(<models::Chart as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScannedWaypoint".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScannedWaypoint {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ScannedWaypoint".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ScannedWaypoint".to_string())?,
            system_symbol: intermediate_rep.system_symbol.into_iter().next().ok_or_else(|| "systemSymbol missing in ScannedWaypoint".to_string())?,
            x: intermediate_rep.x.into_iter().next().ok_or_else(|| "x missing in ScannedWaypoint".to_string())?,
            y: intermediate_rep.y.into_iter().next().ok_or_else(|| "y missing in ScannedWaypoint".to_string())?,
            orbitals: intermediate_rep.orbitals.into_iter().next().ok_or_else(|| "orbitals missing in ScannedWaypoint".to_string())?,
            faction: intermediate_rep.faction.into_iter().next(),
            traits: intermediate_rep.traits.into_iter().next().ok_or_else(|| "traits missing in ScannedWaypoint".to_string())?,
            chart: intermediate_rep.chart.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScannedWaypoint> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScannedWaypoint>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScannedWaypoint>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScannedWaypoint - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScannedWaypoint> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScannedWaypoint as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScannedWaypoint - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SellCargo201Response {
    #[serde(rename = "data")]
    pub data: models::SellCargo201ResponseData,

}

impl SellCargo201Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::SellCargo201ResponseData, ) -> SellCargo201Response {
        SellCargo201Response {
            data,
        }
    }
}

/// Converts the SellCargo201Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SellCargo201Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SellCargo201Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SellCargo201Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::SellCargo201ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SellCargo201Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::SellCargo201ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SellCargo201Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SellCargo201Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in SellCargo201Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SellCargo201Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SellCargo201Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SellCargo201Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SellCargo201Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SellCargo201Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SellCargo201Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SellCargo201Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SellCargo201ResponseData {
    #[serde(rename = "agent")]
    pub agent: models::Agent,

    #[serde(rename = "cargo")]
    pub cargo: models::ShipCargo,

    #[serde(rename = "transaction")]
    pub transaction: models::MarketTransaction,

}

impl SellCargo201ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(agent: models::Agent, cargo: models::ShipCargo, transaction: models::MarketTransaction, ) -> SellCargo201ResponseData {
        SellCargo201ResponseData {
            agent,
            cargo,
            transaction,
        }
    }
}

/// Converts the SellCargo201ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SellCargo201ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping agent in query parameter serialization

            // Skipping cargo in query parameter serialization

            // Skipping transaction in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SellCargo201ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SellCargo201ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub agent: Vec<models::Agent>,
            pub cargo: Vec<models::ShipCargo>,
            pub transaction: Vec<models::MarketTransaction>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SellCargo201ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "agent" => intermediate_rep.agent.push(<models::Agent as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cargo" => intermediate_rep.cargo.push(<models::ShipCargo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "transaction" => intermediate_rep.transaction.push(<models::MarketTransaction as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SellCargo201ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SellCargo201ResponseData {
            agent: intermediate_rep.agent.into_iter().next().ok_or_else(|| "agent missing in SellCargo201ResponseData".to_string())?,
            cargo: intermediate_rep.cargo.into_iter().next().ok_or_else(|| "cargo missing in SellCargo201ResponseData".to_string())?,
            transaction: intermediate_rep.transaction.into_iter().next().ok_or_else(|| "transaction missing in SellCargo201ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SellCargo201ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SellCargo201ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SellCargo201ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SellCargo201ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SellCargo201ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SellCargo201ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SellCargo201ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SellCargoRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "units")]
    pub units: i32,

}

impl SellCargoRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, units: i32, ) -> SellCargoRequest {
        SellCargoRequest {
            symbol,
            units,
        }
    }
}

/// Converts the SellCargoRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SellCargoRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SellCargoRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SellCargoRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub units: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SellCargoRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SellCargoRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SellCargoRequest {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in SellCargoRequest".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in SellCargoRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SellCargoRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SellCargoRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SellCargoRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SellCargoRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SellCargoRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SellCargoRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SellCargoRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A ship
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Ship {
    /// The globally unique identifier of the ship in the following format: `[AGENT_SYMBOL]_[HEX_ID]`
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "registration")]
    pub registration: models::ShipRegistration,

    #[serde(rename = "nav")]
    pub nav: models::ShipNav,

    #[serde(rename = "crew")]
    pub crew: models::ShipCrew,

    #[serde(rename = "frame")]
    pub frame: models::ShipFrame,

    #[serde(rename = "reactor")]
    pub reactor: models::ShipReactor,

    #[serde(rename = "engine")]
    pub engine: models::ShipEngine,

    #[serde(rename = "modules")]
    pub modules: Vec<models::ShipModule>,

    #[serde(rename = "mounts")]
    pub mounts: Vec<models::ShipMount>,

    #[serde(rename = "cargo")]
    pub cargo: models::ShipCargo,

    #[serde(rename = "fuel")]
    pub fuel: models::ShipFuel,

}

impl Ship {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, registration: models::ShipRegistration, nav: models::ShipNav, crew: models::ShipCrew, frame: models::ShipFrame, reactor: models::ShipReactor, engine: models::ShipEngine, modules: Vec<models::ShipModule>, mounts: Vec<models::ShipMount>, cargo: models::ShipCargo, fuel: models::ShipFuel, ) -> Ship {
        Ship {
            symbol,
            registration,
            nav,
            crew,
            frame,
            reactor,
            engine,
            modules,
            mounts,
            cargo,
            fuel,
        }
    }
}

/// Converts the Ship value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Ship {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping registration in query parameter serialization

            // Skipping nav in query parameter serialization

            // Skipping crew in query parameter serialization

            // Skipping frame in query parameter serialization

            // Skipping reactor in query parameter serialization

            // Skipping engine in query parameter serialization

            // Skipping modules in query parameter serialization

            // Skipping mounts in query parameter serialization

            // Skipping cargo in query parameter serialization

            // Skipping fuel in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Ship value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Ship {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub registration: Vec<models::ShipRegistration>,
            pub nav: Vec<models::ShipNav>,
            pub crew: Vec<models::ShipCrew>,
            pub frame: Vec<models::ShipFrame>,
            pub reactor: Vec<models::ShipReactor>,
            pub engine: Vec<models::ShipEngine>,
            pub modules: Vec<Vec<models::ShipModule>>,
            pub mounts: Vec<Vec<models::ShipMount>>,
            pub cargo: Vec<models::ShipCargo>,
            pub fuel: Vec<models::ShipFuel>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Ship".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "registration" => intermediate_rep.registration.push(<models::ShipRegistration as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nav" => intermediate_rep.nav.push(<models::ShipNav as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "crew" => intermediate_rep.crew.push(<models::ShipCrew as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "frame" => intermediate_rep.frame.push(<models::ShipFrame as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reactor" => intermediate_rep.reactor.push(<models::ShipReactor as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "engine" => intermediate_rep.engine.push(<models::ShipEngine as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "modules" => return std::result::Result::Err("Parsing a container in this style is not supported in Ship".to_string()),
                    "mounts" => return std::result::Result::Err("Parsing a container in this style is not supported in Ship".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "cargo" => intermediate_rep.cargo.push(<models::ShipCargo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "fuel" => intermediate_rep.fuel.push(<models::ShipFuel as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Ship".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Ship {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in Ship".to_string())?,
            registration: intermediate_rep.registration.into_iter().next().ok_or_else(|| "registration missing in Ship".to_string())?,
            nav: intermediate_rep.nav.into_iter().next().ok_or_else(|| "nav missing in Ship".to_string())?,
            crew: intermediate_rep.crew.into_iter().next().ok_or_else(|| "crew missing in Ship".to_string())?,
            frame: intermediate_rep.frame.into_iter().next().ok_or_else(|| "frame missing in Ship".to_string())?,
            reactor: intermediate_rep.reactor.into_iter().next().ok_or_else(|| "reactor missing in Ship".to_string())?,
            engine: intermediate_rep.engine.into_iter().next().ok_or_else(|| "engine missing in Ship".to_string())?,
            modules: intermediate_rep.modules.into_iter().next().ok_or_else(|| "modules missing in Ship".to_string())?,
            mounts: intermediate_rep.mounts.into_iter().next().ok_or_else(|| "mounts missing in Ship".to_string())?,
            cargo: intermediate_rep.cargo.into_iter().next().ok_or_else(|| "cargo missing in Ship".to_string())?,
            fuel: intermediate_rep.fuel.into_iter().next().ok_or_else(|| "fuel missing in Ship".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Ship> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Ship>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Ship>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Ship - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Ship> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Ship as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Ship - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipCargo {
    /// The max number of items that can be stored in the cargo hold.
    #[serde(rename = "capacity")]
    pub capacity: u32,

    /// The number of items currently stored in the cargo hold.
    #[serde(rename = "units")]
    pub units: u32,

    /// The items currently in the cargo hold.
    #[serde(rename = "inventory")]
    pub inventory: Vec<models::ShipCargoItem>,

}

impl ShipCargo {
    #[allow(clippy::new_without_default)]
    pub fn new(capacity: u32, units: u32, inventory: Vec<models::ShipCargoItem>, ) -> ShipCargo {
        ShipCargo {
            capacity,
            units,
            inventory,
        }
    }
}

/// Converts the ShipCargo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipCargo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("capacity".to_string()),
            Some(self.capacity.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),

            // Skipping inventory in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipCargo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipCargo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub capacity: Vec<u32>,
            pub units: Vec<u32>,
            pub inventory: Vec<Vec<models::ShipCargoItem>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipCargo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "capacity" => intermediate_rep.capacity.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "inventory" => return std::result::Result::Err("Parsing a container in this style is not supported in ShipCargo".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipCargo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipCargo {
            capacity: intermediate_rep.capacity.into_iter().next().ok_or_else(|| "capacity missing in ShipCargo".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in ShipCargo".to_string())?,
            inventory: intermediate_rep.inventory.into_iter().next().ok_or_else(|| "inventory missing in ShipCargo".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipCargo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipCargo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipCargo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipCargo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipCargo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipCargo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipCargo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The type of cargo item and the number of units.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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
    pub units: u32,

}

impl ShipCargoItem {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, description: String, units: u32, ) -> ShipCargoItem {
        ShipCargoItem {
            symbol,
            name,
            description,
            units,
        }
    }
}

/// Converts the ShipCargoItem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipCargoItem {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipCargoItem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipCargoItem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub units: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipCargoItem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipCargoItem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipCargoItem {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ShipCargoItem".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ShipCargoItem".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in ShipCargoItem".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in ShipCargoItem".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipCargoItem> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipCargoItem>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipCargoItem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipCargoItem - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipCargoItem> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipCargoItem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipCargoItem - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipCondition(i32);

impl std::convert::From<i32> for ShipCondition {
    fn from(x: i32) -> Self {
        ShipCondition(x)
    }
}

impl std::convert::From<ShipCondition> for i32 {
    fn from(x: ShipCondition) -> Self {
        x.0
    }
}

impl std::ops::Deref for ShipCondition {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for ShipCondition {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// The ship's crew service and maintain the ship's systems and equipment.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "rotation")]
    pub rotation: String,

    /// A rough measure of the crew's morale. A higher morale means the crew is happier and more productive. A lower morale means the ship is more prone to accidents.
    #[serde(rename = "morale")]
    pub morale: u8,

    /// The amount of credits per crew member paid per hour. Wages are paid when a ship docks at a civilized waypoint.
    #[serde(rename = "wages")]
    pub wages: u32,

}

impl ShipCrew {
    #[allow(clippy::new_without_default)]
    pub fn new(current: i32, required: i32, capacity: i32, morale: u8, wages: u32, ) -> ShipCrew {
        ShipCrew {
            current,
            required,
            capacity,
            rotation: "STRICT".to_string(),
            morale,
            wages,
        }
    }
}

/// Converts the ShipCrew value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipCrew {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("current".to_string()),
            Some(self.current.to_string()),


            Some("required".to_string()),
            Some(self.required.to_string()),


            Some("capacity".to_string()),
            Some(self.capacity.to_string()),


            Some("rotation".to_string()),
            Some(self.rotation.to_string()),


            Some("morale".to_string()),
            Some(self.morale.to_string()),


            Some("wages".to_string()),
            Some(self.wages.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipCrew value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipCrew {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub current: Vec<i32>,
            pub required: Vec<i32>,
            pub capacity: Vec<i32>,
            pub rotation: Vec<String>,
            pub morale: Vec<u8>,
            pub wages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipCrew".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "current" => intermediate_rep.current.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "required" => intermediate_rep.required.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "capacity" => intermediate_rep.capacity.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "rotation" => intermediate_rep.rotation.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "morale" => intermediate_rep.morale.push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "wages" => intermediate_rep.wages.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipCrew".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipCrew {
            current: intermediate_rep.current.into_iter().next().ok_or_else(|| "current missing in ShipCrew".to_string())?,
            required: intermediate_rep.required.into_iter().next().ok_or_else(|| "required missing in ShipCrew".to_string())?,
            capacity: intermediate_rep.capacity.into_iter().next().ok_or_else(|| "capacity missing in ShipCrew".to_string())?,
            rotation: intermediate_rep.rotation.into_iter().next().ok_or_else(|| "rotation missing in ShipCrew".to_string())?,
            morale: intermediate_rep.morale.into_iter().next().ok_or_else(|| "morale missing in ShipCrew".to_string())?,
            wages: intermediate_rep.wages.into_iter().next().ok_or_else(|| "wages missing in ShipCrew".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipCrew> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipCrew>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipCrew>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipCrew - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipCrew> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipCrew as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipCrew - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The engine determines how quickly a ship travels between waypoints.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipEngine {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub condition: Option<u8>,

    #[serde(rename = "speed")]
    pub speed: f64,

    #[serde(rename = "requirements")]
    pub requirements: models::ShipRequirements,

}

impl ShipEngine {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, description: String, speed: f64, requirements: models::ShipRequirements, ) -> ShipEngine {
        ShipEngine {
            symbol,
            name,
            description,
            condition: None,
            speed,
            requirements,
        }
    }
}

/// Converts the ShipEngine value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipEngine {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),


            self.condition.as_ref().map(|condition| {
                vec![
                    "condition".to_string(),
                    condition.to_string(),
                ].join(",")
            }),


            Some("speed".to_string()),
            Some(self.speed.to_string()),

            // Skipping requirements in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipEngine value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipEngine {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub condition: Vec<u8>,
            pub speed: Vec<f64>,
            pub requirements: Vec<models::ShipRequirements>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipEngine".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "condition" => intermediate_rep.condition.push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "speed" => intermediate_rep.speed.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "requirements" => intermediate_rep.requirements.push(<models::ShipRequirements as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipEngine".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipEngine {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ShipEngine".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ShipEngine".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in ShipEngine".to_string())?,
            condition: intermediate_rep.condition.into_iter().next(),
            speed: intermediate_rep.speed.into_iter().next().ok_or_else(|| "speed missing in ShipEngine".to_string())?,
            requirements: intermediate_rep.requirements.into_iter().next().ok_or_else(|| "requirements missing in ShipEngine".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipEngine> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipEngine>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipEngine>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipEngine - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipEngine> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipEngine as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipEngine - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The frame of the ship. The frame determines the number of modules and mounting points of the ship, as well as base fuel capacity. As the condition of the frame takes more wear, the ship will become more sluggish and less maneuverable.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipFrame {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub condition: Option<u8>,

    #[serde(rename = "moduleSlots")]
    pub module_slots: u32,

    #[serde(rename = "mountingPoints")]
    pub mounting_points: u32,

    #[serde(rename = "fuelCapacity")]
    pub fuel_capacity: u32,

    #[serde(rename = "requirements")]
    pub requirements: models::ShipRequirements,

}

impl ShipFrame {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, description: String, module_slots: u32, mounting_points: u32, fuel_capacity: u32, requirements: models::ShipRequirements, ) -> ShipFrame {
        ShipFrame {
            symbol,
            name,
            description,
            condition: None,
            module_slots,
            mounting_points,
            fuel_capacity,
            requirements,
        }
    }
}

/// Converts the ShipFrame value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipFrame {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),


            self.condition.as_ref().map(|condition| {
                vec![
                    "condition".to_string(),
                    condition.to_string(),
                ].join(",")
            }),


            Some("moduleSlots".to_string()),
            Some(self.module_slots.to_string()),


            Some("mountingPoints".to_string()),
            Some(self.mounting_points.to_string()),


            Some("fuelCapacity".to_string()),
            Some(self.fuel_capacity.to_string()),

            // Skipping requirements in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipFrame value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipFrame {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub condition: Vec<u8>,
            pub module_slots: Vec<u32>,
            pub mounting_points: Vec<u32>,
            pub fuel_capacity: Vec<u32>,
            pub requirements: Vec<models::ShipRequirements>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipFrame".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "condition" => intermediate_rep.condition.push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "moduleSlots" => intermediate_rep.module_slots.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "mountingPoints" => intermediate_rep.mounting_points.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "fuelCapacity" => intermediate_rep.fuel_capacity.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "requirements" => intermediate_rep.requirements.push(<models::ShipRequirements as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipFrame".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipFrame {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ShipFrame".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ShipFrame".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in ShipFrame".to_string())?,
            condition: intermediate_rep.condition.into_iter().next(),
            module_slots: intermediate_rep.module_slots.into_iter().next().ok_or_else(|| "moduleSlots missing in ShipFrame".to_string())?,
            mounting_points: intermediate_rep.mounting_points.into_iter().next().ok_or_else(|| "mountingPoints missing in ShipFrame".to_string())?,
            fuel_capacity: intermediate_rep.fuel_capacity.into_iter().next().ok_or_else(|| "fuelCapacity missing in ShipFrame".to_string())?,
            requirements: intermediate_rep.requirements.into_iter().next().ok_or_else(|| "requirements missing in ShipFrame".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipFrame> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipFrame>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipFrame>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipFrame - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipFrame> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipFrame as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipFrame - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Details of the ship's fuel tanks including how much fuel was consumed during the last transit or action.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipFuel {
    /// The current amount of fuel in the ship's tanks.
    #[serde(rename = "current")]
    pub current: u32,

    /// The maximum amount of fuel the ship's tanks can hold.
    #[serde(rename = "capacity")]
    pub capacity: u32,

    #[serde(rename = "consumed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consumed: Option<models::ShipFuelConsumed>,

}

impl ShipFuel {
    #[allow(clippy::new_without_default)]
    pub fn new(current: u32, capacity: u32, ) -> ShipFuel {
        ShipFuel {
            current,
            capacity,
            consumed: None,
        }
    }
}

/// Converts the ShipFuel value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipFuel {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("current".to_string()),
            Some(self.current.to_string()),


            Some("capacity".to_string()),
            Some(self.capacity.to_string()),

            // Skipping consumed in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipFuel value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipFuel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub current: Vec<u32>,
            pub capacity: Vec<u32>,
            pub consumed: Vec<models::ShipFuelConsumed>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipFuel".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "current" => intermediate_rep.current.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "capacity" => intermediate_rep.capacity.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "consumed" => intermediate_rep.consumed.push(<models::ShipFuelConsumed as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipFuel".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipFuel {
            current: intermediate_rep.current.into_iter().next().ok_or_else(|| "current missing in ShipFuel".to_string())?,
            capacity: intermediate_rep.capacity.into_iter().next().ok_or_else(|| "capacity missing in ShipFuel".to_string())?,
            consumed: intermediate_rep.consumed.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipFuel> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipFuel>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipFuel>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipFuel - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipFuel> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipFuel as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipFuel - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipFuelConsumed {
    /// The amount of fuel consumed by the most recent transit or action.
    #[serde(rename = "amount")]
    pub amount: u32,

    /// The time at which the fuel was consumed.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime::<chrono::Utc>,

}

impl ShipFuelConsumed {
    #[allow(clippy::new_without_default)]
    pub fn new(amount: u32, timestamp: chrono::DateTime::<chrono::Utc>, ) -> ShipFuelConsumed {
        ShipFuelConsumed {
            amount,
            timestamp,
        }
    }
}

/// Converts the ShipFuelConsumed value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipFuelConsumed {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("amount".to_string()),
            Some(self.amount.to_string()),

            // Skipping timestamp in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipFuelConsumed value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipFuelConsumed {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub amount: Vec<u32>,
            pub timestamp: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipFuelConsumed".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "amount" => intermediate_rep.amount.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp" => intermediate_rep.timestamp.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipFuelConsumed".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipFuelConsumed {
            amount: intermediate_rep.amount.into_iter().next().ok_or_else(|| "amount missing in ShipFuelConsumed".to_string())?,
            timestamp: intermediate_rep.timestamp.into_iter().next().ok_or_else(|| "timestamp missing in ShipFuelConsumed".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipFuelConsumed> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipFuelConsumed>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipFuelConsumed>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipFuelConsumed - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipFuelConsumed> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipFuelConsumed as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipFuelConsumed - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A module can be installed in a ship and provides a set of capabilities such as storage space or quarters for crew. Module installations are permanent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipModule {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "capacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub capacity: Option<u32>,

    #[serde(rename = "range")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub range: Option<u32>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "requirements")]
    pub requirements: models::ShipRequirements,

}

impl ShipModule {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, requirements: models::ShipRequirements, ) -> ShipModule {
        ShipModule {
            symbol,
            capacity: None,
            range: None,
            name,
            description: None,
            requirements,
        }
    }
}

/// Converts the ShipModule value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipModule {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            self.capacity.as_ref().map(|capacity| {
                vec![
                    "capacity".to_string(),
                    capacity.to_string(),
                ].join(",")
            }),


            self.range.as_ref().map(|range| {
                vec![
                    "range".to_string(),
                    range.to_string(),
                ].join(",")
            }),


            Some("name".to_string()),
            Some(self.name.to_string()),


            self.description.as_ref().map(|description| {
                vec![
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),

            // Skipping requirements in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipModule value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipModule {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub capacity: Vec<u32>,
            pub range: Vec<u32>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub requirements: Vec<models::ShipRequirements>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipModule".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "capacity" => intermediate_rep.capacity.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "range" => intermediate_rep.range.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "requirements" => intermediate_rep.requirements.push(<models::ShipRequirements as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipModule".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipModule {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ShipModule".to_string())?,
            capacity: intermediate_rep.capacity.into_iter().next(),
            range: intermediate_rep.range.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ShipModule".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            requirements: intermediate_rep.requirements.into_iter().next().ok_or_else(|| "requirements missing in ShipModule".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipModule> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipModule>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipModule>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipModule - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipModule> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipModule as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipModule - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A mount is installed on the exterier of a ship.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipMount {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "strength")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub strength: Option<u32>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "deposits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deposits: Option<Vec<String>>,

    #[serde(rename = "requirements")]
    pub requirements: models::ShipRequirements,

}

impl ShipMount {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, requirements: models::ShipRequirements, ) -> ShipMount {
        ShipMount {
            symbol,
            name,
            description: None,
            strength: None,
            deposits: None,
            requirements,
        }
    }
}

/// Converts the ShipMount value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipMount {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            self.description.as_ref().map(|description| {
                vec![
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),


            self.strength.as_ref().map(|strength| {
                vec![
                    "strength".to_string(),
                    strength.to_string(),
                ].join(",")
            }),


            self.deposits.as_ref().map(|deposits| {
                vec![
                    "deposits".to_string(),
                    deposits.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping requirements in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipMount value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipMount {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub strength: Vec<u32>,
            pub deposits: Vec<Vec<String>>,
            pub requirements: Vec<models::ShipRequirements>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipMount".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "strength" => intermediate_rep.strength.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "deposits" => return std::result::Result::Err("Parsing a container in this style is not supported in ShipMount".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "requirements" => intermediate_rep.requirements.push(<models::ShipRequirements as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipMount".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipMount {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ShipMount".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ShipMount".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            strength: intermediate_rep.strength.into_iter().next(),
            deposits: intermediate_rep.deposits.into_iter().next(),
            requirements: intermediate_rep.requirements.into_iter().next().ok_or_else(|| "requirements missing in ShipMount".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipMount> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipMount>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipMount>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipMount - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipMount> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipMount as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipMount - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The navigation information of the ship.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipNav {
    /// The system symbol of the ship's current location.
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,

    /// The waypoint symbol of the ship's current location, or if the ship is in-transit, the waypoint symbol of the ship's destination.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,

    #[serde(rename = "route")]
    pub route: models::ShipNavRoute,

    #[serde(rename = "status")]
    pub status: models::ShipNavStatus,

    #[serde(rename = "flightMode")]
    pub flight_mode: models::ShipNavFlightMode,

}

impl ShipNav {
    #[allow(clippy::new_without_default)]
    pub fn new(system_symbol: String, waypoint_symbol: String, route: models::ShipNavRoute, status: models::ShipNavStatus, flight_mode: models::ShipNavFlightMode, ) -> ShipNav {
        ShipNav {
            system_symbol,
            waypoint_symbol,
            route,
            status,
            flight_mode,
        }
    }
}

/// Converts the ShipNav value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipNav {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("systemSymbol".to_string()),
            Some(self.system_symbol.to_string()),


            Some("waypointSymbol".to_string()),
            Some(self.waypoint_symbol.to_string()),

            // Skipping route in query parameter serialization

            // Skipping status in query parameter serialization

            // Skipping flightMode in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipNav value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipNav {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub system_symbol: Vec<String>,
            pub waypoint_symbol: Vec<String>,
            pub route: Vec<models::ShipNavRoute>,
            pub status: Vec<models::ShipNavStatus>,
            pub flight_mode: Vec<models::ShipNavFlightMode>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipNav".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "systemSymbol" => intermediate_rep.system_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "waypointSymbol" => intermediate_rep.waypoint_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "route" => intermediate_rep.route.push(<models::ShipNavRoute as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::ShipNavStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "flightMode" => intermediate_rep.flight_mode.push(<models::ShipNavFlightMode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipNav".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipNav {
            system_symbol: intermediate_rep.system_symbol.into_iter().next().ok_or_else(|| "systemSymbol missing in ShipNav".to_string())?,
            waypoint_symbol: intermediate_rep.waypoint_symbol.into_iter().next().ok_or_else(|| "waypointSymbol missing in ShipNav".to_string())?,
            route: intermediate_rep.route.into_iter().next().ok_or_else(|| "route missing in ShipNav".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in ShipNav".to_string())?,
            flight_mode: intermediate_rep.flight_mode.into_iter().next().ok_or_else(|| "flightMode missing in ShipNav".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipNav> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipNav>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipNav>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipNav - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipNav> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipNav as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipNav - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The ship's set speed when traveling between waypoints or systems.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ShipNavFlightMode {
    #[serde(rename = "DRIFT")]
    Drift,
    #[serde(rename = "STEALTH")]
    Stealth,
    #[serde(rename = "CRUISE")]
    Cruise,
    #[serde(rename = "BURN")]
    Burn,
}

impl std::fmt::Display for ShipNavFlightMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShipNavFlightMode::Drift => write!(f, "DRIFT"),
            ShipNavFlightMode::Stealth => write!(f, "STEALTH"),
            ShipNavFlightMode::Cruise => write!(f, "CRUISE"),
            ShipNavFlightMode::Burn => write!(f, "BURN"),
        }
    }
}

impl std::str::FromStr for ShipNavFlightMode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "DRIFT" => std::result::Result::Ok(ShipNavFlightMode::Drift),
            "STEALTH" => std::result::Result::Ok(ShipNavFlightMode::Stealth),
            "CRUISE" => std::result::Result::Ok(ShipNavFlightMode::Cruise),
            "BURN" => std::result::Result::Ok(ShipNavFlightMode::Burn),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// The routing information for the ship's most recent transit or current location.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipNavRoute {
    #[serde(rename = "destination")]
    pub destination: models::ShipNavRouteWaypoint,

    #[serde(rename = "departure")]
    pub departure: models::ShipNavRouteWaypoint,

    /// The date time of the ship's departure.
    #[serde(rename = "departureTime")]
    pub departure_time: chrono::DateTime::<chrono::Utc>,

    /// The date time of the ship's arrival. If the ship is in-transit, this is the expected time of arrival.
    #[serde(rename = "arrival")]
    pub arrival: chrono::DateTime::<chrono::Utc>,

}

impl ShipNavRoute {
    #[allow(clippy::new_without_default)]
    pub fn new(destination: models::ShipNavRouteWaypoint, departure: models::ShipNavRouteWaypoint, departure_time: chrono::DateTime::<chrono::Utc>, arrival: chrono::DateTime::<chrono::Utc>, ) -> ShipNavRoute {
        ShipNavRoute {
            destination,
            departure,
            departure_time,
            arrival,
        }
    }
}

/// Converts the ShipNavRoute value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipNavRoute {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping destination in query parameter serialization

            // Skipping departure in query parameter serialization

            // Skipping departureTime in query parameter serialization

            // Skipping arrival in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipNavRoute value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipNavRoute {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub destination: Vec<models::ShipNavRouteWaypoint>,
            pub departure: Vec<models::ShipNavRouteWaypoint>,
            pub departure_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub arrival: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipNavRoute".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "destination" => intermediate_rep.destination.push(<models::ShipNavRouteWaypoint as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "departure" => intermediate_rep.departure.push(<models::ShipNavRouteWaypoint as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "departureTime" => intermediate_rep.departure_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "arrival" => intermediate_rep.arrival.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipNavRoute".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipNavRoute {
            destination: intermediate_rep.destination.into_iter().next().ok_or_else(|| "destination missing in ShipNavRoute".to_string())?,
            departure: intermediate_rep.departure.into_iter().next().ok_or_else(|| "departure missing in ShipNavRoute".to_string())?,
            departure_time: intermediate_rep.departure_time.into_iter().next().ok_or_else(|| "departureTime missing in ShipNavRoute".to_string())?,
            arrival: intermediate_rep.arrival.into_iter().next().ok_or_else(|| "arrival missing in ShipNavRoute".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipNavRoute> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipNavRoute>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipNavRoute>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipNavRoute - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipNavRoute> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipNavRoute as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipNavRoute - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The destination or departure of a ships nav route.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipNavRouteWaypoint {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "type")]
    pub r#type: models::WaypointType,

    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,

    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

}

impl ShipNavRouteWaypoint {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, r#type: models::WaypointType, system_symbol: String, x: i32, y: i32, ) -> ShipNavRouteWaypoint {
        ShipNavRouteWaypoint {
            symbol,
            r#type,
            system_symbol,
            x,
            y,
        }
    }
}

/// Converts the ShipNavRouteWaypoint value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipNavRouteWaypoint {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping type in query parameter serialization


            Some("systemSymbol".to_string()),
            Some(self.system_symbol.to_string()),


            Some("x".to_string()),
            Some(self.x.to_string()),


            Some("y".to_string()),
            Some(self.y.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipNavRouteWaypoint value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipNavRouteWaypoint {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub r#type: Vec<models::WaypointType>,
            pub system_symbol: Vec<String>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipNavRouteWaypoint".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::WaypointType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "systemSymbol" => intermediate_rep.system_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipNavRouteWaypoint".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipNavRouteWaypoint {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ShipNavRouteWaypoint".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ShipNavRouteWaypoint".to_string())?,
            system_symbol: intermediate_rep.system_symbol.into_iter().next().ok_or_else(|| "systemSymbol missing in ShipNavRouteWaypoint".to_string())?,
            x: intermediate_rep.x.into_iter().next().ok_or_else(|| "x missing in ShipNavRouteWaypoint".to_string())?,
            y: intermediate_rep.y.into_iter().next().ok_or_else(|| "y missing in ShipNavRouteWaypoint".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipNavRouteWaypoint> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipNavRouteWaypoint>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipNavRouteWaypoint>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipNavRouteWaypoint - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipNavRouteWaypoint> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipNavRouteWaypoint as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipNavRouteWaypoint - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The current status of the ship
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ShipNavStatus {
    #[serde(rename = "IN_TRANSIT")]
    InTransit,
    #[serde(rename = "IN_ORBIT")]
    InOrbit,
    #[serde(rename = "DOCKED")]
    Docked,
}

impl std::fmt::Display for ShipNavStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShipNavStatus::InTransit => write!(f, "IN_TRANSIT"),
            ShipNavStatus::InOrbit => write!(f, "IN_ORBIT"),
            ShipNavStatus::Docked => write!(f, "DOCKED"),
        }
    }
}

impl std::str::FromStr for ShipNavStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "IN_TRANSIT" => std::result::Result::Ok(ShipNavStatus::InTransit),
            "IN_ORBIT" => std::result::Result::Ok(ShipNavStatus::InOrbit),
            "DOCKED" => std::result::Result::Ok(ShipNavStatus::Docked),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// The reactor of the ship. The reactor is responsible for powering the ship's systems and weapons.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipReactor {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    /// Condition is a range of 0 to 100 where 0 is completely worn out and 100 is brand new.
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub condition: Option<u8>,

    #[serde(rename = "powerOutput")]
    pub power_output: u32,

    #[serde(rename = "requirements")]
    pub requirements: models::ShipRequirements,

}

impl ShipReactor {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, description: String, power_output: u32, requirements: models::ShipRequirements, ) -> ShipReactor {
        ShipReactor {
            symbol,
            name,
            description,
            condition: None,
            power_output,
            requirements,
        }
    }
}

/// Converts the ShipReactor value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipReactor {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),


            self.condition.as_ref().map(|condition| {
                vec![
                    "condition".to_string(),
                    condition.to_string(),
                ].join(",")
            }),


            Some("powerOutput".to_string()),
            Some(self.power_output.to_string()),

            // Skipping requirements in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipReactor value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipReactor {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub condition: Vec<u8>,
            pub power_output: Vec<u32>,
            pub requirements: Vec<models::ShipRequirements>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipReactor".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "condition" => intermediate_rep.condition.push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "powerOutput" => intermediate_rep.power_output.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "requirements" => intermediate_rep.requirements.push(<models::ShipRequirements as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipReactor".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipReactor {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in ShipReactor".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ShipReactor".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in ShipReactor".to_string())?,
            condition: intermediate_rep.condition.into_iter().next(),
            power_output: intermediate_rep.power_output.into_iter().next().ok_or_else(|| "powerOutput missing in ShipReactor".to_string())?,
            requirements: intermediate_rep.requirements.into_iter().next().ok_or_else(|| "requirements missing in ShipReactor".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipReactor> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipReactor>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipReactor>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipReactor - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipReactor> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipReactor as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipReactor - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipRefine200Response {
    #[serde(rename = "data")]
    pub data: models::ShipRefine200ResponseData,

}

impl ShipRefine200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::ShipRefine200ResponseData, ) -> ShipRefine200Response {
        ShipRefine200Response {
            data,
        }
    }
}

/// Converts the ShipRefine200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipRefine200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipRefine200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipRefine200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::ShipRefine200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipRefine200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::ShipRefine200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipRefine200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipRefine200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in ShipRefine200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipRefine200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipRefine200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipRefine200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipRefine200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipRefine200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipRefine200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipRefine200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipRefine200ResponseData {
    #[serde(rename = "cargo")]
    pub cargo: models::ShipCargo,

    #[serde(rename = "cooldown")]
    pub cooldown: models::Cooldown,

    #[serde(rename = "produced")]
    pub produced: Vec<models::ShipRefine200ResponseDataProducedInner>,

    #[serde(rename = "consumed")]
    pub consumed: Vec<models::ShipRefine200ResponseDataProducedInner>,

}

impl ShipRefine200ResponseData {
    #[allow(clippy::new_without_default)]
    pub fn new(cargo: models::ShipCargo, cooldown: models::Cooldown, produced: Vec<models::ShipRefine200ResponseDataProducedInner>, consumed: Vec<models::ShipRefine200ResponseDataProducedInner>, ) -> ShipRefine200ResponseData {
        ShipRefine200ResponseData {
            cargo,
            cooldown,
            produced,
            consumed,
        }
    }
}

/// Converts the ShipRefine200ResponseData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipRefine200ResponseData {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cargo in query parameter serialization

            // Skipping cooldown in query parameter serialization

            // Skipping produced in query parameter serialization

            // Skipping consumed in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipRefine200ResponseData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipRefine200ResponseData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cargo: Vec<models::ShipCargo>,
            pub cooldown: Vec<models::Cooldown>,
            pub produced: Vec<Vec<models::ShipRefine200ResponseDataProducedInner>>,
            pub consumed: Vec<Vec<models::ShipRefine200ResponseDataProducedInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipRefine200ResponseData".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cargo" => intermediate_rep.cargo.push(<models::ShipCargo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(<models::Cooldown as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "produced" => return std::result::Result::Err("Parsing a container in this style is not supported in ShipRefine200ResponseData".to_string()),
                    "consumed" => return std::result::Result::Err("Parsing a container in this style is not supported in ShipRefine200ResponseData".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipRefine200ResponseData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipRefine200ResponseData {
            cargo: intermediate_rep.cargo.into_iter().next().ok_or_else(|| "cargo missing in ShipRefine200ResponseData".to_string())?,
            cooldown: intermediate_rep.cooldown.into_iter().next().ok_or_else(|| "cooldown missing in ShipRefine200ResponseData".to_string())?,
            produced: intermediate_rep.produced.into_iter().next().ok_or_else(|| "produced missing in ShipRefine200ResponseData".to_string())?,
            consumed: intermediate_rep.consumed.into_iter().next().ok_or_else(|| "consumed missing in ShipRefine200ResponseData".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipRefine200ResponseData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipRefine200ResponseData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipRefine200ResponseData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipRefine200ResponseData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipRefine200ResponseData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipRefine200ResponseData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipRefine200ResponseData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipRefine200ResponseDataProducedInner {
    #[serde(rename = "tradeSymbol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trade_symbol: Option<String>,

    #[serde(rename = "units")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub units: Option<i32>,

}

impl ShipRefine200ResponseDataProducedInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ShipRefine200ResponseDataProducedInner {
        ShipRefine200ResponseDataProducedInner {
            trade_symbol: None,
            units: None,
        }
    }
}

/// Converts the ShipRefine200ResponseDataProducedInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipRefine200ResponseDataProducedInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.trade_symbol.as_ref().map(|trade_symbol| {
                vec![
                    "tradeSymbol".to_string(),
                    trade_symbol.to_string(),
                ].join(",")
            }),


            self.units.as_ref().map(|units| {
                vec![
                    "units".to_string(),
                    units.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipRefine200ResponseDataProducedInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipRefine200ResponseDataProducedInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trade_symbol: Vec<String>,
            pub units: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipRefine200ResponseDataProducedInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tradeSymbol" => intermediate_rep.trade_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipRefine200ResponseDataProducedInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipRefine200ResponseDataProducedInner {
            trade_symbol: intermediate_rep.trade_symbol.into_iter().next(),
            units: intermediate_rep.units.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipRefine200ResponseDataProducedInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipRefine200ResponseDataProducedInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipRefine200ResponseDataProducedInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipRefine200ResponseDataProducedInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipRefine200ResponseDataProducedInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipRefine200ResponseDataProducedInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipRefine200ResponseDataProducedInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipRefineRequest {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "produce")]
    pub produce: String,

}

impl ShipRefineRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(produce: String, ) -> ShipRefineRequest {
        ShipRefineRequest {
            produce,
        }
    }
}

/// Converts the ShipRefineRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipRefineRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("produce".to_string()),
            Some(self.produce.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipRefineRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipRefineRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub produce: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipRefineRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "produce" => intermediate_rep.produce.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipRefineRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipRefineRequest {
            produce: intermediate_rep.produce.into_iter().next().ok_or_else(|| "produce missing in ShipRefineRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipRefineRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipRefineRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipRefineRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipRefineRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipRefineRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipRefineRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipRefineRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The public registration information of the ship
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipRegistration {
    /// The agent's registered name of the ship
    #[serde(rename = "name")]
    pub name: String,

    /// The symbol of the faction the ship is registered with
    #[serde(rename = "factionSymbol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub faction_symbol: Option<String>,

    #[serde(rename = "role")]
    pub role: models::ShipRole,

}

impl ShipRegistration {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, role: models::ShipRole, ) -> ShipRegistration {
        ShipRegistration {
            name,
            faction_symbol: None,
            role,
        }
    }
}

/// Converts the ShipRegistration value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipRegistration {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),


            self.faction_symbol.as_ref().map(|faction_symbol| {
                vec![
                    "factionSymbol".to_string(),
                    faction_symbol.to_string(),
                ].join(",")
            }),

            // Skipping role in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipRegistration value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipRegistration {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub faction_symbol: Vec<String>,
            pub role: Vec<models::ShipRole>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipRegistration".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "factionSymbol" => intermediate_rep.faction_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "role" => intermediate_rep.role.push(<models::ShipRole as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipRegistration".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipRegistration {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ShipRegistration".to_string())?,
            faction_symbol: intermediate_rep.faction_symbol.into_iter().next(),
            role: intermediate_rep.role.into_iter().next().ok_or_else(|| "role missing in ShipRegistration".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipRegistration> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipRegistration>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipRegistration>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipRegistration - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipRegistration> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipRegistration as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipRegistration - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The requirements for installation on a ship
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipRequirements {
    /// The amount of power required from the reactor.
    #[serde(rename = "power")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub power: Option<i32>,

    /// The number of crew required for operation.
    #[serde(rename = "crew")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub crew: Option<i32>,

    /// The number of module slots required for installation.
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slots: Option<i32>,

}

impl ShipRequirements {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ShipRequirements {
        ShipRequirements {
            power: None,
            crew: None,
            slots: None,
        }
    }
}

/// Converts the ShipRequirements value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipRequirements {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.power.as_ref().map(|power| {
                vec![
                    "power".to_string(),
                    power.to_string(),
                ].join(",")
            }),


            self.crew.as_ref().map(|crew| {
                vec![
                    "crew".to_string(),
                    crew.to_string(),
                ].join(",")
            }),


            self.slots.as_ref().map(|slots| {
                vec![
                    "slots".to_string(),
                    slots.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipRequirements value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipRequirements {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub power: Vec<i32>,
            pub crew: Vec<i32>,
            pub slots: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipRequirements".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "power" => intermediate_rep.power.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "crew" => intermediate_rep.crew.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "slots" => intermediate_rep.slots.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipRequirements".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipRequirements {
            power: intermediate_rep.power.into_iter().next(),
            crew: intermediate_rep.crew.into_iter().next(),
            slots: intermediate_rep.slots.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipRequirements> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipRequirements>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipRequirements>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipRequirements - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipRequirements> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipRequirements as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipRequirements - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The registered role of the ship
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ShipRole {
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

impl std::fmt::Display for ShipRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShipRole::Fabricator => write!(f, "FABRICATOR"),
            ShipRole::Harvester => write!(f, "HARVESTER"),
            ShipRole::Hauler => write!(f, "HAULER"),
            ShipRole::Interceptor => write!(f, "INTERCEPTOR"),
            ShipRole::Excavator => write!(f, "EXCAVATOR"),
            ShipRole::Transport => write!(f, "TRANSPORT"),
            ShipRole::Repair => write!(f, "REPAIR"),
            ShipRole::Surveyor => write!(f, "SURVEYOR"),
            ShipRole::Command => write!(f, "COMMAND"),
            ShipRole::Carrier => write!(f, "CARRIER"),
            ShipRole::Patrol => write!(f, "PATROL"),
            ShipRole::Satellite => write!(f, "SATELLITE"),
            ShipRole::Explorer => write!(f, "EXPLORER"),
            ShipRole::Refinery => write!(f, "REFINERY"),
        }
    }
}

impl std::str::FromStr for ShipRole {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "FABRICATOR" => std::result::Result::Ok(ShipRole::Fabricator),
            "HARVESTER" => std::result::Result::Ok(ShipRole::Harvester),
            "HAULER" => std::result::Result::Ok(ShipRole::Hauler),
            "INTERCEPTOR" => std::result::Result::Ok(ShipRole::Interceptor),
            "EXCAVATOR" => std::result::Result::Ok(ShipRole::Excavator),
            "TRANSPORT" => std::result::Result::Ok(ShipRole::Transport),
            "REPAIR" => std::result::Result::Ok(ShipRole::Repair),
            "SURVEYOR" => std::result::Result::Ok(ShipRole::Surveyor),
            "COMMAND" => std::result::Result::Ok(ShipRole::Command),
            "CARRIER" => std::result::Result::Ok(ShipRole::Carrier),
            "PATROL" => std::result::Result::Ok(ShipRole::Patrol),
            "SATELLITE" => std::result::Result::Ok(ShipRole::Satellite),
            "EXPLORER" => std::result::Result::Ok(ShipRole::Explorer),
            "REFINERY" => std::result::Result::Ok(ShipRole::Refinery),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// 
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ShipType {
    #[serde(rename = "SHIP_PROBE")]
    Probe,
    #[serde(rename = "SHIP_MINING_DRONE")]
    MiningDrone,
    #[serde(rename = "SHIP_INTERCEPTOR")]
    Interceptor,
    #[serde(rename = "SHIP_LIGHT_HAULER")]
    LightHauler,
    #[serde(rename = "SHIP_COMMAND_FRIGATE")]
    CommandFrigate,
    #[serde(rename = "SHIP_EXPLORER")]
    Explorer,
    #[serde(rename = "SHIP_HEAVY_FREIGHTER")]
    HeavyFreighter,
    #[serde(rename = "SHIP_LIGHT_SHUTTLE")]
    LightShuttle,
    #[serde(rename = "SHIP_ORE_HOUND")]
    OreHound,
    #[serde(rename = "SHIP_REFINING_FREIGHTER")]
    RefiningFreighter,
}

impl std::fmt::Display for ShipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShipType::Probe => write!(f, "SHIP_PROBE"),
            ShipType::MiningDrone => write!(f, "SHIP_MINING_DRONE"),
            ShipType::Interceptor => write!(f, "SHIP_INTERCEPTOR"),
            ShipType::LightHauler => write!(f, "SHIP_LIGHT_HAULER"),
            ShipType::CommandFrigate => write!(f, "SHIP_COMMAND_FRIGATE"),
            ShipType::Explorer => write!(f, "SHIP_EXPLORER"),
            ShipType::HeavyFreighter => write!(f, "SHIP_HEAVY_FREIGHTER"),
            ShipType::LightShuttle => write!(f, "SHIP_LIGHT_SHUTTLE"),
            ShipType::OreHound => write!(f, "SHIP_ORE_HOUND"),
            ShipType::RefiningFreighter => write!(f, "SHIP_REFINING_FREIGHTER"),
        }
    }
}

impl std::str::FromStr for ShipType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "SHIP_PROBE" => std::result::Result::Ok(ShipType::Probe),
            "SHIP_MINING_DRONE" => std::result::Result::Ok(ShipType::MiningDrone),
            "SHIP_INTERCEPTOR" => std::result::Result::Ok(ShipType::Interceptor),
            "SHIP_LIGHT_HAULER" => std::result::Result::Ok(ShipType::LightHauler),
            "SHIP_COMMAND_FRIGATE" => std::result::Result::Ok(ShipType::CommandFrigate),
            "SHIP_EXPLORER" => std::result::Result::Ok(ShipType::Explorer),
            "SHIP_HEAVY_FREIGHTER" => std::result::Result::Ok(ShipType::HeavyFreighter),
            "SHIP_LIGHT_SHUTTLE" => std::result::Result::Ok(ShipType::LightShuttle),
            "SHIP_ORE_HOUND" => std::result::Result::Ok(ShipType::OreHound),
            "SHIP_REFINING_FREIGHTER" => std::result::Result::Ok(ShipType::RefiningFreighter),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Shipyard {
    /// The symbol of the shipyard. The symbol is the same as the waypoint where the shipyard is located.
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// The list of ship types available for purchase at this shipyard.
    #[serde(rename = "shipTypes")]
    pub ship_types: Vec<models::ShipyardShipTypesInner>,

    /// The list of recent transactions at this shipyard.
    #[serde(rename = "transactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transactions: Option<Vec<models::ShipyardTransaction>>,

    /// The ships that are currently available for purchase at the shipyard.
    #[serde(rename = "ships")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ships: Option<Vec<models::ShipyardShip>>,

}

impl Shipyard {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ship_types: Vec<models::ShipyardShipTypesInner>, ) -> Shipyard {
        Shipyard {
            symbol,
            ship_types,
            transactions: None,
            ships: None,
        }
    }
}

/// Converts the Shipyard value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Shipyard {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping shipTypes in query parameter serialization

            // Skipping transactions in query parameter serialization

            // Skipping ships in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Shipyard value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Shipyard {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub ship_types: Vec<Vec<models::ShipyardShipTypesInner>>,
            pub transactions: Vec<Vec<models::ShipyardTransaction>>,
            pub ships: Vec<Vec<models::ShipyardShip>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Shipyard".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "shipTypes" => return std::result::Result::Err("Parsing a container in this style is not supported in Shipyard".to_string()),
                    "transactions" => return std::result::Result::Err("Parsing a container in this style is not supported in Shipyard".to_string()),
                    "ships" => return std::result::Result::Err("Parsing a container in this style is not supported in Shipyard".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Shipyard".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Shipyard {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in Shipyard".to_string())?,
            ship_types: intermediate_rep.ship_types.into_iter().next().ok_or_else(|| "shipTypes missing in Shipyard".to_string())?,
            transactions: intermediate_rep.transactions.into_iter().next(),
            ships: intermediate_rep.ships.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Shipyard> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Shipyard>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Shipyard>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Shipyard - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Shipyard> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Shipyard as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Shipyard - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipyardShip {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<models::ShipType>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "purchasePrice")]
    pub purchase_price: i32,

    #[serde(rename = "frame")]
    pub frame: models::ShipFrame,

    #[serde(rename = "reactor")]
    pub reactor: models::ShipReactor,

    #[serde(rename = "engine")]
    pub engine: models::ShipEngine,

    #[serde(rename = "modules")]
    pub modules: Vec<models::ShipModule>,

    #[serde(rename = "mounts")]
    pub mounts: Vec<models::ShipMount>,

}

impl ShipyardShip {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, description: String, purchase_price: i32, frame: models::ShipFrame, reactor: models::ShipReactor, engine: models::ShipEngine, modules: Vec<models::ShipModule>, mounts: Vec<models::ShipMount>, ) -> ShipyardShip {
        ShipyardShip {
            r#type: None,
            name,
            description,
            purchase_price,
            frame,
            reactor,
            engine,
            modules,
            mounts,
        }
    }
}

/// Converts the ShipyardShip value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipyardShip {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping type in query parameter serialization


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),


            Some("purchasePrice".to_string()),
            Some(self.purchase_price.to_string()),

            // Skipping frame in query parameter serialization

            // Skipping reactor in query parameter serialization

            // Skipping engine in query parameter serialization

            // Skipping modules in query parameter serialization

            // Skipping mounts in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipyardShip value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipyardShip {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<models::ShipType>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub purchase_price: Vec<i32>,
            pub frame: Vec<models::ShipFrame>,
            pub reactor: Vec<models::ShipReactor>,
            pub engine: Vec<models::ShipEngine>,
            pub modules: Vec<Vec<models::ShipModule>>,
            pub mounts: Vec<Vec<models::ShipMount>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipyardShip".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::ShipType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "purchasePrice" => intermediate_rep.purchase_price.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "frame" => intermediate_rep.frame.push(<models::ShipFrame as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reactor" => intermediate_rep.reactor.push(<models::ShipReactor as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "engine" => intermediate_rep.engine.push(<models::ShipEngine as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "modules" => return std::result::Result::Err("Parsing a container in this style is not supported in ShipyardShip".to_string()),
                    "mounts" => return std::result::Result::Err("Parsing a container in this style is not supported in ShipyardShip".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipyardShip".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipyardShip {
            r#type: intermediate_rep.r#type.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ShipyardShip".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in ShipyardShip".to_string())?,
            purchase_price: intermediate_rep.purchase_price.into_iter().next().ok_or_else(|| "purchasePrice missing in ShipyardShip".to_string())?,
            frame: intermediate_rep.frame.into_iter().next().ok_or_else(|| "frame missing in ShipyardShip".to_string())?,
            reactor: intermediate_rep.reactor.into_iter().next().ok_or_else(|| "reactor missing in ShipyardShip".to_string())?,
            engine: intermediate_rep.engine.into_iter().next().ok_or_else(|| "engine missing in ShipyardShip".to_string())?,
            modules: intermediate_rep.modules.into_iter().next().ok_or_else(|| "modules missing in ShipyardShip".to_string())?,
            mounts: intermediate_rep.mounts.into_iter().next().ok_or_else(|| "mounts missing in ShipyardShip".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipyardShip> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipyardShip>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipyardShip>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipyardShip - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipyardShip> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipyardShip as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipyardShip - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipyardShipTypesInner {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<models::ShipType>,

}

impl ShipyardShipTypesInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ShipyardShipTypesInner {
        ShipyardShipTypesInner {
            r#type: None,
        }
    }
}

/// Converts the ShipyardShipTypesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipyardShipTypesInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping type in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipyardShipTypesInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipyardShipTypesInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<models::ShipType>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipyardShipTypesInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::ShipType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipyardShipTypesInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipyardShipTypesInner {
            r#type: intermediate_rep.r#type.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipyardShipTypesInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipyardShipTypesInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipyardShipTypesInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipyardShipTypesInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipyardShipTypesInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipyardShipTypesInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipyardShipTypesInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShipyardTransaction {
    /// The symbol of the waypoint where the transaction took place.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,

    /// The symbol of the ship that was purchased.
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,

    /// The price of the transaction.
    #[serde(rename = "price")]
    pub price: u32,

    /// The symbol of the agent that made the transaction.
    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,

    /// The timestamp of the transaction.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime::<chrono::Utc>,

}

impl ShipyardTransaction {
    #[allow(clippy::new_without_default)]
    pub fn new(waypoint_symbol: String, ship_symbol: String, price: u32, agent_symbol: String, timestamp: chrono::DateTime::<chrono::Utc>, ) -> ShipyardTransaction {
        ShipyardTransaction {
            waypoint_symbol,
            ship_symbol,
            price,
            agent_symbol,
            timestamp,
        }
    }
}

/// Converts the ShipyardTransaction value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShipyardTransaction {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("waypointSymbol".to_string()),
            Some(self.waypoint_symbol.to_string()),


            Some("shipSymbol".to_string()),
            Some(self.ship_symbol.to_string()),


            Some("price".to_string()),
            Some(self.price.to_string()),


            Some("agentSymbol".to_string()),
            Some(self.agent_symbol.to_string()),

            // Skipping timestamp in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShipyardTransaction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShipyardTransaction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub waypoint_symbol: Vec<String>,
            pub ship_symbol: Vec<String>,
            pub price: Vec<u32>,
            pub agent_symbol: Vec<String>,
            pub timestamp: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShipyardTransaction".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "waypointSymbol" => intermediate_rep.waypoint_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "shipSymbol" => intermediate_rep.ship_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "price" => intermediate_rep.price.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "agentSymbol" => intermediate_rep.agent_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp" => intermediate_rep.timestamp.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShipyardTransaction".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShipyardTransaction {
            waypoint_symbol: intermediate_rep.waypoint_symbol.into_iter().next().ok_or_else(|| "waypointSymbol missing in ShipyardTransaction".to_string())?,
            ship_symbol: intermediate_rep.ship_symbol.into_iter().next().ok_or_else(|| "shipSymbol missing in ShipyardTransaction".to_string())?,
            price: intermediate_rep.price.into_iter().next().ok_or_else(|| "price missing in ShipyardTransaction".to_string())?,
            agent_symbol: intermediate_rep.agent_symbol.into_iter().next().ok_or_else(|| "agentSymbol missing in ShipyardTransaction".to_string())?,
            timestamp: intermediate_rep.timestamp.into_iter().next().ok_or_else(|| "timestamp missing in ShipyardTransaction".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShipyardTransaction> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShipyardTransaction>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShipyardTransaction>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShipyardTransaction - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShipyardTransaction> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShipyardTransaction as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShipyardTransaction - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A resource survey of a waypoint, detailing a specific extraction location and the types of resources that can be found there.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Survey {
    /// A unique signature for the location of this survey. This signature is verified when attempting an extraction using this survey.
    #[serde(rename = "signature")]
    pub signature: String,

    /// The symbol of the waypoint that this survey is for.
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// A list of deposits that can be found at this location.
    #[serde(rename = "deposits")]
    pub deposits: Vec<models::SurveyDeposit>,

    /// The date and time when the survey expires. After this date and time, the survey will no longer be available for extraction.
    #[serde(rename = "expiration")]
    pub expiration: chrono::DateTime::<chrono::Utc>,

    /// The size of the deposit. This value indicates how much can be extracted from the survey before it is exhausted.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "size")]
    pub size: String,

}

impl Survey {
    #[allow(clippy::new_without_default)]
    pub fn new(signature: String, symbol: String, deposits: Vec<models::SurveyDeposit>, expiration: chrono::DateTime::<chrono::Utc>, size: String, ) -> Survey {
        Survey {
            signature,
            symbol,
            deposits,
            expiration,
            size,
        }
    }
}

/// Converts the Survey value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Survey {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("signature".to_string()),
            Some(self.signature.to_string()),


            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping deposits in query parameter serialization

            // Skipping expiration in query parameter serialization


            Some("size".to_string()),
            Some(self.size.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Survey value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Survey {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub signature: Vec<String>,
            pub symbol: Vec<String>,
            pub deposits: Vec<Vec<models::SurveyDeposit>>,
            pub expiration: Vec<chrono::DateTime::<chrono::Utc>>,
            pub size: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Survey".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "signature" => intermediate_rep.signature.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "deposits" => return std::result::Result::Err("Parsing a container in this style is not supported in Survey".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "expiration" => intermediate_rep.expiration.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "size" => intermediate_rep.size.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Survey".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Survey {
            signature: intermediate_rep.signature.into_iter().next().ok_or_else(|| "signature missing in Survey".to_string())?,
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in Survey".to_string())?,
            deposits: intermediate_rep.deposits.into_iter().next().ok_or_else(|| "deposits missing in Survey".to_string())?,
            expiration: intermediate_rep.expiration.into_iter().next().ok_or_else(|| "expiration missing in Survey".to_string())?,
            size: intermediate_rep.size.into_iter().next().ok_or_else(|| "size missing in Survey".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Survey> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Survey>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Survey>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Survey - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Survey> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Survey as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Survey - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A surveyed deposit of a mineral or resource available for extraction.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SurveyDeposit {
    /// The symbol of the deposit.
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl SurveyDeposit {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ) -> SurveyDeposit {
        SurveyDeposit {
            symbol,
        }
    }
}

/// Converts the SurveyDeposit value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SurveyDeposit {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SurveyDeposit value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SurveyDeposit {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SurveyDeposit".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SurveyDeposit".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SurveyDeposit {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in SurveyDeposit".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SurveyDeposit> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SurveyDeposit>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SurveyDeposit>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SurveyDeposit - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SurveyDeposit> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SurveyDeposit as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SurveyDeposit - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct System {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "sectorSymbol")]
    pub sector_symbol: String,

    #[serde(rename = "type")]
    pub r#type: models::SystemType,

    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

    #[serde(rename = "waypoints")]
    pub waypoints: Vec<models::SystemWaypoint>,

    #[serde(rename = "factions")]
    pub factions: Vec<models::SystemFaction>,

}

impl System {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, sector_symbol: String, r#type: models::SystemType, x: i32, y: i32, waypoints: Vec<models::SystemWaypoint>, factions: Vec<models::SystemFaction>, ) -> System {
        System {
            symbol,
            sector_symbol,
            r#type,
            x,
            y,
            waypoints,
            factions,
        }
    }
}

/// Converts the System value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for System {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("sectorSymbol".to_string()),
            Some(self.sector_symbol.to_string()),

            // Skipping type in query parameter serialization


            Some("x".to_string()),
            Some(self.x.to_string()),


            Some("y".to_string()),
            Some(self.y.to_string()),

            // Skipping waypoints in query parameter serialization

            // Skipping factions in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a System value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for System {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub sector_symbol: Vec<String>,
            pub r#type: Vec<models::SystemType>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
            pub waypoints: Vec<Vec<models::SystemWaypoint>>,
            pub factions: Vec<Vec<models::SystemFaction>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing System".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sectorSymbol" => intermediate_rep.sector_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::SystemType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "waypoints" => return std::result::Result::Err("Parsing a container in this style is not supported in System".to_string()),
                    "factions" => return std::result::Result::Err("Parsing a container in this style is not supported in System".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing System".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(System {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in System".to_string())?,
            sector_symbol: intermediate_rep.sector_symbol.into_iter().next().ok_or_else(|| "sectorSymbol missing in System".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in System".to_string())?,
            x: intermediate_rep.x.into_iter().next().ok_or_else(|| "x missing in System".to_string())?,
            y: intermediate_rep.y.into_iter().next().ok_or_else(|| "y missing in System".to_string())?,
            waypoints: intermediate_rep.waypoints.into_iter().next().ok_or_else(|| "waypoints missing in System".to_string())?,
            factions: intermediate_rep.factions.into_iter().next().ok_or_else(|| "factions missing in System".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<System> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<System>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<System>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for System - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<System> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <System as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into System - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SystemFaction {
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl SystemFaction {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ) -> SystemFaction {
        SystemFaction {
            symbol,
        }
    }
}

/// Converts the SystemFaction value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SystemFaction {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SystemFaction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SystemFaction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SystemFaction".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SystemFaction".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SystemFaction {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in SystemFaction".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SystemFaction> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SystemFaction>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SystemFaction>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SystemFaction - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SystemFaction> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SystemFaction as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SystemFaction - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The type of waypoint.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum SystemType {
    #[serde(rename = "NEUTRON_STAR")]
    NeutronStar,
    #[serde(rename = "RED_STAR")]
    RedStar,
    #[serde(rename = "ORANGE_STAR")]
    OrangeStar,
    #[serde(rename = "BLUE_STAR")]
    BlueStar,
    #[serde(rename = "YOUNG_STAR")]
    YoungStar,
    #[serde(rename = "WHITE_DWARF")]
    WhiteDwarf,
    #[serde(rename = "BLACK_HOLE")]
    BlackHole,
    #[serde(rename = "HYPERGIANT")]
    Hypergiant,
    #[serde(rename = "NEBULA")]
    Nebula,
    #[serde(rename = "UNSTABLE")]
    Unstable,
}

impl std::fmt::Display for SystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SystemType::NeutronStar => write!(f, "NEUTRON_STAR"),
            SystemType::RedStar => write!(f, "RED_STAR"),
            SystemType::OrangeStar => write!(f, "ORANGE_STAR"),
            SystemType::BlueStar => write!(f, "BLUE_STAR"),
            SystemType::YoungStar => write!(f, "YOUNG_STAR"),
            SystemType::WhiteDwarf => write!(f, "WHITE_DWARF"),
            SystemType::BlackHole => write!(f, "BLACK_HOLE"),
            SystemType::Hypergiant => write!(f, "HYPERGIANT"),
            SystemType::Nebula => write!(f, "NEBULA"),
            SystemType::Unstable => write!(f, "UNSTABLE"),
        }
    }
}

impl std::str::FromStr for SystemType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NEUTRON_STAR" => std::result::Result::Ok(SystemType::NeutronStar),
            "RED_STAR" => std::result::Result::Ok(SystemType::RedStar),
            "ORANGE_STAR" => std::result::Result::Ok(SystemType::OrangeStar),
            "BLUE_STAR" => std::result::Result::Ok(SystemType::BlueStar),
            "YOUNG_STAR" => std::result::Result::Ok(SystemType::YoungStar),
            "WHITE_DWARF" => std::result::Result::Ok(SystemType::WhiteDwarf),
            "BLACK_HOLE" => std::result::Result::Ok(SystemType::BlackHole),
            "HYPERGIANT" => std::result::Result::Ok(SystemType::Hypergiant),
            "NEBULA" => std::result::Result::Ok(SystemType::Nebula),
            "UNSTABLE" => std::result::Result::Ok(SystemType::Unstable),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SystemWaypoint {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "type")]
    pub r#type: models::WaypointType,

    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

}

impl SystemWaypoint {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, r#type: models::WaypointType, x: i32, y: i32, ) -> SystemWaypoint {
        SystemWaypoint {
            symbol,
            r#type,
            x,
            y,
        }
    }
}

/// Converts the SystemWaypoint value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SystemWaypoint {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping type in query parameter serialization


            Some("x".to_string()),
            Some(self.x.to_string()),


            Some("y".to_string()),
            Some(self.y.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SystemWaypoint value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SystemWaypoint {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub r#type: Vec<models::WaypointType>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SystemWaypoint".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::WaypointType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SystemWaypoint".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SystemWaypoint {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in SystemWaypoint".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in SystemWaypoint".to_string())?,
            x: intermediate_rep.x.into_iter().next().ok_or_else(|| "x missing in SystemWaypoint".to_string())?,
            y: intermediate_rep.y.into_iter().next().ok_or_else(|| "y missing in SystemWaypoint".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SystemWaypoint> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SystemWaypoint>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SystemWaypoint>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SystemWaypoint - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SystemWaypoint> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SystemWaypoint as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SystemWaypoint - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeGood {
    #[serde(rename = "symbol")]
    pub symbol: models::TradeSymbol,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

}

impl TradeGood {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: models::TradeSymbol, name: String, description: String, ) -> TradeGood {
        TradeGood {
            symbol,
            name,
            description,
        }
    }
}

/// Converts the TradeGood value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TradeGood {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping symbol in query parameter serialization


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TradeGood value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TradeGood {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<models::TradeSymbol>,
            pub name: Vec<String>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TradeGood".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<models::TradeSymbol as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TradeGood".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TradeGood {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in TradeGood".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in TradeGood".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in TradeGood".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TradeGood> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TradeGood>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TradeGood>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TradeGood - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TradeGood> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TradeGood as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TradeGood - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TradeSymbol {
    #[serde(rename = "PRECIOUS_STONES")]
    PreciousStones,
    #[serde(rename = "QUARTZ_SAND")]
    QuartzSand,
    #[serde(rename = "SILICON_CRYSTALS")]
    SiliconCrystals,
    #[serde(rename = "AMMONIA_ICE")]
    AmmoniaIce,
    #[serde(rename = "LIQUID_HYDROGEN")]
    LiquidHydrogen,
    #[serde(rename = "LIQUID_NITROGEN")]
    LiquidNitrogen,
    #[serde(rename = "ICE_WATER")]
    IceWater,
    #[serde(rename = "EXOTIC_MATTER")]
    ExoticMatter,
    #[serde(rename = "ADVANCED_CIRCUITRY")]
    AdvancedCircuitry,
    #[serde(rename = "GRAVITON_EMITTERS")]
    GravitonEmitters,
    #[serde(rename = "IRON")]
    Iron,
    #[serde(rename = "IRON_ORE")]
    IronOre,
    #[serde(rename = "COPPER")]
    Copper,
    #[serde(rename = "COPPER_ORE")]
    CopperOre,
    #[serde(rename = "ALUMINUM")]
    Aluminum,
    #[serde(rename = "ALUMINUM_ORE")]
    AluminumOre,
    #[serde(rename = "SILVER")]
    Silver,
    #[serde(rename = "SILVER_ORE")]
    SilverOre,
    #[serde(rename = "GOLD")]
    Gold,
    #[serde(rename = "GOLD_ORE")]
    GoldOre,
    #[serde(rename = "PLATINUM")]
    Platinum,
    #[serde(rename = "PLATINUM_ORE")]
    PlatinumOre,
    #[serde(rename = "DIAMONDS")]
    Diamonds,
    #[serde(rename = "URANITE")]
    Uranite,
    #[serde(rename = "URANITE_ORE")]
    UraniteOre,
    #[serde(rename = "MERITIUM")]
    Meritium,
    #[serde(rename = "MERITIUM_ORE")]
    MeritiumOre,
    #[serde(rename = "HYDROCARBON")]
    Hydrocarbon,
    #[serde(rename = "ANTIMATTER")]
    Antimatter,
    #[serde(rename = "FERTILIZERS")]
    Fertilizers,
    #[serde(rename = "FABRICS")]
    Fabrics,
    #[serde(rename = "FOOD")]
    Food,
    #[serde(rename = "JEWELRY")]
    Jewelry,
    #[serde(rename = "MACHINERY")]
    Machinery,
    #[serde(rename = "FIREARMS")]
    Firearms,
    #[serde(rename = "ASSAULT_RIFLES")]
    AssaultRifles,
    #[serde(rename = "MILITARY_EQUIPMENT")]
    MilitaryEquipment,
    #[serde(rename = "EXPLOSIVES")]
    Explosives,
    #[serde(rename = "LAB_INSTRUMENTS")]
    LabInstruments,
    #[serde(rename = "AMMUNITION")]
    Ammunition,
    #[serde(rename = "ELECTRONICS")]
    Electronics,
    #[serde(rename = "SHIP_PLATING")]
    ShipPlating,
    #[serde(rename = "EQUIPMENT")]
    Equipment,
    #[serde(rename = "FUEL")]
    Fuel,
    #[serde(rename = "MEDICINE")]
    Medicine,
    #[serde(rename = "DRUGS")]
    Drugs,
    #[serde(rename = "CLOTHING")]
    Clothing,
    #[serde(rename = "MICROPROCESSORS")]
    Microprocessors,
    #[serde(rename = "PLASTICS")]
    Plastics,
    #[serde(rename = "POLYNUCLEOTIDES")]
    Polynucleotides,
    #[serde(rename = "BIOCOMPOSITES")]
    Biocomposites,
    #[serde(rename = "NANOBOTS")]
    Nanobots,
    #[serde(rename = "AI_MAINFRAMES")]
    AiMainframes,
    #[serde(rename = "QUANTUM_DRIVES")]
    QuantumDrives,
    #[serde(rename = "ROBOTIC_DRONES")]
    RoboticDrones,
    #[serde(rename = "CYBER_IMPLANTS")]
    CyberImplants,
    #[serde(rename = "GENE_THERAPEUTICS")]
    GeneTherapeutics,
    #[serde(rename = "NEURAL_CHIPS")]
    NeuralChips,
    #[serde(rename = "MOOD_REGULATORS")]
    MoodRegulators,
    #[serde(rename = "VIRAL_AGENTS")]
    ViralAgents,
    #[serde(rename = "MICRO_FUSION_GENERATORS")]
    MicroFusionGenerators,
    #[serde(rename = "SUPERGRAINS")]
    Supergrains,
    #[serde(rename = "LASER_RIFLES")]
    LaserRifles,
    #[serde(rename = "HOLOGRAPHICS")]
    Holographics,
    #[serde(rename = "SHIP_SALVAGE")]
    ShipSalvage,
    #[serde(rename = "RELIC_TECH")]
    RelicTech,
    #[serde(rename = "NOVEL_LIFEFORMS")]
    NovelLifeforms,
    #[serde(rename = "BOTANICAL_SPECIMENS")]
    BotanicalSpecimens,
    #[serde(rename = "CULTURAL_ARTIFACTS")]
    CulturalArtifacts,
    #[serde(rename = "REACTOR_SOLAR_I")]
    ReactorSolarI,
    #[serde(rename = "REACTOR_FUSION_I")]
    ReactorFusionI,
    #[serde(rename = "REACTOR_FISSION_I")]
    ReactorFissionI,
    #[serde(rename = "REACTOR_CHEMICAL_I")]
    ReactorChemicalI,
    #[serde(rename = "REACTOR_ANTIMATTER_I")]
    ReactorAntimatterI,
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
    EngineImpulseDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_I")]
    EngineIonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    EngineIonDriveIi,
    #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
    EngineHyperDriveI,
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    ModuleMineralProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    ModuleCargoHoldI,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    ModuleCrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    ModuleEnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    ModulePassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    ModuleMicroRefineryI,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    ModuleOreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    ModuleFuelRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ModuleScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    ModuleJumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    ModuleJumpDriveIi,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    ModuleJumpDriveIii,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    ModuleWarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    ModuleWarpDriveIi,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    ModuleWarpDriveIii,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ModuleShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ModuleShieldGeneratorIi,
    #[serde(rename = "MOUNT_GAS_SIPHON_I")]
    MountGasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    MountGasSiphonIi,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    MountGasSiphonIii,
    #[serde(rename = "MOUNT_SURVEYOR_I")]
    MountSurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    MountSurveyorIi,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    MountSurveyorIii,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
    MountSensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    MountSensorArrayIi,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    MountSensorArrayIii,
    #[serde(rename = "MOUNT_MINING_LASER_I")]
    MountMiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MountMiningLaserIi,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MountMiningLaserIii,
    #[serde(rename = "MOUNT_LASER_CANNON_I")]
    MountLaserCannonI,
    #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
    MountMissileLauncherI,
    #[serde(rename = "MOUNT_TURRET_I")]
    MountTurretI,
}

impl std::fmt::Display for TradeSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TradeSymbol::PreciousStones => write!(f, "PRECIOUS_STONES"),
            TradeSymbol::QuartzSand => write!(f, "QUARTZ_SAND"),
            TradeSymbol::SiliconCrystals => write!(f, "SILICON_CRYSTALS"),
            TradeSymbol::AmmoniaIce => write!(f, "AMMONIA_ICE"),
            TradeSymbol::LiquidHydrogen => write!(f, "LIQUID_HYDROGEN"),
            TradeSymbol::LiquidNitrogen => write!(f, "LIQUID_NITROGEN"),
            TradeSymbol::IceWater => write!(f, "ICE_WATER"),
            TradeSymbol::ExoticMatter => write!(f, "EXOTIC_MATTER"),
            TradeSymbol::AdvancedCircuitry => write!(f, "ADVANCED_CIRCUITRY"),
            TradeSymbol::GravitonEmitters => write!(f, "GRAVITON_EMITTERS"),
            TradeSymbol::Iron => write!(f, "IRON"),
            TradeSymbol::IronOre => write!(f, "IRON_ORE"),
            TradeSymbol::Copper => write!(f, "COPPER"),
            TradeSymbol::CopperOre => write!(f, "COPPER_ORE"),
            TradeSymbol::Aluminum => write!(f, "ALUMINUM"),
            TradeSymbol::AluminumOre => write!(f, "ALUMINUM_ORE"),
            TradeSymbol::Silver => write!(f, "SILVER"),
            TradeSymbol::SilverOre => write!(f, "SILVER_ORE"),
            TradeSymbol::Gold => write!(f, "GOLD"),
            TradeSymbol::GoldOre => write!(f, "GOLD_ORE"),
            TradeSymbol::Platinum => write!(f, "PLATINUM"),
            TradeSymbol::PlatinumOre => write!(f, "PLATINUM_ORE"),
            TradeSymbol::Diamonds => write!(f, "DIAMONDS"),
            TradeSymbol::Uranite => write!(f, "URANITE"),
            TradeSymbol::UraniteOre => write!(f, "URANITE_ORE"),
            TradeSymbol::Meritium => write!(f, "MERITIUM"),
            TradeSymbol::MeritiumOre => write!(f, "MERITIUM_ORE"),
            TradeSymbol::Hydrocarbon => write!(f, "HYDROCARBON"),
            TradeSymbol::Antimatter => write!(f, "ANTIMATTER"),
            TradeSymbol::Fertilizers => write!(f, "FERTILIZERS"),
            TradeSymbol::Fabrics => write!(f, "FABRICS"),
            TradeSymbol::Food => write!(f, "FOOD"),
            TradeSymbol::Jewelry => write!(f, "JEWELRY"),
            TradeSymbol::Machinery => write!(f, "MACHINERY"),
            TradeSymbol::Firearms => write!(f, "FIREARMS"),
            TradeSymbol::AssaultRifles => write!(f, "ASSAULT_RIFLES"),
            TradeSymbol::MilitaryEquipment => write!(f, "MILITARY_EQUIPMENT"),
            TradeSymbol::Explosives => write!(f, "EXPLOSIVES"),
            TradeSymbol::LabInstruments => write!(f, "LAB_INSTRUMENTS"),
            TradeSymbol::Ammunition => write!(f, "AMMUNITION"),
            TradeSymbol::Electronics => write!(f, "ELECTRONICS"),
            TradeSymbol::ShipPlating => write!(f, "SHIP_PLATING"),
            TradeSymbol::Equipment => write!(f, "EQUIPMENT"),
            TradeSymbol::Fuel => write!(f, "FUEL"),
            TradeSymbol::Medicine => write!(f, "MEDICINE"),
            TradeSymbol::Drugs => write!(f, "DRUGS"),
            TradeSymbol::Clothing => write!(f, "CLOTHING"),
            TradeSymbol::Microprocessors => write!(f, "MICROPROCESSORS"),
            TradeSymbol::Plastics => write!(f, "PLASTICS"),
            TradeSymbol::Polynucleotides => write!(f, "POLYNUCLEOTIDES"),
            TradeSymbol::Biocomposites => write!(f, "BIOCOMPOSITES"),
            TradeSymbol::Nanobots => write!(f, "NANOBOTS"),
            TradeSymbol::AiMainframes => write!(f, "AI_MAINFRAMES"),
            TradeSymbol::QuantumDrives => write!(f, "QUANTUM_DRIVES"),
            TradeSymbol::RoboticDrones => write!(f, "ROBOTIC_DRONES"),
            TradeSymbol::CyberImplants => write!(f, "CYBER_IMPLANTS"),
            TradeSymbol::GeneTherapeutics => write!(f, "GENE_THERAPEUTICS"),
            TradeSymbol::NeuralChips => write!(f, "NEURAL_CHIPS"),
            TradeSymbol::MoodRegulators => write!(f, "MOOD_REGULATORS"),
            TradeSymbol::ViralAgents => write!(f, "VIRAL_AGENTS"),
            TradeSymbol::MicroFusionGenerators => write!(f, "MICRO_FUSION_GENERATORS"),
            TradeSymbol::Supergrains => write!(f, "SUPERGRAINS"),
            TradeSymbol::LaserRifles => write!(f, "LASER_RIFLES"),
            TradeSymbol::Holographics => write!(f, "HOLOGRAPHICS"),
            TradeSymbol::ShipSalvage => write!(f, "SHIP_SALVAGE"),
            TradeSymbol::RelicTech => write!(f, "RELIC_TECH"),
            TradeSymbol::NovelLifeforms => write!(f, "NOVEL_LIFEFORMS"),
            TradeSymbol::BotanicalSpecimens => write!(f, "BOTANICAL_SPECIMENS"),
            TradeSymbol::CulturalArtifacts => write!(f, "CULTURAL_ARTIFACTS"),
            TradeSymbol::ReactorSolarI => write!(f, "REACTOR_SOLAR_I"),
            TradeSymbol::ReactorFusionI => write!(f, "REACTOR_FUSION_I"),
            TradeSymbol::ReactorFissionI => write!(f, "REACTOR_FISSION_I"),
            TradeSymbol::ReactorChemicalI => write!(f, "REACTOR_CHEMICAL_I"),
            TradeSymbol::ReactorAntimatterI => write!(f, "REACTOR_ANTIMATTER_I"),
            TradeSymbol::EngineImpulseDriveI => write!(f, "ENGINE_IMPULSE_DRIVE_I"),
            TradeSymbol::EngineIonDriveI => write!(f, "ENGINE_ION_DRIVE_I"),
            TradeSymbol::EngineIonDriveIi => write!(f, "ENGINE_ION_DRIVE_II"),
            TradeSymbol::EngineHyperDriveI => write!(f, "ENGINE_HYPER_DRIVE_I"),
            TradeSymbol::ModuleMineralProcessorI => write!(f, "MODULE_MINERAL_PROCESSOR_I"),
            TradeSymbol::ModuleCargoHoldI => write!(f, "MODULE_CARGO_HOLD_I"),
            TradeSymbol::ModuleCrewQuartersI => write!(f, "MODULE_CREW_QUARTERS_I"),
            TradeSymbol::ModuleEnvoyQuartersI => write!(f, "MODULE_ENVOY_QUARTERS_I"),
            TradeSymbol::ModulePassengerCabinI => write!(f, "MODULE_PASSENGER_CABIN_I"),
            TradeSymbol::ModuleMicroRefineryI => write!(f, "MODULE_MICRO_REFINERY_I"),
            TradeSymbol::ModuleOreRefineryI => write!(f, "MODULE_ORE_REFINERY_I"),
            TradeSymbol::ModuleFuelRefineryI => write!(f, "MODULE_FUEL_REFINERY_I"),
            TradeSymbol::ModuleScienceLabI => write!(f, "MODULE_SCIENCE_LAB_I"),
            TradeSymbol::ModuleJumpDriveI => write!(f, "MODULE_JUMP_DRIVE_I"),
            TradeSymbol::ModuleJumpDriveIi => write!(f, "MODULE_JUMP_DRIVE_II"),
            TradeSymbol::ModuleJumpDriveIii => write!(f, "MODULE_JUMP_DRIVE_III"),
            TradeSymbol::ModuleWarpDriveI => write!(f, "MODULE_WARP_DRIVE_I"),
            TradeSymbol::ModuleWarpDriveIi => write!(f, "MODULE_WARP_DRIVE_II"),
            TradeSymbol::ModuleWarpDriveIii => write!(f, "MODULE_WARP_DRIVE_III"),
            TradeSymbol::ModuleShieldGeneratorI => write!(f, "MODULE_SHIELD_GENERATOR_I"),
            TradeSymbol::ModuleShieldGeneratorIi => write!(f, "MODULE_SHIELD_GENERATOR_II"),
            TradeSymbol::MountGasSiphonI => write!(f, "MOUNT_GAS_SIPHON_I"),
            TradeSymbol::MountGasSiphonIi => write!(f, "MOUNT_GAS_SIPHON_II"),
            TradeSymbol::MountGasSiphonIii => write!(f, "MOUNT_GAS_SIPHON_III"),
            TradeSymbol::MountSurveyorI => write!(f, "MOUNT_SURVEYOR_I"),
            TradeSymbol::MountSurveyorIi => write!(f, "MOUNT_SURVEYOR_II"),
            TradeSymbol::MountSurveyorIii => write!(f, "MOUNT_SURVEYOR_III"),
            TradeSymbol::MountSensorArrayI => write!(f, "MOUNT_SENSOR_ARRAY_I"),
            TradeSymbol::MountSensorArrayIi => write!(f, "MOUNT_SENSOR_ARRAY_II"),
            TradeSymbol::MountSensorArrayIii => write!(f, "MOUNT_SENSOR_ARRAY_III"),
            TradeSymbol::MountMiningLaserI => write!(f, "MOUNT_MINING_LASER_I"),
            TradeSymbol::MountMiningLaserIi => write!(f, "MOUNT_MINING_LASER_II"),
            TradeSymbol::MountMiningLaserIii => write!(f, "MOUNT_MINING_LASER_III"),
            TradeSymbol::MountLaserCannonI => write!(f, "MOUNT_LASER_CANNON_I"),
            TradeSymbol::MountMissileLauncherI => write!(f, "MOUNT_MISSILE_LAUNCHER_I"),
            TradeSymbol::MountTurretI => write!(f, "MOUNT_TURRET_I"),
        }
    }
}

impl std::str::FromStr for TradeSymbol {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PRECIOUS_STONES" => std::result::Result::Ok(TradeSymbol::PreciousStones),
            "QUARTZ_SAND" => std::result::Result::Ok(TradeSymbol::QuartzSand),
            "SILICON_CRYSTALS" => std::result::Result::Ok(TradeSymbol::SiliconCrystals),
            "AMMONIA_ICE" => std::result::Result::Ok(TradeSymbol::AmmoniaIce),
            "LIQUID_HYDROGEN" => std::result::Result::Ok(TradeSymbol::LiquidHydrogen),
            "LIQUID_NITROGEN" => std::result::Result::Ok(TradeSymbol::LiquidNitrogen),
            "ICE_WATER" => std::result::Result::Ok(TradeSymbol::IceWater),
            "EXOTIC_MATTER" => std::result::Result::Ok(TradeSymbol::ExoticMatter),
            "ADVANCED_CIRCUITRY" => std::result::Result::Ok(TradeSymbol::AdvancedCircuitry),
            "GRAVITON_EMITTERS" => std::result::Result::Ok(TradeSymbol::GravitonEmitters),
            "IRON" => std::result::Result::Ok(TradeSymbol::Iron),
            "IRON_ORE" => std::result::Result::Ok(TradeSymbol::IronOre),
            "COPPER" => std::result::Result::Ok(TradeSymbol::Copper),
            "COPPER_ORE" => std::result::Result::Ok(TradeSymbol::CopperOre),
            "ALUMINUM" => std::result::Result::Ok(TradeSymbol::Aluminum),
            "ALUMINUM_ORE" => std::result::Result::Ok(TradeSymbol::AluminumOre),
            "SILVER" => std::result::Result::Ok(TradeSymbol::Silver),
            "SILVER_ORE" => std::result::Result::Ok(TradeSymbol::SilverOre),
            "GOLD" => std::result::Result::Ok(TradeSymbol::Gold),
            "GOLD_ORE" => std::result::Result::Ok(TradeSymbol::GoldOre),
            "PLATINUM" => std::result::Result::Ok(TradeSymbol::Platinum),
            "PLATINUM_ORE" => std::result::Result::Ok(TradeSymbol::PlatinumOre),
            "DIAMONDS" => std::result::Result::Ok(TradeSymbol::Diamonds),
            "URANITE" => std::result::Result::Ok(TradeSymbol::Uranite),
            "URANITE_ORE" => std::result::Result::Ok(TradeSymbol::UraniteOre),
            "MERITIUM" => std::result::Result::Ok(TradeSymbol::Meritium),
            "MERITIUM_ORE" => std::result::Result::Ok(TradeSymbol::MeritiumOre),
            "HYDROCARBON" => std::result::Result::Ok(TradeSymbol::Hydrocarbon),
            "ANTIMATTER" => std::result::Result::Ok(TradeSymbol::Antimatter),
            "FERTILIZERS" => std::result::Result::Ok(TradeSymbol::Fertilizers),
            "FABRICS" => std::result::Result::Ok(TradeSymbol::Fabrics),
            "FOOD" => std::result::Result::Ok(TradeSymbol::Food),
            "JEWELRY" => std::result::Result::Ok(TradeSymbol::Jewelry),
            "MACHINERY" => std::result::Result::Ok(TradeSymbol::Machinery),
            "FIREARMS" => std::result::Result::Ok(TradeSymbol::Firearms),
            "ASSAULT_RIFLES" => std::result::Result::Ok(TradeSymbol::AssaultRifles),
            "MILITARY_EQUIPMENT" => std::result::Result::Ok(TradeSymbol::MilitaryEquipment),
            "EXPLOSIVES" => std::result::Result::Ok(TradeSymbol::Explosives),
            "LAB_INSTRUMENTS" => std::result::Result::Ok(TradeSymbol::LabInstruments),
            "AMMUNITION" => std::result::Result::Ok(TradeSymbol::Ammunition),
            "ELECTRONICS" => std::result::Result::Ok(TradeSymbol::Electronics),
            "SHIP_PLATING" => std::result::Result::Ok(TradeSymbol::ShipPlating),
            "EQUIPMENT" => std::result::Result::Ok(TradeSymbol::Equipment),
            "FUEL" => std::result::Result::Ok(TradeSymbol::Fuel),
            "MEDICINE" => std::result::Result::Ok(TradeSymbol::Medicine),
            "DRUGS" => std::result::Result::Ok(TradeSymbol::Drugs),
            "CLOTHING" => std::result::Result::Ok(TradeSymbol::Clothing),
            "MICROPROCESSORS" => std::result::Result::Ok(TradeSymbol::Microprocessors),
            "PLASTICS" => std::result::Result::Ok(TradeSymbol::Plastics),
            "POLYNUCLEOTIDES" => std::result::Result::Ok(TradeSymbol::Polynucleotides),
            "BIOCOMPOSITES" => std::result::Result::Ok(TradeSymbol::Biocomposites),
            "NANOBOTS" => std::result::Result::Ok(TradeSymbol::Nanobots),
            "AI_MAINFRAMES" => std::result::Result::Ok(TradeSymbol::AiMainframes),
            "QUANTUM_DRIVES" => std::result::Result::Ok(TradeSymbol::QuantumDrives),
            "ROBOTIC_DRONES" => std::result::Result::Ok(TradeSymbol::RoboticDrones),
            "CYBER_IMPLANTS" => std::result::Result::Ok(TradeSymbol::CyberImplants),
            "GENE_THERAPEUTICS" => std::result::Result::Ok(TradeSymbol::GeneTherapeutics),
            "NEURAL_CHIPS" => std::result::Result::Ok(TradeSymbol::NeuralChips),
            "MOOD_REGULATORS" => std::result::Result::Ok(TradeSymbol::MoodRegulators),
            "VIRAL_AGENTS" => std::result::Result::Ok(TradeSymbol::ViralAgents),
            "MICRO_FUSION_GENERATORS" => std::result::Result::Ok(TradeSymbol::MicroFusionGenerators),
            "SUPERGRAINS" => std::result::Result::Ok(TradeSymbol::Supergrains),
            "LASER_RIFLES" => std::result::Result::Ok(TradeSymbol::LaserRifles),
            "HOLOGRAPHICS" => std::result::Result::Ok(TradeSymbol::Holographics),
            "SHIP_SALVAGE" => std::result::Result::Ok(TradeSymbol::ShipSalvage),
            "RELIC_TECH" => std::result::Result::Ok(TradeSymbol::RelicTech),
            "NOVEL_LIFEFORMS" => std::result::Result::Ok(TradeSymbol::NovelLifeforms),
            "BOTANICAL_SPECIMENS" => std::result::Result::Ok(TradeSymbol::BotanicalSpecimens),
            "CULTURAL_ARTIFACTS" => std::result::Result::Ok(TradeSymbol::CulturalArtifacts),
            "REACTOR_SOLAR_I" => std::result::Result::Ok(TradeSymbol::ReactorSolarI),
            "REACTOR_FUSION_I" => std::result::Result::Ok(TradeSymbol::ReactorFusionI),
            "REACTOR_FISSION_I" => std::result::Result::Ok(TradeSymbol::ReactorFissionI),
            "REACTOR_CHEMICAL_I" => std::result::Result::Ok(TradeSymbol::ReactorChemicalI),
            "REACTOR_ANTIMATTER_I" => std::result::Result::Ok(TradeSymbol::ReactorAntimatterI),
            "ENGINE_IMPULSE_DRIVE_I" => std::result::Result::Ok(TradeSymbol::EngineImpulseDriveI),
            "ENGINE_ION_DRIVE_I" => std::result::Result::Ok(TradeSymbol::EngineIonDriveI),
            "ENGINE_ION_DRIVE_II" => std::result::Result::Ok(TradeSymbol::EngineIonDriveIi),
            "ENGINE_HYPER_DRIVE_I" => std::result::Result::Ok(TradeSymbol::EngineHyperDriveI),
            "MODULE_MINERAL_PROCESSOR_I" => std::result::Result::Ok(TradeSymbol::ModuleMineralProcessorI),
            "MODULE_CARGO_HOLD_I" => std::result::Result::Ok(TradeSymbol::ModuleCargoHoldI),
            "MODULE_CREW_QUARTERS_I" => std::result::Result::Ok(TradeSymbol::ModuleCrewQuartersI),
            "MODULE_ENVOY_QUARTERS_I" => std::result::Result::Ok(TradeSymbol::ModuleEnvoyQuartersI),
            "MODULE_PASSENGER_CABIN_I" => std::result::Result::Ok(TradeSymbol::ModulePassengerCabinI),
            "MODULE_MICRO_REFINERY_I" => std::result::Result::Ok(TradeSymbol::ModuleMicroRefineryI),
            "MODULE_ORE_REFINERY_I" => std::result::Result::Ok(TradeSymbol::ModuleOreRefineryI),
            "MODULE_FUEL_REFINERY_I" => std::result::Result::Ok(TradeSymbol::ModuleFuelRefineryI),
            "MODULE_SCIENCE_LAB_I" => std::result::Result::Ok(TradeSymbol::ModuleScienceLabI),
            "MODULE_JUMP_DRIVE_I" => std::result::Result::Ok(TradeSymbol::ModuleJumpDriveI),
            "MODULE_JUMP_DRIVE_II" => std::result::Result::Ok(TradeSymbol::ModuleJumpDriveIi),
            "MODULE_JUMP_DRIVE_III" => std::result::Result::Ok(TradeSymbol::ModuleJumpDriveIii),
            "MODULE_WARP_DRIVE_I" => std::result::Result::Ok(TradeSymbol::ModuleWarpDriveI),
            "MODULE_WARP_DRIVE_II" => std::result::Result::Ok(TradeSymbol::ModuleWarpDriveIi),
            "MODULE_WARP_DRIVE_III" => std::result::Result::Ok(TradeSymbol::ModuleWarpDriveIii),
            "MODULE_SHIELD_GENERATOR_I" => std::result::Result::Ok(TradeSymbol::ModuleShieldGeneratorI),
            "MODULE_SHIELD_GENERATOR_II" => std::result::Result::Ok(TradeSymbol::ModuleShieldGeneratorIi),
            "MOUNT_GAS_SIPHON_I" => std::result::Result::Ok(TradeSymbol::MountGasSiphonI),
            "MOUNT_GAS_SIPHON_II" => std::result::Result::Ok(TradeSymbol::MountGasSiphonIi),
            "MOUNT_GAS_SIPHON_III" => std::result::Result::Ok(TradeSymbol::MountGasSiphonIii),
            "MOUNT_SURVEYOR_I" => std::result::Result::Ok(TradeSymbol::MountSurveyorI),
            "MOUNT_SURVEYOR_II" => std::result::Result::Ok(TradeSymbol::MountSurveyorIi),
            "MOUNT_SURVEYOR_III" => std::result::Result::Ok(TradeSymbol::MountSurveyorIii),
            "MOUNT_SENSOR_ARRAY_I" => std::result::Result::Ok(TradeSymbol::MountSensorArrayI),
            "MOUNT_SENSOR_ARRAY_II" => std::result::Result::Ok(TradeSymbol::MountSensorArrayIi),
            "MOUNT_SENSOR_ARRAY_III" => std::result::Result::Ok(TradeSymbol::MountSensorArrayIii),
            "MOUNT_MINING_LASER_I" => std::result::Result::Ok(TradeSymbol::MountMiningLaserI),
            "MOUNT_MINING_LASER_II" => std::result::Result::Ok(TradeSymbol::MountMiningLaserIi),
            "MOUNT_MINING_LASER_III" => std::result::Result::Ok(TradeSymbol::MountMiningLaserIii),
            "MOUNT_LASER_CANNON_I" => std::result::Result::Ok(TradeSymbol::MountLaserCannonI),
            "MOUNT_MISSILE_LAUNCHER_I" => std::result::Result::Ok(TradeSymbol::MountMissileLauncherI),
            "MOUNT_TURRET_I" => std::result::Result::Ok(TradeSymbol::MountTurretI),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TransferCargo200Response {
    #[serde(rename = "data")]
    pub data: models::Jettison200ResponseData,

}

impl TransferCargo200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::Jettison200ResponseData, ) -> TransferCargo200Response {
        TransferCargo200Response {
            data,
        }
    }
}

/// Converts the TransferCargo200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TransferCargo200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TransferCargo200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TransferCargo200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::Jettison200ResponseData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TransferCargo200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(<models::Jettison200ResponseData as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TransferCargo200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TransferCargo200Response {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in TransferCargo200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TransferCargo200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TransferCargo200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TransferCargo200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TransferCargo200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TransferCargo200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TransferCargo200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TransferCargo200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TransferCargoRequest {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,

    #[serde(rename = "units")]
    pub units: i32,

    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,

}

impl TransferCargoRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(trade_symbol: String, units: i32, ship_symbol: String, ) -> TransferCargoRequest {
        TransferCargoRequest {
            trade_symbol,
            units,
            ship_symbol,
        }
    }
}

/// Converts the TransferCargoRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TransferCargoRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("tradeSymbol".to_string()),
            Some(self.trade_symbol.to_string()),


            Some("units".to_string()),
            Some(self.units.to_string()),


            Some("shipSymbol".to_string()),
            Some(self.ship_symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TransferCargoRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TransferCargoRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trade_symbol: Vec<String>,
            pub units: Vec<i32>,
            pub ship_symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TransferCargoRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tradeSymbol" => intermediate_rep.trade_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "units" => intermediate_rep.units.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "shipSymbol" => intermediate_rep.ship_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TransferCargoRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TransferCargoRequest {
            trade_symbol: intermediate_rep.trade_symbol.into_iter().next().ok_or_else(|| "tradeSymbol missing in TransferCargoRequest".to_string())?,
            units: intermediate_rep.units.into_iter().next().ok_or_else(|| "units missing in TransferCargoRequest".to_string())?,
            ship_symbol: intermediate_rep.ship_symbol.into_iter().next().ok_or_else(|| "shipSymbol missing in TransferCargoRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TransferCargoRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TransferCargoRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TransferCargoRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TransferCargoRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TransferCargoRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TransferCargoRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TransferCargoRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A waypoint is a location that ships can travel to such as a Planet, Moon or Space Station.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Waypoint {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "type")]
    pub r#type: models::WaypointType,

    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,

    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

    #[serde(rename = "orbitals")]
    pub orbitals: Vec<models::WaypointOrbital>,

    #[serde(rename = "faction")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub faction: Option<models::WaypointFaction>,

    /// The traits of the waypoint.
    #[serde(rename = "traits")]
    pub traits: Vec<models::WaypointTrait>,

    #[serde(rename = "chart")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chart: Option<models::Chart>,

}

impl Waypoint {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, r#type: models::WaypointType, system_symbol: String, x: i32, y: i32, orbitals: Vec<models::WaypointOrbital>, traits: Vec<models::WaypointTrait>, ) -> Waypoint {
        Waypoint {
            symbol,
            r#type,
            system_symbol,
            x,
            y,
            orbitals,
            faction: None,
            traits,
            chart: None,
        }
    }
}

/// Converts the Waypoint value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Waypoint {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

            // Skipping type in query parameter serialization


            Some("systemSymbol".to_string()),
            Some(self.system_symbol.to_string()),


            Some("x".to_string()),
            Some(self.x.to_string()),


            Some("y".to_string()),
            Some(self.y.to_string()),

            // Skipping orbitals in query parameter serialization

            // Skipping faction in query parameter serialization

            // Skipping traits in query parameter serialization

            // Skipping chart in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Waypoint value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Waypoint {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub r#type: Vec<models::WaypointType>,
            pub system_symbol: Vec<String>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
            pub orbitals: Vec<Vec<models::WaypointOrbital>>,
            pub faction: Vec<models::WaypointFaction>,
            pub traits: Vec<Vec<models::WaypointTrait>>,
            pub chart: Vec<models::Chart>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Waypoint".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::WaypointType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "systemSymbol" => intermediate_rep.system_symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "orbitals" => return std::result::Result::Err("Parsing a container in this style is not supported in Waypoint".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "faction" => intermediate_rep.faction.push(<models::WaypointFaction as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "traits" => return std::result::Result::Err("Parsing a container in this style is not supported in Waypoint".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "chart" => intermediate_rep.chart.push(<models::Chart as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Waypoint".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Waypoint {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in Waypoint".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in Waypoint".to_string())?,
            system_symbol: intermediate_rep.system_symbol.into_iter().next().ok_or_else(|| "systemSymbol missing in Waypoint".to_string())?,
            x: intermediate_rep.x.into_iter().next().ok_or_else(|| "x missing in Waypoint".to_string())?,
            y: intermediate_rep.y.into_iter().next().ok_or_else(|| "y missing in Waypoint".to_string())?,
            orbitals: intermediate_rep.orbitals.into_iter().next().ok_or_else(|| "orbitals missing in Waypoint".to_string())?,
            faction: intermediate_rep.faction.into_iter().next(),
            traits: intermediate_rep.traits.into_iter().next().ok_or_else(|| "traits missing in Waypoint".to_string())?,
            chart: intermediate_rep.chart.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Waypoint> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Waypoint>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Waypoint>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Waypoint - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Waypoint> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Waypoint as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Waypoint - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WaypointFaction {
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl WaypointFaction {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ) -> WaypointFaction {
        WaypointFaction {
            symbol,
        }
    }
}

/// Converts the WaypointFaction value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WaypointFaction {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WaypointFaction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WaypointFaction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WaypointFaction".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing WaypointFaction".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WaypointFaction {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in WaypointFaction".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WaypointFaction> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WaypointFaction>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WaypointFaction>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WaypointFaction - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WaypointFaction> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WaypointFaction as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WaypointFaction - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// An orbital is another waypoint that orbits a parent waypoint.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WaypointOrbital {
    #[serde(rename = "symbol")]
    pub symbol: String,

}

impl WaypointOrbital {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, ) -> WaypointOrbital {
        WaypointOrbital {
            symbol,
        }
    }
}

/// Converts the WaypointOrbital value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WaypointOrbital {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WaypointOrbital value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WaypointOrbital {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WaypointOrbital".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing WaypointOrbital".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WaypointOrbital {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in WaypointOrbital".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WaypointOrbital> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WaypointOrbital>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WaypointOrbital>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WaypointOrbital - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WaypointOrbital> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WaypointOrbital as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WaypointOrbital - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WaypointTrait {
    /// The unique identifier of the trait.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// The name of the trait.
    #[serde(rename = "name")]
    pub name: String,

    /// A description of the trait.
    #[serde(rename = "description")]
    pub description: String,

}

impl WaypointTrait {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, name: String, description: String, ) -> WaypointTrait {
        WaypointTrait {
            symbol,
            name,
            description,
        }
    }
}

/// Converts the WaypointTrait value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WaypointTrait {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("description".to_string()),
            Some(self.description.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WaypointTrait value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WaypointTrait {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WaypointTrait".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing WaypointTrait".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WaypointTrait {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in WaypointTrait".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in WaypointTrait".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or_else(|| "description missing in WaypointTrait".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WaypointTrait> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WaypointTrait>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WaypointTrait>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WaypointTrait - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WaypointTrait> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WaypointTrait as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WaypointTrait - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The type of waypoint.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum WaypointType {
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

impl std::fmt::Display for WaypointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WaypointType::Planet => write!(f, "PLANET"),
            WaypointType::GasGiant => write!(f, "GAS_GIANT"),
            WaypointType::Moon => write!(f, "MOON"),
            WaypointType::OrbitalStation => write!(f, "ORBITAL_STATION"),
            WaypointType::JumpGate => write!(f, "JUMP_GATE"),
            WaypointType::AsteroidField => write!(f, "ASTEROID_FIELD"),
            WaypointType::Nebula => write!(f, "NEBULA"),
            WaypointType::DebrisField => write!(f, "DEBRIS_FIELD"),
            WaypointType::GravityWell => write!(f, "GRAVITY_WELL"),
        }
    }
}

impl std::str::FromStr for WaypointType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PLANET" => std::result::Result::Ok(WaypointType::Planet),
            "GAS_GIANT" => std::result::Result::Ok(WaypointType::GasGiant),
            "MOON" => std::result::Result::Ok(WaypointType::Moon),
            "ORBITAL_STATION" => std::result::Result::Ok(WaypointType::OrbitalStation),
            "JUMP_GATE" => std::result::Result::Ok(WaypointType::JumpGate),
            "ASTEROID_FIELD" => std::result::Result::Ok(WaypointType::AsteroidField),
            "NEBULA" => std::result::Result::Ok(WaypointType::Nebula),
            "DEBRIS_FIELD" => std::result::Result::Ok(WaypointType::DebrisField),
            "GRAVITY_WELL" => std::result::Result::Ok(WaypointType::GravityWell),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
