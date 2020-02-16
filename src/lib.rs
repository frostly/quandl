#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", allow(unstable_features))]
#![deny(clippy::all)]

//! Library to fetch data from the [Quandl v3 API](https://www.quandl.com/docs/api)
//! for financial and economic datasets.

extern crate hyper;
extern crate serde_json;
extern crate tokio;
#[macro_use]
extern crate quick_error;
extern crate chrono;

pub use crate::chrono::NaiveDate;
pub use crate::error::{Error, Result};
pub use crate::quandl::Quandl;
pub use crate::quandl_request::*;
pub use crate::serde_json::Value as JsonValue;

/// Errors
pub mod error;
/// Handles common information across requests.
pub mod quandl;
/// Handles building and sending requests to Quandl
pub mod quandl_request;
