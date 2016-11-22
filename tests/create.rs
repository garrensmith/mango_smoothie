extern crate mango_smoothie;
extern crate hyper;

use mango_smoothie::{database};

#[test]
fn creates_index () {
    let resp = database("http://tester:testerpass@127.0.0.1:5984/animaldb").unwrap()
               .create_index(&["class", "name"]);

    assert!(resp.is_ok());
}
