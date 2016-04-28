#[macro_use]
#[cfg(test)]
pub mod test_helpers {
    use hyper_mock::MockStream;
    use hyper;
    use log;
    // use std::io::{self, Read, Write, Cursor};
    use std::io::Cursor;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct HostToReplyConnectorBytes {
        pub m: HashMap<String, Vec<u8>>,
    }

    impl hyper::net::NetworkConnector for HostToReplyConnectorBytes {
        type Stream = MockStream;

        fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<MockStream> {
            debug!("HostToReplyConnector::connect({:?}, {:?}, {:?})",
                   host,
                   port,
                   scheme);
            let key = format!("{}://{}", scheme, host);
            // ignore port for now
            match self.m.get(&key) {
                Some(res) => {
                    Ok(MockStream {
                        write: vec![],
                        read: Cursor::new(res.clone()),
                    })
                }
                None => panic!("HostToReplyConnectorBytes doesn't know url {}", key),
            }
        }
    }

    macro_rules! mock_quandl_responder {
        ($name:ident, $data:expr) => {
            mock_connector!($name {
                "https://www.quandl.com" => "HTTP/1.1 200 OK\r\n\r\n".to_owned() + $data
            });
        }
    }

    macro_rules! mock_quandl_responder_bytes {
        ($name:ident, $bytes:expr) => {
            mock_connector_bytes!($name {
                "https://www.quandl.com" => b"HTTP/1.1 200 OK\r\n\r\n".iter().chain($bytes().iter()).cloned().collect::<Vec<u8>>()
            });
        }
    }

    macro_rules! mock_connector_bytes {
        ($name:ident {
            $($url:expr => $res:expr)*
        }) => (
            use hyper_mock;

            struct $name(HostToReplyConnectorBytes);

            impl Default for $name {
                fn default() -> $name {
                    let mut c = $name(Default::default());
                    $(c.0.m.insert($url.to_string(), $res);)*
                    c
                }
            }

            impl hyper::net::NetworkConnector for $name {
                type Stream = hyper_mock::MockStream;

                fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<hyper_mock::MockStream> {
                    self.0.connect(host, port, scheme)
                }
            }
        )
    }

}
