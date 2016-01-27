use hyper;
use serde_json;
use chrono;
use url;

/// Result type often returned from methods that can have quandl `Error`s.
pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    /// Error enum for quandl lib
    #[derive(Debug)]
    pub enum Error {
        /// Hyper client errors
        Hyper(err: hyper::Error) {
            from()
            description("hyper error")
            display("hyper error: {}", err)
            cause(err)
        }
        /// Serde json error
        SerdeJson(err: serde_json::error::Error) {
            from()
            description("serde_json error")
            display("serde_json error: {}", err)
            cause(err)
        }
        /// Chrono parse error
        Chrono(err: chrono::format::ParseError) {
            from()
            description("chrono parse error")
            display("chrono parse error: {}", err)
            cause(err)
        }
        /// Quandl error returned from request
        Quandl(err: String) {
            description("quandl error")
            display("quandl error: {}", err)
        }
        /// Date error used to ensure start_date > end_date
        Date(err: String) {
            description("date error")
            display("date error: {}", err)
        }
        /// Url Error
        Url(err: url::ParseError) {
            from()
            description("url parse error")
            display("url parse error: {}", err)
            cause(err)
        }
    }
}
