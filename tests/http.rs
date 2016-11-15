extern crate rustybench;

use rustybench::http::{get};

#[test]
fn get_returns_response () {
    let res = get("http://tester:testerpass@127.0.0.1:5984/");
    assert!(res.is_ok());
}

#[test]
fn get_returns_error_response () {
    let res = get("http://tester:testerpass@127.0.0.1:5984/notexistdatabase");
    assert!(res.is_err());
}
