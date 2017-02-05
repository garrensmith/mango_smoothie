use hyper::{Client, Url};
use hyper::status::StatusCode;
use hyper::header::{ContentType, Headers, Authorization, Basic};
use std::io::Read;
use errors::Error;

fn get_headers(url: Url) -> Headers {
    let mut headers = Headers::new();

    headers.set(ContentType::json());
    headers.set(Authorization(Basic {
        username: url.username().to_string(),
        password: match url.password() {
            None => None,
            Some(password) => Some(password.to_string()),
        },
    }));

    headers
}

#[doc(hidden)]
pub fn post(url: &Url, body: &str) -> Result<String, Error> {
    let headers = get_headers(url.clone());
    let client = Client::new();
    let mut res = try!(client.post(url.clone())
        .headers(headers)
        .body(body)
        .send());

    let mut resp = String::new();
    res.read_to_string(&mut resp).unwrap();
    println!("POST QUERY {} {}", url.to_string(), resp);
    println!("{}", body);

    match res.status {
        StatusCode::Created | StatusCode::Ok => Ok(resp),
        _ => Err(Error::from_couch(&resp)),
    }
}

#[doc(hidden)]
pub fn get(url: &Url) -> Result<String, Error> {
    let headers = get_headers(url.clone());
    let client = Client::new();
    let mut res = try!(client.get(url.clone())
        .headers(headers)
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
