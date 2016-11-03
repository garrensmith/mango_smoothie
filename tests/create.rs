extern crate rustybench;

use rustybench::{database};

#[test]
fn creates_index () {
    let resp = database("http://tester:testerpass@127.0.0.1:5984/animaldb")
               .create_index(&["class", "diet"]);

    assert!(resp.is_ok());
}
