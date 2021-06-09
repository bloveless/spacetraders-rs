//! This crate is an api wrapper around the popular [SpaceTraders] API game of the same name.
//!
//! The goal is to wrap all available endpoints in strongly typed requests and responses that
//! we expect from rust.
//!
//! [SpaceTraders]: https://spacetraders.io/
#![warn(missing_docs)]

pub mod client;
pub mod shared;
pub mod responses;
pub mod requests;
pub mod errors;
