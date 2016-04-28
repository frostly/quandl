// `impl DatasetListCall` will have at least the following fns: `get_url`, `run`
// `get_url` builds the url according to this schema:
// `GET https://www.quandl.com/api/v3/databases/:database_code/codes.json` - for now, don't add api_key to url

use Quandl;
use url::Url;
use error::Result;
use std::io::prelude::*;
use zip::read::ZipArchive;
use error::Error;
use quick_csv::Csv;
use std::io::BufReader;
use HttpClient;

const QUANDL_DATABASE_URL: &'static str = "https://www.quandl.com/api/v3/databases";
const SEPARATOR: char = '/';

/// Request to find all dataset codes in a database.
#[derive(Debug)]
pub struct DatasetListCall<'a> {
    /// The unique database code on Quandl (ex. WIKI)
    pub database_code: String,
    /// Reference to Quandl
    pub quandl: &'a Quandl,
}

/// Build URL to get dataset_list from Quandl.
impl<'a> DatasetListCall<'a> {
    fn get_url(&self) -> Result<Url> {
        // TODO: ensure API key is set
        Ok(try!(Url::parse(&format!("{}/{}/codes.csv", QUANDL_DATABASE_URL, self.database_code))))
    }

    /// Make a request to Quandl API to get database.
    pub fn run(&self) -> Result<Vec<DatasetCode>> {
        let mut res = try!(self.quandl.http_client().get(try!(self.get_url())).send());
        // unzip res
        let mut bytes = vec![];
        try!(res.read_to_end(&mut bytes));
        let byte_cursor = ::std::io::Cursor::new(bytes);
        let mut zip = try!(ZipArchive::new(byte_cursor));
        println!("valid directory archive");
        if zip.len() != 1 {
            return Err(Error::Quandl(format!("Expected one file in zip archive, found: {}",
                                             zip.len())));
        }
        let unzipped_file = try!(zip.by_index(0));
        // // convert unzipped res (Zipfile<'a>) to csv.
        let csv = Csv::from_reader(BufReader::new(unzipped_file));

        csv.into_iter()
           .map(|row| {
               let row = try!(row);
               // db_with_code = YC/MYS5Y
               let (db_with_code, desc) = try!(row.decode::<(String, String)>());
               println!("{:?}", db_with_code);
               match db_with_code.find(SEPARATOR) {
                   Some(_) => {
                       Ok(DatasetCode {
                           code: String::from(db_with_code.split(SEPARATOR).last().unwrap()),
                           desc: String::from(desc),
                       })
                   }
                   None => {
                       Err(Error::Quandl(format!("error: `/` not found in `{}`", db_with_code)))
                   }
               }
           })
           .collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct DatasetCode {
    pub code: String,
    pub desc: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use zip::ZipArchive;
    use zip::CompressionMethod;
    use std::fs::File;
    use std::io::Read;
    use quick_csv::{Csv, Row};
    use std::io::BufReader;
    use quandl::Quandl;
    use hyper;
    use macros::test_helpers::HostToReplyConnectorBytes;
    use std::io::Write;
    use std::io::Cursor;
    use zip::write::ZipWriter;

    #[test]
    fn test_make_read_zip() {
        let buf: Vec<u8> = Vec::new();
        let mut w = Cursor::new(buf);
        let mut zip = ZipWriter::new(w);
        zip.start_file("YC-dataset-codes.csv", CompressionMethod::Deflated).unwrap();
        let csv = b"YC/MYS5Y,Malaysian Government 5-Year Bond Yield
        YC/CHN7Y,Chinese Government 7-Year Bond Yield
        YC/SGP3M,Singapore Government 3-Month Money Market Rate
        YC/ZAF9M,South African Government 9-Month Money Market Rate";
        zip.write(csv).unwrap();
        let orig_buf = zip.finish().unwrap().into_inner();
        println!("{:?}", orig_buf);
        let mut reader = ::std::io::Cursor::new(orig_buf);
        let mut zip = ZipArchive::new(reader).unwrap();
        let mut s = String::new();
        let mut file = zip.by_index(0).unwrap();
        file.read_to_string(&mut s);
        println!("contents = {}", s);
    }

    #[test]
    fn test_dataset_list_call() {
        // convert generated zip file to u8 so you can pass it into
        // mock_quandl_responder_bytes.

        // pass in bytes into macro.
        mock_quandl_responder_bytes!(MockZipConnector, || {
            let buf: Vec<u8> = Vec::new();
            // use cursor, provides Seek trait
            // takes buf (buffer is a type of reader)
            // returns Cursor<Vec::new()>.
            let mut w = Cursor::new(buf);
            // initialize ZipWriter
            // pass in Cursor<Vec::new()>
            // returns ZipWriter<Cursor<Vec::new()>>
            let mut zip = ZipWriter::new(w);
            // Must start_file before using write.
            // takes name of file and CompressionMethod.
            // returns ZipResult<T, ZipError>
            zip.start_file("YC-dataset-codes.csv", CompressionMethod::Deflated).unwrap();
            // write file: takes in buf u8
            // returns Result<usize>
            let csv = b"YC/MYS5Y,Malaysian Government 5-Year Bond Yield
YC/CHN7Y,Chinese Government 7-Year Bond Yield
YC/SGP3M,Singapore Government 3-Month Money Market Rate
YC/ZAF9M,South African Government 9-Month Money Market Rate";
            zip.write(csv).unwrap();
            zip.finish().unwrap().into_inner()
        });
        let mut q = Quandl::new().set_http_client_with_connector(MockZipConnector::default());
        let dl = q.new_dataset_list_call("YC");
        let res = dl.run().unwrap();
        println!("res={:?}", res);
        assert_eq!(res[0],
                   DatasetCode {
                       code: String::from("MYS5Y"),
                       desc: String::from("Malaysian Government 5-Year Bond Yield"),
                   });
    }

    #[test]
    fn test_zip_list_codes() {
        let file = File::open("./tests/data/YC-datasets-codes.zip").unwrap();
        let mut zip = ZipArchive::new(file).unwrap();
        let mut unzip_file = zip.by_index(0).unwrap();
        let mut file_content = String::new();
        unzip_file.read_to_string(&mut file_content);
        let csv = Csv::from_string(&file_content);

        let codes = csv.into_iter()
                       .map(|row| {
                           let row = row.unwrap();
                           let (a, _) = row.decode::<(String, String)>().unwrap();
                           // impl String
                           // fn split<'a, P>(&'a self, pat: P) -> Split<'a, P>
                           a.split('/').map(String::from).nth(1).unwrap()
                       })
                       .collect::<Vec<String>>();
        println!("{:?}", codes);
    }
}
