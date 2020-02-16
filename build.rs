#[cfg(all(feature = "skeptic", feature = "test-quandl-api"))]
extern crate skeptic;

#[cfg(all(feature = "skeptic", feature = "test-quandl-api"))]
fn main() {
    skeptic::generate_doc_tests(&["README.md"]);
}

// do nothing unless skeptic feature is present
#[cfg(not(all(feature = "skeptic", feature = "test-quandl-api")))]
fn main() {}
