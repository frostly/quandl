# Quandl

[![Travis Build Status](https://travis-ci.org/frostly/quandl.svg?branch=master)](https://travis-ci.org/frostly/quandl)
[![Coverage Status](https://coveralls.io/repos/frostly/quandl/badge.svg?branch=master)](https://coveralls.io/r/frostly/quandl?branch=master)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE-APACHE)
[![crates.io](http://meritbadge.herokuapp.com/quandl)](https://crates.io/crates/quandl)

[Documentation](https://open.frostly.com/quandl)

# Overview

Uses the [Quandl v3 API](https://www.quandl.com/docs/api) to retrieve financial and economic
datasets.

# Usage

```rust
extern crate quandl;
extern crate chrono;

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
