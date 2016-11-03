use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::{ContentType};
use std::io::Read;

use errors::{Error};

pub fn post (url: &str, body: &str) -> Result<String, Error> {
    let client = Client::new();
    let mut res = try!(client
                    .post(url)
                    .header(ContentType::json())
                    .body(body)
                    .send());

    let mut resp = String::new();
    res.read_to_string(&mut resp).unwrap();
    println!("POST QUERY {} {}",url, resp);
    println!("{}", res.status);

    if res.status == StatusCode::Ok {
        Ok(resp)
    } else {
        Err(Error::from_couch(&resp))
    }
}

pub fn get (url :&str) -> Result<String, Error> {
    let client = Client::new();
    let mut res = try!(client
                    .get(url)
                    .header(ContentType::json())
                    .send());

    let mut resp = String::new();
    try!(res.read_to_string(&mut resp));

    println!("GET {}", resp);
    println!("{}", res.status);
    if res.status == StatusCode::Ok {
        Ok(resp)
    } else {
        Err(Error::from_couch(&resp))
    }
}
