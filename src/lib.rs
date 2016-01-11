#![deny(missing_docs,
        missing_debug_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(feature = "clippy", allow(unstable_features))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))]

//! Library to fetch data from the [Quandl v3 API](https://www.quandl.com/docs/api)
//! for financial and economic datasets.

extern crate curl;
extern crate serde_json;
extern crate url;
extern crate hyper;
#[macro_use] extern crate quick_error;
extern crate chrono;

pub use quandl::Quandl;
pub use quandl_request::*;
pub use error::{Error, Result};
pub use serde_json::Value as JsonValue;
pub use chrono::NaiveDate as NaiveDate;

/// Handles common information across requests.
pub mod quandl;
/// Handles building and sending requests to Quandl
pub mod quandl_request;
/// Errors
pub mod error;
