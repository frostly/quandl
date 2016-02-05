use std::fmt::{self, Formatter, Debug};
use hyper;
use requests::DatasetDataCall;
use HttpClient;

/// Parameters for Quandl
pub struct Quandl {
    /// Http client
    http_client: hyper::Client,
    /// Quandl API key. Used for premium databases and/or increased usage limits
    pub api_key: Option<String>,
}

impl Quandl {
    /// Creates a struct that holds the http client and an optional api_key
    pub fn new() -> Quandl {
        Default::default()
    }

    /// Creates a new `DatasetDataCall` using the specified database_code and dataset code.
    /// All other parameters as taken from the default implementation, setting the optional
    /// parameters to `None`.
    pub fn new_dataset_data_call(&self,
                                 database_code: &str,
                                 dataset_code: &str)
                                 -> DatasetDataCall {
        DatasetDataCall {
            database_code: String::from(database_code),
            dataset_code: String::from(dataset_code),
            ..DatasetDataCall::default(&self)
        }
    }
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
        fmt.debug_struct("Quandl")
           .field("api_key", &self.api_key)
           .finish()
    }
}

impl HttpClient for Quandl {
    fn http_client(&self) -> &hyper::Client {
        &self.http_client
    }
}
