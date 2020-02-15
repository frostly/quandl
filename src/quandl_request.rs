use super::{JsonValue, NaiveDate, Quandl};
use crate::error::{Error, Result};
use bytes::buf::BufExt as _;
use std::fmt::{self, Debug, Display, Formatter};

/// use v3 of Quandl API
const QUANDL_BASE_URL: &str = "https://www.quandl.com/api/v3/datasets";

/// Parameters for the request to Quandl API
pub struct QuandlRequest<'a> {
    /// Reference to Quandl struct. This information will be available for all
    /// Quandl requests.
    pub quandl: &'a Quandl,
    /// The unique database code on Quandl (ex. WIKI)
    pub database_code: String,
    /// The unique dataset code on Quandl (ex. APPL)
    pub dataset_code: String,
    /// You can use `limit=n` to get only the first `n` rows of your dataset. Use `limit=1` to get
    /// the latest observation for any dataset.
    pub limit: Option<u64>,
    /// You can use `rows=n` to get only the first `n` rows of your dataset. Use `rows=1` to get
    /// the latest observation for any dataset.
    pub rows: Option<u64>,
    /// Request specific column.
    pub column_index: Option<u64>,
    /// Retrieve data within a specific date range, by setting start dates for your query.
    /// Set the start date with: start_date=yyyy-mm-dd
    pub start_date: Option<NaiveDate>,
    /// Retrieve data within a specific date range, by setting end dates for your query.
    /// Set the end date with: end_date=yyyy-mm-dd
    pub end_date: Option<NaiveDate>,
    /// Sort in ascending or descending order.
    pub order: Option<Order>,
    /// Parameters to indicate the desired frequency.
    pub collapse: Option<Collapse>,
    /// Perform calculations on your data prior to downloading.
    pub transform: Option<Transform>,
}

impl<'a> Debug for QuandlRequest<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("QuandlRequest")
            .field("quandl", &self.quandl)
            .field("database_code", &self.database_code)
            .field("dataset_code", &self.dataset_code)
            .field("limit", &self.limit)
            .field("rows", &self.rows)
            .field("column_index", &self.column_index)
            .field("start_date", &self.start_date)
            .field("end_date", &self.end_date)
            .field("order", &self.order)
            .field("collapse", &self.collapse)
            .field("transform", &self.transform)
            .finish()
    }
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Order::Asc => "asc",
                Order::Desc => "desc",
            }
        )
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
        write!(
            f,
            "{}",
            match *self {
                Collapse::Daily => "daily",
                Collapse::Weekly => "weekly",
                Collapse::Monthly => "monthly",
                Collapse::Quarterly => "quarterly",
                Collapse::Annual => "annual",
            }
        )
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
        write!(
            f,
            "{}",
            match *self {
                Transform::Diff => "diff",
                Transform::Rdiff => "rdiff",
                Transform::Cumul => "cumul",
                Transform::Normalize => "normalize",
            }
        )
    }
}

impl<'a> QuandlRequest<'a> {
    /// Creates a new `QuandlRequest` using the specified database_code and dataset code.
    /// All other parameters as taken from the default implementation, setting the optional
    /// parameters to `None`.
    /// You can use `limit=n` to get only the first `n` rows of your dataset. Use `limit=1` to get
    /// the latest observation for any dataset.
    pub fn limit(mut self, limit: u64) -> QuandlRequest<'a> {
        self.limit = Some(limit);
        self
    }

    /// You can use `rows=n` to get only the first `n` rows of your dataset. Use `rows=1` to get
    /// the latest observation for any dataset.
    pub fn rows(mut self, rows: u64) -> QuandlRequest<'a> {
        self.rows = Some(rows);
        self
    }

    /// Request a specific column by passing the `column_index=n` parameter. Column 0 is the
    /// date column and is always returned. Data begins at column 1.
    pub fn column_index(mut self, indx: u64) -> QuandlRequest<'a> {
        self.column_index = Some(indx);
        self
    }

    /// Retrieve data within a specific date range, by setting start date for your query.
    /// Takes either a `&str` in the format of `yyyy-mm-dd` or a `chrono::NaiveDate` as input.
    pub fn start_date<T: ?Sized + DateInput>(mut self, date: &T) -> Result<QuandlRequest<'a>> {
        date.set_start_date(&mut self)?;
        Ok(self)
    }

    /// Retrieve data within a specific date range, by setting end date for your query.
    /// Takes either a `&str` in the format of `yyyy-mm-dd` or a `chrono::NaiveDate` as input.
    pub fn end_date<T: ?Sized + DateInput>(mut self, date: &T) -> Result<QuandlRequest<'a>> {
        date.set_end_date(&mut self)?;
        Ok(self)
    }

    /// Select the sort order. The default sort order is `Desc`.
    pub fn order(mut self, order: Order) -> QuandlRequest<'a> {
        self.order = Some(order);
        self
    }

    /// Parameters to indicate the desired frequency. When you change the frequency of a dataset,
    /// Quandl returns the last observation for the given period. By collapsing a daily dataset
    /// to monthly, you will get a sample of the original dataset where the observation for each
    /// month is the last data point available for that month. Set collapse with:
    /// `collapse=none|daily|weekly|monthly|quarterly|annual`.
    pub fn collapse(mut self, collapse: Collapse) -> QuandlRequest<'a> {
        self.collapse = Some(collapse);
        self
    }

    /// Perform calculations on your data prior to downloading. The transformations currently
    /// available are row-on-row change, percentage change, cumulative sum, and normalize
    /// (set starting value at 100). Set the transform parameter with:
    /// `transform=none|diff|rdiff|cumul|normalize`.
    pub fn transform(mut self, transform: Transform) -> QuandlRequest<'a> {
        self.transform = Some(transform);
        self
    }

    /// Build the URI to send to the Quandl API
    fn get_uri(&self) -> hyper::Uri {
        let uri = &format!(
            "{}/{}/{}/data.json",
            QUANDL_BASE_URL, self.database_code, self.dataset_code
        );
        let mut query: Vec<(&str, String)> = Vec::new();

        set_query_pair(&mut query, "api_key", &self.quandl.api_key);
        set_query_pair(&mut query, "limit", &self.limit);
        set_query_pair(&mut query, "rows", &self.rows);
        set_query_pair(&mut query, "column_index", &self.column_index);
        set_query_pair(&mut query, "start_date", &self.start_date);
        set_query_pair(&mut query, "end_date", &self.end_date);
        set_query_pair(&mut query, "order", &self.order);
        set_query_pair(&mut query, "collapse", &self.collapse);
        set_query_pair(&mut query, "transform", &self.transform);

        let uri = if query.is_empty() {
            uri.to_string()
        } else {
            let params: Vec<String> = query
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect();
            format!("{}?{}", uri, &params.join("&"))
        };

        let uri: hyper::Uri = uri.parse::<hyper::Uri>().unwrap();
        uri
    }

    /// Make a request to the Quandl API with the specified parameters
    #[tokio::main]
    pub async fn run(&self) -> Result<JsonValue> {
        let uri = self.get_uri();
        let res = self.quandl.http_client.get(uri).await?;

        match res.status() {
            hyper::StatusCode::OK => {
                let body = hyper::body::aggregate(res).await?;

                let data: JsonValue = serde_json::from_reader(body.reader())?;
                Ok(data)
            }
            // something happened, quandl rejected the request
            status => {
                let body = hyper::body::aggregate(res).await?;

                let data: JsonValue = serde_json::from_reader(body.reader())?;
                Err(Error::Quandl(format!(
                    "quandl request failed with code `{}` and response: \
                                           {:?}",
                    status, data
                )))
            }
        }
    }

    /// Create a default QuandlRequest
    pub fn default(quandl: &'a Quandl) -> QuandlRequest<'a> {
        QuandlRequest {
            quandl,
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
        let date = self.parse::<NaiveDate>()?;
        date.set_start_date(quandl_request)
    }
    fn set_end_date(&self, quandl_request: &mut QuandlRequest) -> Result<()> {
        let date = self.parse::<NaiveDate>()?;
        date.set_end_date(quandl_request)
    }
}

impl DateInput for NaiveDate {
    fn set_start_date(&self, quandl_request: &mut QuandlRequest) -> Result<()> {
        validate_date(Some(*self), quandl_request.end_date)?;
        quandl_request.start_date = Some(*self);
        Ok(())
    }
    fn set_end_date(&self, quandl_request: &mut QuandlRequest) -> Result<()> {
        validate_date(quandl_request.start_date, Some(*self))?;
        quandl_request.end_date = Some(*self);
        Ok(())
    }
}

/// Compare two `NaiveDate`s and fail when `start_date` > `end_date`
fn validate_date(start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) -> Result<()> {
    if let (Some(start_date), Some(end_date)) = (start_date, end_date) {
        if start_date > end_date {
            return Err(Error::Date(format!(
                "start date `{}` is after end date `{}`",
                start_date, end_date
            )));
        }
    }
    Ok(())
}

/// Set query parameters for the given option if it is `Some(T)`
fn set_query_pair<'a, T: Display>(
    query: &mut Vec<(&'a str, String)>,
    key: &'a str,
    option: &Option<T>,
) {
    if let Some(ref value) = *option {
        query.push((key, value.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::super::{NaiveDate, Quandl};
    use super::*;
    use hyper;

    fn new_quandl_request(quandl: &Quandl) -> QuandlRequest {
        quandl.new_request("WIKI", "AAPL")
    }

    #[test]
    fn test_new_quandl_request() {
        let q = Quandl::new();
        let qr = new_quandl_request(&q);
        assert_eq!(qr.database_code, String::from("WIKI"));
        assert_eq!(qr.dataset_code, String::from("AAPL"));
        assert_eq!(qr.limit, None);
    }

    #[test]
    fn test_limit() {
        let q = Quandl::new();
        let qr = new_quandl_request(&q).limit(12);
        assert_eq!(qr.limit, Some(12));
    }

    #[test]
    fn test_rows() {
        let q = Quandl::new();
        let qr = new_quandl_request(&q).rows(2);
        assert_eq!(qr.rows, Some(2));
    }

    #[test]
    fn test_column_index() {
        let q = Quandl::new();
        let qr = new_quandl_request(&q).column_index(0);
        assert_eq!(qr.column_index, Some(0));
    }

    #[test]
    fn test_order() {
        use super::Order::*;
        let q = Quandl::new();
        let qr = new_quandl_request(&q).order(Asc);
        assert_eq!(qr.order, Some(Asc));
    }

    #[test]
    fn test_collapse() {
        use super::Collapse::*;
        let q = Quandl::new();
        let qr = new_quandl_request(&q).collapse(Daily);
        assert_eq!(qr.collapse, Some(Daily));
    }

    #[test]
    fn test_transform() {
        use super::Transform::*;
        let q = Quandl::new();
        let qr = new_quandl_request(&q).transform(Rdiff);
        assert_eq!(qr.transform, Some(Rdiff));
    }

    #[test]
    fn test_url_query() {
        let u_str = "https://www.quandl.com/api/v3/datasets/WIKI/AAPL/data.\
                     json?limit=10&rows=1&column_index=25&start_date=2015-02-10&end_date=2015-03-1\
                     0&order=asc&collapse=daily&transform=normalize";
        let uri = u_str.parse::<hyper::Uri>().unwrap();

        let q = Quandl::new();
        let qr = new_quandl_request(&q)
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
        assert_eq!(qr.get_uri(), uri);
    }

    #[test]
    fn test_validate_date_err() {
        // from str
        let q = Quandl::new();
        let qr = new_quandl_request(&q)
            .start_date("2015-02-10")
            .unwrap()
            .end_date("2015-01-10");

        println!("{:?}", qr);
        assert_eq!(&qr.is_err(), &true);

        // from NaiveDate
        let q = Quandl::new();
        let qr = new_quandl_request(&q)
            .start_date(&NaiveDate::from_ymd(2015, 2, 10))
            .unwrap()
            .end_date(&NaiveDate::from_ymd(2015, 1, 10));

        println!("{:?}", qr);
        assert_eq!(&qr.is_err(), &true);

        // from NaiveDate + str
        let q = Quandl::new();
        let qr = new_quandl_request(&q)
            .start_date(&NaiveDate::from_ymd(2015, 2, 10))
            .unwrap()
            .end_date("2015-01-10");

        println!("{:?}", qr);
        assert_eq!(&qr.is_err(), &true);

        // from NaiveDate + str
        let q = Quandl::new();
        let qr = new_quandl_request(&q)
            .end_date(&NaiveDate::from_ymd(2015, 2, 10))
            .unwrap()
            .start_date("2015-03-10");

        println!("{:?}", qr);
        assert_eq!(&qr.is_err(), &true);

        // correct params should not have an error
        let q = Quandl::new();
        let qr = new_quandl_request(&q)
            .start_date(&NaiveDate::from_ymd(2015, 2, 10))
            .unwrap()
            .end_date("2015-03-10");

        println!("{:?}", qr);
        assert_eq!(&qr.is_ok(), &true);
    }

    #[cfg(feature = "test-quandl-api")]
    #[test]
    fn test_quandl_not_found_error() {
        use crate::error::Error;
        let q = Quandl::new();
        let res = q.new_request("WIKI", "AAAPL").rows(1u64).run();

        assert_eq!(&res.is_err(), &true);
        match res.unwrap_err() {
            Error::Quandl(s) => println!("{}", s),
            e => panic!("unexpected error type: {:?}", e),
        }
    }

    #[cfg(feature = "test-quandl-api")]
    #[test]
    fn test_quandl_works() {
        let q = Quandl::new();
        let res = new_quandl_request(&q).rows(1u64).run();
        if res.is_err() {
            panic!("quandl req failed: {:?}", res)
        }
    }
}
