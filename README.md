# Overview

Uses the [Quandl v3 API](https://www.quandl.com/docs/api) to retrieve financial and economic
datasets.

# Usage

```rust
extern crate quandl;
extern crate chrono;

use quandl::quandl_request::QuandlRequest;
use quandl::QuandlRequest;
use chrono::NaiveDate;

fn main() {
  // basic request
  let _ = QuandlRequest::new("WIKI", "AAPL")
    .rows(5)
    .run();

  // specify some dates
  // Note: setting the start and end dates could fail (parsing error or inconsistency)
  // so when set, a Result is returned
  let _ = QuandlRequest::new("WIKI", "AAPL")
    .start_date("2015-11-10")
    .unwrap()
    .end_date("2015-11-12")
    .unwrap()
    .run();

  // pass a NaiveDate instead of a &str
  let d = NaiveDate::from_ymd(2015, 11, 20);
  let _ = QuandlRequest::new("WIKI", "AAPL")
    .start_date(&d)
    .unwrap()
    .run();
}
```

# License

This library is distributed under similar terms to Rust: dual licensed under the MIT license and the Apache license (version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.
