# Quandl
[![Travis Build Status](https://img.shields.io/travis/frostly/quandl.svg)](https://travis-ci.org/frostly/quandl)
[![Documentation](https://img.shields.io/badge/docs-latest-C9893D.svg)](https://open.frostly.com/quandl)
[![Coverage Status](https://img.shields.io/coveralls/frostly/quandl.svg)](https://coveralls.io/github/frostly/quandl?branch=master)
[![crates.io](https://img.shields.io/crates/v/quandl.svg)](https://crates.io/crates/quandl)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE-APACHE)

# Overview

Uses the [Quandl v3 API](https://www.quandl.com/docs/api) to retrieve financial and economic
datasets.

# Usage

```rust
extern crate quandl;
extern crate chrono;

use quandl::Quandl;
use chrono::NaiveDate;

fn main() {
  // basic request
  let q = Quandl::new();
  let _ = q.new_request("WIKI", "AAAPL")
    .rows(5)
    .run();

  // specify some dates
  // Note: setting the start and end dates could fail (parsing error or inconsistency)
  // so when set, a Result is returned
  let _ = q.new_request("WIKI", "AAPL")
    .start_date("2015-11-10")
    .unwrap()
    .end_date("2015-11-12")
    .unwrap()
    .run();

  // pass a NaiveDate instead of a &str
  let d = NaiveDate::from_ymd(2015, 11, 20);
  let _ = q.new_request("WIKI", "AAPL")
    .start_date(&d)
    .unwrap()
    .run();
}
```

# Testing

Some notes about the different testing options:

- `cargo test` will run all tests that don't call the Quandl API.
API.
- `cargo test --features test-quandl-api` will include tests that call the Quandl API.
- `cargo test --features "skeptic test-quandl-api"` will run all tests including the tests in this
README file.

[clippy](https://github.com/Manishearth/rust-clippy) is also run as part of the nightly build on travis.

# License

This library is distributed under similar terms to Rust: dual licensed under the MIT license and the Apache license (version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.
