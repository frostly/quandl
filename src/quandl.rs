use std::fmt::{self, Formatter, Debug};
use hyper;
use super::QuandlRequest;

/// Parameters for Quandl
pub struct Quandl {
    /// Http client
    pub http_client: hyper::Client,
    /// Quandl API key. Used for premium databases and/or increased usage limits
    pub api_key: Option<String>,
}

impl Quandl {
    /// Creates a struct that holds the http client and an optional api_key
    pub fn new() -> Quandl {
        Default::default()
    }

    /// Creates a new `QuandlRequest` using the specified database_code and dataset code.
    /// All other parameters as taken from the default implementation, setting the optional
    /// parameters to `None`.
    pub fn new_request(&self, database_code: &str, dataset_code: &str) -> QuandlRequest {
        QuandlRequest {
            database_code: String::from(database_code),
            dataset_code: String::from(dataset_code),
            ..QuandlRequest::default(&self)
        }
    }

    /// Quandl API key. Used for premium databases and/or increased usage limits.
    pub fn api_key(mut self, key: &str) -> Quandl {
        self.api_key = Some(String::from(key));
        self
    }
}

impl Default for Quandl {
    fn default() -> Quandl {
        Quandl {
            http_client: hyper::Client::new(),
            api_key: None,
        }
    }
}

impl Debug for Quandl {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("QuandlRequest")
           .field("api_key", &self.api_key)
           .finish()
    }
}
