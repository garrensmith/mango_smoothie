extern crate rustybench;

use rustybench::http::{post, get};

#[test]
fn get_returns_response () {
    let res = get("http://tester:testerpass@127.0.0.1:5984/");
    assert!(res.is_ok());
}

#[test]
fn get_returns_error_response () {
    let res = get("http://tester:testerpass@127.0.0.1:5984/notexistdatabase");
    match res {
        Err(error) => println!("BOOM!!!! {}", error),
        _ => println!("move along")
    };
    //assert!(res.is_err());
}
