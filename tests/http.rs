extern crate mango_smoothie;
extern crate hyper;

use mango_smoothie::http::get;
use hyper::Url;

#[test]
fn get_returns_response() {
    let res = get(&Url::parse("http://tester:testerpass@127.0.0.1:5984/").unwrap());
    assert!(res.is_ok());
}

#[test]
fn get_returns_error_response() {
    let res = get(&Url::parse("http://tester:testerpass@127.0.0.1:5984/notexistdatabase").unwrap());
    assert!(res.is_err());
}
