use url::Url;
use std::fmt::{Display, Formatter};
use hyper;
use serde_json;
use error::{Error, Result};
use chrono::NaiveDate;

/// use v3 of Quandl API
const QUANDL_BASE_URL: &'static str = "https://www.quandl.com/api/v3/datasets";

/// Parameters for the request to Quandl API
#[derive(Debug, PartialEq)]
pub struct QuandlRequest {
    /// Quandl API key. Used for premium databases and/or increased usage limits
    api_key: Option<String>,
    /// The unique database code on Quandl (ex. WIKI)
    database_code: String,
    /// The unique dataset code on Quandl (ex. APPL)
    dataset_code: String,
    limit: Option<u64>,
    rows: Option<u64>,
    /// Request specific column.
    column_index: Option<u64>,
    /// Retrieve data within a specific date range, by setting start dates for your query.
    /// Set the start date with: start_date=yyyy-mm-dd
    start_date: Option<NaiveDate>,
    /// Retrieve data within a specific date range, by setting end dates for your query.
    /// Set the end date with: end_date=yyyy-mm-dd
    end_date: Option<NaiveDate>,
    /// Sort in ascending or descending order.
    order: Option<Order>,
    ///Parameters to indicate the desired frequency.
    collapse: Option<Collapse>,
    ///Perform calculations on your data prior to downloading.
    transform: Option<Transform>,
}

/// Sort in ascending or descending order.
#[derive(Debug, PartialEq)]
pub enum Order {
    /// ascending order
    Asc,
    /// descending order
    Desc,
}

/// Converts `Order` enum variants to what is expected as input parameters in the URL
/// to the Quandl API
impl Display for Order {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Order::Asc => "asc",
                   Order::Desc => "desc",
               })
    }
}

/// Parameters to indicate the desired frequency.
#[derive(Debug, PartialEq)]
pub enum Collapse {
    /// Daily
    Daily,
    /// Weekly
    Weekly,
    /// Monthly
    Monthly,
    /// Quarterly
    Quarterly,
    /// Annual
    Annual,
}

/// Converts `Collapse` enum variants to what is expected as input parameters for the URL
/// to the Quandl API.
impl Display for Collapse {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Collapse::Daily => "daily",
                   Collapse::Weekly => "weekly",
                   Collapse::Monthly => "monthly",
                   Collapse::Quarterly => "quarterly",
                   Collapse::Annual => "annual",
               })
    }
}

/// Perform calculations on your data prior to downloading.
#[derive(Debug, PartialEq)]
pub enum Transform {
    /// Row on row change. A parameter that will transform the data to show the difference
    /// between days.
    Diff,
    /// Percentage change. A parameter that will transform the data to show the difference
    /// between days divided by the previous day.
    Rdiff,
    /// Cummulative change. A parameter that will calculate the sum of all preceding data returned.
    Cumul,
    /// Normalize (set starting value at 100). A parameter that will normalize the data to the
    /// oldest datapoint returned.
    Normalize,
}


/// Converts `Transform` enum variants to what is expected as input parameters in the URL
/// to the Quandl API.
impl Display for Transform {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Transform::Diff => "diff",
                   Transform::Rdiff => "rdiff",
                   Transform::Cumul => "cumul",
                   Transform::Normalize => "normalize",
               })
    }
}

impl Default for QuandlRequest {
    fn default() -> QuandlRequest {
        QuandlRequest {
            api_key: None,
            database_code: String::from(""),
            dataset_code: String::from(""),
            limit: None,
            rows: None,
            column_index: None,
            start_date: None,
            end_date: None,
            order: None,
            collapse: None,
            transform: None,
        }
    }
}

impl QuandlRequest {
    /// Creates a new `QuandlRequest` using the specified database_code and dataset code.
    /// All other parameters as taken from the default implementation, setting the optional
    /// parameters to `None`.
    pub fn new(database_code: &str, dataset_code: &str) -> QuandlRequest {
        QuandlRequest {
            database_code: String::from(database_code),
            dataset_code: String::from(dataset_code),
            ..Default::default()
        }
    }

    /// Quandl API key. Used for premium databases and/or increased usage limits.
    pub fn api_key(mut self, key: String) -> QuandlRequest {
        self.api_key = Some(key);
        self
    }

    /// You can use `limit=n` to get only the first `n` rows of your dataset. Use `limit=1` to get
    /// the latest observation for any dataset.
    pub fn limit(mut self, limit: u64) -> QuandlRequest {
        self.limit = Some(limit);
        self
    }

    /// You can use `rows=n` to get only the first `n` rows of your dataset. Use `rows=1` to get
    /// the latest observation for any dataset.
    pub fn rows(mut self, rows: u64) -> QuandlRequest {
        self.rows = Some(rows);
        self
    }

    /// Request a specific column by passing the `column_index=n` parameter. Column 0 is the
    /// date column and is always returned. Data begins at column 1.
    pub fn column_index(mut self, indx: u64) -> QuandlRequest {
        self.column_index = Some(indx);
        self
    }


    /// Retrieve data within a specific date range, by setting start date for your query.
    /// Takes either a `&str` in the format of `yyyy-mm-dd` or a `chrono::NaiveDate` as input.
    pub fn start_date<T: ?Sized + DateInput>(mut self, date: &T) -> Result<QuandlRequest> {
        try!(date.set_start_date(&mut self));
        Ok(self)
    }

    /// Retrieve data within a specific date range, by setting end date for your query.
    /// Takes either a `&str` in the format of `yyyy-mm-dd` or a `chrono::NaiveDate` as input.
    pub fn end_date<T: ?Sized + DateInput>(mut self, date: &T) -> Result<QuandlRequest> {
        try!(date.set_end_date(&mut self));
        Ok(self)
    }

    /// Select the sort order. The default sort order is `Desc`.
    pub fn order(mut self, order: Order) -> QuandlRequest {
        self.order = Some(order);
        self
    }

    /// Parameters to indicate the desired frequency. When you change the frequency of a dataset,
    /// Quandl returns the last observation for the given period. By collapsing a daily dataset
    /// to monthly, you will get a sample of the original dataset where the observation for each
    /// month is the last data point available for that month. Set collapse with:
    /// `collapse=none|daily|weekly|monthly|quarterly|annual`.
    pub fn collapse(mut self, collapse: Collapse) -> QuandlRequest {
        self.collapse = Some(collapse);
        self
    }

    /// Perform calculations on your data prior to downloading. The transformations currently
    /// available are row-on-row change, percentage change, cumulative sum, and normalize
    /// (set starting value at 100). Set the transform parameter with:
    /// `transform=none|diff|rdiff|cumul|normalize`.
    pub fn transform(mut self, transform: Transform) -> QuandlRequest {
        self.transform = Some(transform);
        self
    }

    /// Build the URL to send to the Quandl API
    fn get_url(&self) -> Url {
        let mut url: Url = Url::parse(&format!("{}/{}/{}/data.json",
                                               QUANDL_BASE_URL,
                                               self.database_code,
                                               self.dataset_code))
                               .unwrap();
        let mut query: Vec<(&str, String)> = Vec::new();

        set_query_pair(&mut query, "api_key", &self.api_key);
        set_query_pair(&mut query, "limit", &self.limit);
        set_query_pair(&mut query, "rows", &self.rows);
        set_query_pair(&mut query, "column_index", &self.column_index);
        set_query_pair(&mut query, "start_date", &self.start_date);
        set_query_pair(&mut query, "end_date", &self.end_date);
        set_query_pair(&mut query, "order", &self.order);
        set_query_pair(&mut query, "collapse", &self.collapse);
        set_query_pair(&mut query, "transform", &self.transform);
        url.set_query_from_pairs(query);

        url
    }

    /// Make a request to the Quandl API with the specified parameters
    pub fn run(&self) -> Result<serde_json::Value> {
        let client = hyper::Client::new();
        let url = self.get_url();
        let res = try!(client.get(url).send());

        match res.status {
            hyper::Ok => {
                let data: serde_json::Value = try!(serde_json::from_reader(res));
                Ok(data)
            }
            // something happened, quandl rejected the request
            status => {
                let data: serde_json::Value = try!(serde_json::from_reader(res));
                Err(Error::Quandl(format!("quandl request failed with code `{}` and response: \
                                           {:?}",
                                          status,
                                          data)))
            }
        }
    }
}

/// Allow for multiple types to be used as input to the `start_date` and `end_date` `QuandlRequest`
/// parameters.
pub trait DateInput {
    /// Set the `start_date` for `QuandlRequest`.
    fn set_start_date(&self, quandl_request: &mut QuandlRequest) -> Result<()>;
    /// Set the `end_date` for `QuandlRequest`.
    fn set_end_date(&self, quandl_request: &mut QuandlRequest) -> Result<()>;
}

impl DateInput for str {
    fn set_start_date(&self, quandl_request: &mut QuandlRequest) -> Result<()> {
        let date = try!(self.parse::<NaiveDate>());
        date.set_start_date(quandl_request)
    }
    fn set_end_date(&self, quandl_request: &mut QuandlRequest) -> Result<()> {
        let date = try!(self.parse::<NaiveDate>());
        date.set_end_date(quandl_request)
    }
}

impl DateInput for NaiveDate {
    fn set_start_date(&self, quandl_request: &mut QuandlRequest) -> Result<()> {
        try!(validate_date(&Some(*self), &quandl_request.end_date));
        quandl_request.start_date = Some(*self);
        Ok(())
    }
    fn set_end_date(&self, quandl_request: &mut QuandlRequest) -> Result<()> {
        try!(validate_date(&quandl_request.start_date, &Some(*self)));
        quandl_request.end_date = Some(*self);
        Ok(())
    }
             }

/// Compare two `NaiveDate`s and fail when `start_date` > `end_date`
fn validate_date(start_date: &Option<NaiveDate>, end_date: &Option<NaiveDate>) -> Result<()> {
    if let (Some(start_date), Some(end_date)) = (*start_date, *end_date) {
        if start_date > end_date {
            return Err(Error::Date(format!("start date `{}` is after end date `{}`",
                                           start_date,
                                           end_date)));
        }
    }
    Ok(())
}

/// Set query parameters for the given option if it is `Some(T)`
pub fn set_query_pair<'a, T: Display>(query: &mut Vec<(&'a str, String)>,
                                      key: &'a str,
                                      option: &Option<T>) {
    if let Some(ref value) = *option {
        query.push((key, value.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use chrono::NaiveDate;

    fn new_quandl_request() -> QuandlRequest {
        QuandlRequest::new("WIKI", "AAPL")
    }

    #[test]
    fn test_new_quandl_request() {
        let q = new_quandl_request();
        assert_eq!(q.database_code, String::from("WIKI"));
        assert_eq!(q.dataset_code, String::from("AAPL"));
        assert_eq!(q.limit, None);
    }

    #[test]
    fn test_limit() {
        let q = new_quandl_request().limit(12);
        assert_eq!(q.limit, Some(12));
    }

    #[test]
    fn test_rows() {
        let q = new_quandl_request().rows(2);
        assert_eq!(q.rows, Some(2));
    }

    #[test]
    fn test_column_index() {
        let q = new_quandl_request().column_index(0);
        assert_eq!(q.column_index, Some(0));
    }

    #[test]
    fn test_order() {
        use super::Order::*;
        let q = new_quandl_request().order(Asc);
        assert_eq!(q.order, Some(Asc));
    }

    #[test]
    fn test_collapse() {
        use super::Collapse::*;
        let q = new_quandl_request().collapse(Daily);
        assert_eq!(q.collapse, Some(Daily));
    }

    #[test]
    fn test_transform() {
        use super::Transform::*;
        let q = new_quandl_request().transform(Rdiff);
        assert_eq!(q.transform, Some(Rdiff));
    }

    #[test]
    fn test_url_query() {
        let u_str = "https://www.quandl.com/api/v3/datasets/WIKI/AAPL/data.\
                     json?limit=10&rows=1&column_index=25&start_date=2015-02-10&end_date=2015-03-1\
                     0&order=asc&collapse=daily&transform=normalize";
        let url = Url::parse(u_str).unwrap();

        let q = new_quandl_request()
                    .order(Order::Asc)
                    .limit(10u64)
                    .collapse(Collapse::Daily)
                    .rows(1u64)
                    .column_index(25u64)
                    .transform(Transform::Normalize)
                    .start_date("2015-02-10")
                    .unwrap()
                    .end_date("2015-03-10")
                    .unwrap();
        assert_eq!(url, q.get_url());
    }

    #[test]
    fn test_validate_date_err() {
        // from str
        let q = new_quandl_request()
                    .start_date("2015-02-10")
                    .unwrap()
                    .end_date("2015-01-10");

        println!("{:?}", q);
        assert_eq!(&q.is_err(), &true);

        // from NaiveDate
        let q = new_quandl_request()
                    .start_date(&NaiveDate::from_ymd(2015, 02, 10))
                    .unwrap()
                    .end_date(&NaiveDate::from_ymd(2015, 01, 10));

        println!("{:?}", q);
        assert_eq!(&q.is_err(), &true);

        // from NaiveDate + str
        let q = new_quandl_request()
                    .start_date(&NaiveDate::from_ymd(2015, 02, 10))
                    .unwrap()
                    .end_date("2015-01-10");

        println!("{:?}", q);
        assert_eq!(&q.is_err(), &true);

        // from NaiveDate + str
        let q = new_quandl_request()
                    .end_date(&NaiveDate::from_ymd(2015, 02, 10))
                    .unwrap()
                    .start_date("2015-03-10");

        println!("{:?}", q);
        assert_eq!(&q.is_err(), &true);

        // correct params should not have an error
        let q = new_quandl_request()
                    .start_date(&NaiveDate::from_ymd(2015, 02, 10))
                    .unwrap()
                    .end_date("2015-03-10");

        println!("{:?}", q);
        assert_eq!(&q.is_ok(), &true);
    }

    #[cfg(feature = "test-quandl-api")]
    #[test]
    fn test_quandl_not_found_error() {
        use error::Error;
        let res = QuandlRequest::new("WIKI", "AAAPL").rows(1u64).run();

        assert_eq!(&res.is_err(), &true);
        match res.unwrap_err() {
            Error::Quandl(s) => println!("{}", s),
            e => panic!("unexpected error type: {:?}", e),
        }
    }

    #[cfg(feature = "test-quandl-api")]
    #[test]
    fn test_quandl_works() {
        let res = QuandlRequest::new("WIKI", "AMZN").rows(1u64).run();
        if res.is_err() {
            panic!("quandl req failed: {:?}", res)
        }
    }
}
