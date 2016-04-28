// #![deny(missing_docs,
//         missing_debug_implementations,
//         trivial_casts,
//         trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces,
//         unused_qualifications)]
// #![cfg_attr(all(test, feature = "nightly"), feature(test))]
// #![cfg_attr(test, deny(warnings))]
// #![cfg_attr(feature = "clippy", allow(unstable_features))]
// #![cfg_attr(feature = "clippy", feature(plugin))]
// #![cfg_attr(feature = "clippy", plugin(clippy))]
// #![cfg_attr(feature = "clippy", deny(clippy))]

//! Library to fetch data from the [Quandl v3 API](https://www.quandl.com/docs/api)
//! for financial and economic datasets.

extern crate curl;
extern crate serde_json;
extern crate url;
extern crate hyper;
#[macro_use]
extern crate quick_error;
extern crate chrono;
extern crate zip;
extern crate quick_csv;
#[cfg(test)]
#[macro_use]
extern crate yup_hyper_mock as hyper_mock;
#[cfg(test)]
#[macro_use]
extern crate log;

pub use quandl::Quandl;
pub use requests::dataset_data_call::*;
pub use error::{Error, Result};
pub use serde_json::Value as JsonValue;
pub use chrono::NaiveDate;

/// macros
#[macro_use]
mod macros;
/// Handles common information across requests.
pub mod quandl;
/// Handles building and sending requests to Quandl
pub mod requests;
/// Errors
pub mod error;

/// Avoid exposing quandl.http_client to public but allow code inside the crate to use it
/// via this trait
trait HttpClient {
    fn http_client(&self) -> &hyper::Client;
}
